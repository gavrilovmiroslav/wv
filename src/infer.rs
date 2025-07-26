use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Query<L> {
    fn vars(&self) -> HashSet<L>;
}

#[derive(Debug, Clone, Default)]
pub enum Candidates<L> {
    #[default]
    All,
    Set(HashSet<L>),
}

pub type InferenceCandidateSet<L> = HashMap<L, Candidates<L>>;

#[derive(Debug, Clone)]
pub struct IntermediateQuery<'q, L, Q: Query<L>> {
    pub query: &'q Q,
    pub candidates: InferenceCandidateSet<L>
}

pub trait KnowledgeBase<L> {
    fn contains(&self, l: L) -> bool;
}

pub trait SyntaxQueryBridge<L, Q> {
    fn syn(q: &Q) -> L;
    fn query(l: &L) -> Q;
}

pub trait InferencePruningStep<L, Q: Query<L>> {
    fn prune(&self, knowledge: &dyn KnowledgeBase<L>, query: IntermediateQuery<L, Q>) -> IntermediateQuery<L, Q>;
}

pub type InferenceEnvironment<L> = HashMap<L, L>;

pub trait InferenceUnificationContext<L: PartialEq + Eq + Hash, Q: Query<L>> {
    fn unify(&self, q: &Q) -> Option<InferenceEnvironment<L>>;
}

pub type InferenceChoice<L, Q> = (Q, InferenceEnvironment<L>, Vec<Q>);

pub trait InferenceEngine<L: Debug + Clone + PartialEq + Eq + Hash, Q: Query<L> + Debug> {
    fn pruning_steps() -> Vec<Box<dyn InferencePruningStep<L, Q>>>;
    fn unifier() -> Box<dyn InferenceUnificationContext<L, Q>>;
    fn solve(query: &Q, goals: &mut Vec<Q>,
             env: &mut InferenceEnvironment<L>,
             choices: &mut Vec<InferenceChoice<L, Q>>);

    fn infer(knowledge: &dyn KnowledgeBase<L>, query: Q) -> Vec<InferenceEnvironment<L>> {
        let mut iq = IntermediateQuery { query: &query, candidates: InferenceCandidateSet::new() };

        query.vars().iter().for_each(|f| {
            iq.candidates.insert(f.clone(), Candidates::All);
        });

        let pruning = Self::pruning_steps();
        for p in &pruning {
            iq = p.prune(knowledge, iq);
        }

        iq.candidates.iter().for_each(|(l, c)| {
            println!("Candidates({:?}) = {:?}", l, c);
        });

        let mut goals = vec![ query ];
        let mut choices = vec![];
        let mut env = InferenceEnvironment::<L>::new();

        while let Some(goal) = goals.pop() {
            Self::solve(&goal, &mut goals, &mut env, &mut choices);
        }

        println!("{:?}", env);
        vec![ env ]
    }
}

