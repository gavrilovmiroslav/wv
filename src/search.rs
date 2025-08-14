use std::collections::{HashMap, HashSet};
use multimap::MultiMap;
use scryer_prolog::machine::config::MachineConfig;
use scryer_prolog::machine::parsed_results::QueryResolution::Matches;
use scryer_prolog::machine::parsed_results::Value;
use crate::core::{EntityId, Weave};
use crate::r#move::{arrows_in, arrows_out, down};

pub fn pattern_match(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Vec<HashMap<EntityId, EntityId>> {
    fn generate_product(wv: &Weave, entities: &[EntityId], candidates: MultiMap<EntityId, EntityId>) -> Vec<HashMap<EntityId, EntityId>> {
        fn rec_generate_product(index: usize, entities: &[EntityId], used: &[EntityId],
                                collected: &mut HashMap<EntityId, EntityId>,
                                candidates: &MultiMap<EntityId, EntityId>,
                                ret: &mut Vec<HashMap<EntityId, EntityId>>) {

            let next = entities[index];

            for candidate in candidates.get(&next) {
                if used.contains(candidate) {
                    continue;
                }

                collected.insert(next, *candidate);
                if index < entities.len() - 1 {
                    rec_generate_product(index + 1, entities, &[&used[..], &[*candidate]].concat(), collected, candidates, ret);
                } else {
                    ret.push(collected.clone());
                }
                collected.remove(&next);
            }
        }

        let mut ret = Vec::new();
        rec_generate_product(0, entities, &[], &mut HashMap::new(), &candidates, &mut ret);

        ret
    }

    let in_pattern = down(wv, hoist_pattern);
    let in_target = down(wv, hoist_target);

    let mut degrees = MultiMap::new();
    let mut candidates = MultiMap::new();

    for entity in &in_pattern {
        let in_degree = arrows_in(wv, &[ *entity ]).len();
        let out_degree = arrows_out(wv, &[ *entity ]).len();
        degrees.insert(*entity, (in_degree, out_degree));
    }

    for entity in &in_target {
        let in_degree = arrows_in(wv, &[ *entity ]).len();
        let out_degree = arrows_out(wv, &[ *entity ]).len();
        for (&candidate, &(in_d, out_d)) in degrees.iter() {
            if in_degree >= in_d && out_degree >= out_d {
                candidates.insert(*entity, candidate);
            }
        }
    }

    println!("{:?}", degrees);
    println!("{:?}", candidates);

    generate_product(wv, &in_target, candidates)
}

pub fn pattern_match_bindings(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Vec<HashMap<EntityId, EntityId>> {
    let in_pattern = down(wv, hoist_pattern);
    let in_target = down(wv, hoist_target);

    let mut variable_entities = HashMap::new();
    let mut variables = vec![];
    let mut ptn_ids = vec![];
    let mut ptn_sources = vec![];
    let mut ptn_targets = vec![];

    for entity in &in_pattern {
        variable_entities.insert(format!("X{}", entity), entity);
        variables.push(format!("X{}", entity));
        ptn_sources.push(format!("\twv_src(X{}, X{})", entity, wv.src(*entity)));
        ptn_targets.push(format!("\twv_tgt(X{}, X{})", entity, wv.tgt(*entity)));
    }

    ptn_ids.extend(ptn_sources);
    ptn_ids.extend(ptn_targets);

    let vars = variables.join(", ");
    let query_definition = format!("wv_query({}) :-\n{}.", vars, ptn_ids.join(", \n"));
    let query = format!("wv_query({}).", vars);

    let mut tgt_ids = vec![];
    let mut tgt_sources = vec![];
    let mut tgt_targets = vec![];

    for entity in in_target {
        tgt_sources.push(format!("wv_src({}, {}).", entity, wv.src(entity)));
        tgt_targets.push(format!("wv_tgt({}, {}).", entity, wv.tgt(entity)));
    }

    tgt_ids.extend(tgt_sources);
    tgt_ids.extend(tgt_targets);
    let data_entries = tgt_ids.join("\n");
    let code = format!("{}\n\n{}\nmain :- {}\n:- initialization(main).", data_entries, query_definition, query);

    //println!("{}", code.clone());
    //println!("{}", query.clone());

    let mut results = vec![];
    let mut m = scryer_prolog::machine::Machine::new(MachineConfig::default());

    m.consult_module_string("user", code);
    if let Ok(Matches(result)) = m.run_query(query) {
        'out: for mtch in result {
            let mut unique = HashSet::new();
            let mut binding = HashMap::new();
            for (s, v) in mtch.bindings.into_iter() {
                let value = match v {
                    Value::Float(f) => f.0 as EntityId,
                    _ => Weave::NIL
                };
                if unique.contains(&value)
                {
                   continue 'out;
                }
                unique.insert(value);
                binding.insert(**variable_entities.get(&s).unwrap(), value);
            }

            results.push(binding);
        }

        return results;
    } else {
        return vec![];
    }
}