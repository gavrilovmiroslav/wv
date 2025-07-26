use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Write};
use crate::infer::{InferenceChoice, InferenceEngine, InferenceEnvironment, InferencePruningStep, InferenceUnificationContext, Query, SyntaxQueryBridge};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrologSyntax {
    True,
    False,
    And(Box<PrologSyntax>, Box<PrologSyntax>),
    Or(Box<PrologSyntax>, Box<PrologSyntax>),
    Atom(String),
    Var(String),
    Def(String, Vec<PrologSyntax>, Vec<PrologSyntax>),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PrologQuery {
    True,
    False,
    And(Box<PrologQuery>, Box<PrologQuery>),
    Or(Box<PrologQuery>, Box<PrologQuery>),
    Atom(String),
    Var(String),
    Fun(String, Vec<PrologQuery>),
    Unify(Box<PrologQuery>, Box<PrologQuery>),
}

impl Debug for PrologQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrologQuery::True => f.write_str("True"),
            PrologQuery::False => f.write_str("False"),
            PrologQuery::Atom(a) => f.write_str(a),
            PrologQuery::Var(v) => f.write_fmt(format_args!("${v}")),
            PrologQuery::And(v1, v2) =>
                f.write_fmt(format_args!("{:?}, {:?}", v1, v2)),
            PrologQuery::Or(v1, v2) =>
                f.write_fmt(format_args!("{:?}; {:?}", v1, v2)),
            PrologQuery::Fun(name, args) =>
                f.write_fmt(format_args!("{name}({})",
                    args.iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", "))),
            PrologQuery::Unify(a, b) => {
                f.write_fmt(format_args!("{:?} = {:?}", a, b))
            }
        }
    }
}

pub fn t() -> PrologQuery { PrologQuery::True }
pub fn f() -> PrologQuery { PrologQuery::False }
pub fn a(s: &str) -> PrologQuery { PrologQuery::Atom(s.into()) }
pub fn v(s: &str) -> PrologQuery { PrologQuery::Var(s.into()) }
pub fn p(s: &str, args: &[PrologQuery]) -> PrologQuery { PrologQuery::Fun(s.into(), args.into()) }
pub fn u(l: PrologQuery, r: PrologQuery) -> PrologQuery { PrologQuery::Unify(Box::new(l), Box::new(r)) }

impl Query<PrologSyntax> for PrologQuery {
    fn vars(&self) -> HashSet<PrologSyntax> {
        fn goals_rec(q: &PrologQuery, set: &mut HashSet<PrologSyntax>) {
            match q {
                PrologQuery::Var(v) => { set.insert(PrologSyntax::Var(v.clone())); },
                PrologQuery::Fun(_, args) => {
                    args.iter().flat_map(|q| q.vars()).for_each(|e| {
                        set.insert(e);
                    });
                },
                PrologQuery::Unify(a, b) => {
                    a.vars().into_iter().for_each(|e| { set.insert(e); });
                    b.vars().into_iter().for_each(|e| { set.insert(e); });
                },
                _ => {}
            }
        }

        let mut set = HashSet::new();
        goals_rec(self, &mut set);
        set
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrologInferenceEngine;
pub struct PrologUnificationContext;
impl SyntaxQueryBridge<PrologSyntax, PrologQuery> for PrologUnificationContext {
    fn syn(q: &PrologQuery) -> PrologSyntax {
        match q {
            PrologQuery::True => PrologSyntax::True,
            PrologQuery::False => PrologSyntax::False,
            PrologQuery::And(v1, v2) => PrologSyntax::And(Box::new(Self::syn(v1.as_ref())), Box::new(Self::syn(v2.as_ref()))),
            PrologQuery::Or(v1, v2) => PrologSyntax::Or(Box::new(Self::syn(v1.as_ref())), Box::new(Self::syn(v2.as_ref()))),
            PrologQuery::Atom(a) => PrologSyntax::Atom(a.clone()),
            PrologQuery::Var(v) => PrologSyntax::Var(v.clone()),
            PrologQuery::Fun(n, v) => PrologSyntax::Def(n.clone(), v.iter().map(Self::syn).collect(), vec![]),
            PrologQuery::Unify(_, _) => PrologSyntax::False,
        }
    }

    fn query(l: &PrologSyntax) -> PrologQuery {
        match l {
            PrologSyntax::True => PrologQuery::True,
            PrologSyntax::False => PrologQuery::False,
            PrologSyntax::And(v1, v2) => PrologQuery::And(Box::new(Self::query(v1.as_ref())), Box::new(Self::query(v2.as_ref()))),
            PrologSyntax::Or(v1, v2) => PrologQuery::Or(Box::new(Self::query(v1.as_ref())), Box::new(Self::query(v2.as_ref()))),
            PrologSyntax::Atom(a) => PrologQuery::Atom(a.clone()),
            PrologSyntax::Var(v) => PrologQuery::Var(v.clone()),
            PrologSyntax::Def(n, a, _) => PrologQuery::Fun(n.clone(), a.iter().map(Self::query).collect())
        }
    }
}

fn merge_envs(
    old: &mut InferenceEnvironment<PrologSyntax>,
    new: InferenceEnvironment<PrologSyntax>) -> bool {
    for (k, v) in new {
        if old.contains_key(&k) && old.get(&k) != Some(&v) {
            return false;
        } else if !old.contains_key(&k) {
            old.entry(k).or_insert(v);
        }
    }

    true
}

impl InferenceUnificationContext<PrologSyntax, PrologQuery> for PrologUnificationContext {
    fn unify(&self, q: &PrologQuery) -> Option<InferenceEnvironment<PrologSyntax>> {
        type Ctx = PrologUnificationContext;
        fn unify_pair(a: &PrologQuery, b: &PrologQuery) -> Option<InferenceEnvironment<PrologSyntax>> {
            let mut env = InferenceEnvironment::<PrologSyntax>::new();
            match (a, b) {
                (PrologQuery::True, PrologQuery::True) => {},
                (PrologQuery::Atom(n1), PrologQuery::Atom(n2)) if n1 == n2 => {},
                (v @ PrologQuery::Var(_), o) => { env.insert(Ctx::syn(v), Ctx::syn(o)); },
                (o, v @ PrologQuery::Var(_)) => { env.insert(Ctx::syn(v), Ctx::syn(o)); },
                (PrologQuery::Fun(n1, a1), PrologQuery::Fun(n2, a2)) if n1 == n2 && a1.len() == a2.len() => {
                    for i in 0..a1.len() {
                        if let Some(new_env) = unify_pair(&a1[i], &a2[i]) {
                            if !merge_envs(&mut env, new_env) {
                                return None;
                            }
                        }
                    }
                }
                _ => return None
            }

            Some(env)
        }

        match q {
            f@PrologQuery::Fun(_, _) => unify_pair(&f, &PrologQuery::True),
            PrologQuery::Unify(a, b) => unify_pair(a, b),
            _ => None
        }
    }
}

impl InferenceEngine<PrologSyntax, PrologQuery> for PrologInferenceEngine {
    fn pruning_steps() -> Vec<Box<dyn InferencePruningStep<PrologSyntax, PrologQuery>>> {
        vec![]
    }

    fn unifier() -> Box<dyn InferenceUnificationContext<PrologSyntax, PrologQuery>> {
        Box::new(PrologUnificationContext)
    }

    fn solve(
        query: &PrologQuery,
        goals: &mut Vec<PrologQuery>,
        env: &mut InferenceEnvironment<PrologSyntax>,
        choices: &mut Vec<InferenceChoice<PrologSyntax, PrologQuery>>,) {

        match query {
            PrologQuery::And(v1, v2) => {
                goals.push(v2.as_ref().clone());
                goals.push(v1.as_ref().clone());
            }

            PrologQuery::Or(v1, v2) => {
                choices.push((v2.as_ref().clone(), env.clone(), goals.clone()));
                goals.push(v1.as_ref().clone());
            }

            q@PrologQuery::Unify(_, _) => {
                if let Some(new_env) = Self::unifier().unify(q) {
                    if !merge_envs(env, new_env) {
                        goals.push(PrologQuery::False);
                    }
                }
            }

            PrologQuery::False => {
                if choices.is_empty() {
                    env.clear();
                    goals.clear();
                } else {
                    if let Some((body, e, next_goals)) = choices.pop() {
                        merge_envs(env, e);
                        for g in &next_goals {
                            goals.push(g.clone());
                        }
                        goals.push(body);
                    }
                }
            }

            other => {
                println!("{:?}", other);
            }
        }
    }
}