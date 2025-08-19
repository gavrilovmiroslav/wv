use std::collections::{HashMap, HashSet};
use multimap::MultiMap;
use crate::core::{DataField, DataValue, EntityId, Weave};
use crate::shape::{annotate};
use crate::traverse::{arrows_in, arrows_out, down, marks};

#[derive(Debug, Clone, PartialEq)]
pub enum Diff {
    Spawn(EntityId, (EntityId, EntityId)),
    ChangeSource(EntityId, EntityId),
    ChangeTarget(EntityId, EntityId),
    Destroy(EntityId),
    ChangeData(EntityId, String, Vec<DataField>),
}

#[derive(Debug)]
pub(crate) struct SearchSpace {
    entities: Vec<EntityId>,
    candidates: MultiMap<EntityId, EntityId>,
}

pub(crate) fn generate_single_product(wv: &Weave, search_space: &SearchSpace, seed: HashMap<EntityId, EntityId>) -> Option<HashMap<EntityId, EntityId>> {
    fn rec_generate_product(wv: &Weave, index: usize,
                            search_space: &SearchSpace,
                            used: &mut Vec<EntityId>,
                            collected: &mut HashMap<EntityId, EntityId>) -> Option<HashMap<EntityId, EntityId>> {

        let next = search_space.entities[index];

        for candidate in search_space.candidates.get_vec(&next).unwrap() {
            if used.contains(candidate) {
                continue;
            }

            used.push(*candidate);
            collected.insert(next, *candidate);
            if index < search_space.entities.len() - 1 {
                if let Some(res) = rec_generate_product(wv, index + 1, search_space, used, collected) {
                    return Some(res);
                }
            } else {
                if check_solution(wv, &collected) {
                    return Some(collected.clone());
                }
            }

            collected.remove(&next);
            used.remove(used.iter().position(|e| e == candidate).unwrap());
        }

        None
    }

    let mut seed = seed;
    rec_generate_product(wv, 0, search_space, &mut vec![], &mut seed)
}

pub(crate) fn generate_products(wv: &Weave, search_space: &SearchSpace, seed: HashMap<EntityId, EntityId>) -> Vec<HashMap<EntityId, EntityId>> {
    fn rec_generate_products(wv: &Weave, index: usize,
                             search_space: &SearchSpace,
                             used: &mut Vec<EntityId>,
                             collected: &mut HashMap<EntityId, EntityId>,
                             ret: &mut Vec<HashMap<EntityId, EntityId>>) {

        let next = search_space.entities[index];

        if let Some(v) = search_space.candidates.get_vec(&next) {
            for candidate in v {
                if used.contains(candidate) {
                    continue;
                }

                used.push(*candidate);
                collected.insert(next, *candidate);
                if index < search_space.entities.len() - 1 {
                    rec_generate_products(wv, index + 1, search_space, used, collected, ret);
                } else {
                    if check_solution(wv, &collected) {
                        ret.push(collected.clone());
                    }
                }

                collected.remove(&next);
                used.remove(used.iter().position(|e| e == candidate).unwrap());
            }
        }
    }

    let mut ret = Vec::new();
    let mut seed = seed;
    rec_generate_products(wv, 0, search_space, &mut vec![], &mut seed, &mut ret);

    ret
}

pub(crate) fn check_solution(wv: &Weave, solution: &HashMap<EntityId, EntityId>) -> bool {
    let keys = solution.keys().collect::<HashSet<_>>();
    for (node, _) in solution {
        for dep in wv.get_dependents(*node) {
            if !keys.contains(&dep) {
                continue;
            }

            let (ls, lt) = (wv.src(dep), wv.tgt(dep));
            let r = *solution.get(&dep).unwrap();
            let (rs, rt) = (wv.src(r), wv.tgt(r));
            let okay = *solution.get(&ls).unwrap() == rs && *solution.get(&lt).unwrap() == rt;
            if !okay {
                return false;
            }
        }
    }

    true
}

pub(crate) fn prepare_search_space(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId, seed: &HashMap<EntityId, EntityId>) -> Option<SearchSpace> {
    fn get_component_name<'s>(wv: &Weave, e: EntityId) -> String {
        if let DataValue::String(s) = wv.get_component(e, "With").first().unwrap() {
            s.clone()
        } else {
            panic!("Component name isn't a string!");
        }
    }

    let mut seed_vals: HashMap<EntityId, EntityId> = HashMap::new();
    for (k, v) in seed {
        seed_vals.insert(*v, *k);
    }

    let mut in_pattern = down(wv, hoist_pattern);
    let in_target = down(wv, hoist_target);

    let mut degrees = MultiMap::new();
    let mut with_components = HashMap::new();
    let mut without_components = HashMap::new();
    let mut candidates = MultiMap::new();

    for entity in &in_pattern {
        let in_degree = arrows_in(wv, &[ *entity ]).len();
        let out_degree = arrows_out(wv, &[ *entity ]).len();
        let withs = marks(wv, &[ *entity ]).iter()
            .filter(|&m| wv.has_component(*m, "With"))
            .map(|&m| get_component_name(wv, m))
            .collect::<Vec<_>>();
        let withouts = marks(wv, &[ *entity ]).iter()
            .filter(|&m| wv.has_component(*m, "Without"))
            .map(|&m| get_component_name(wv, m))
            .collect::<Vec<_>>();
        degrees.insert(*entity, (in_degree, out_degree));
        with_components.insert(*entity, withs);
        without_components.insert(*entity, withouts);
    }

    for entity in &in_target {
        if seed_vals.contains_key(&entity) {
            candidates.insert(seed_vals.get(&entity).unwrap().clone(), *entity);
            continue;
        }

        let mut has_candidates = false;
        let in_degree = arrows_in(wv, &[ *entity ]).len();
        let out_degree = arrows_out(wv, &[ *entity ]).len();
        'candidates: for (&candidate, &(in_d, out_d)) in degrees.iter() {
            if in_degree >= in_d && out_degree >= out_d {
                let withs = with_components.get(&candidate).unwrap();
                for with in withs {
                    if !wv.has_component(*entity, with) {
                        continue 'candidates;
                    }
                }

                let withouts = without_components.get(&candidate).unwrap();
                for without in withouts {
                    if wv.has_component(*entity, without) {
                        continue 'candidates;
                    }
                }

                if !seed.contains_key(&candidate) {
                    candidates.insert(candidate, *entity);
                    has_candidates = true;
                }
            }
        }

        if !has_candidates {
            return None;
        }
    }

    if candidates.len() >= in_pattern.len() {
        in_pattern.sort_by(|a, b| {
            let za = candidates.get_vec(a).unwrap().len();
            let zb = candidates.get_vec(b).unwrap().len();
            za.cmp(&zb)
        });
    }

    Some(SearchSpace {
        entities: in_pattern,
        candidates,
    })
}

pub fn require_component(wv: &mut Weave, entity: EntityId, name: &str) {
    annotate(wv, entity, "With", &[ DataValue::String(name.to_string()) ]);
}

pub fn require_no_component(wv: &mut Weave, entity: EntityId, name: &str) {
    annotate(wv, entity, "Without", &[ DataValue::String(name.to_string()) ]);
}

pub fn find_all(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Vec<HashMap<EntityId, EntityId>> {
    let seed = HashMap::default();
    if let Some(search_space) = prepare_search_space(wv, hoist_pattern, hoist_target, &seed) {
        generate_products(wv, &search_space, seed)
    } else {
        vec![]
    }
}

pub fn find_one(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Option<HashMap<EntityId, EntityId>> {
    let seed = HashMap::default();
    if let Some(search_space) = prepare_search_space(wv, hoist_pattern, hoist_target, &seed) {
        generate_single_product(wv, &search_space, seed)
    } else {
        None
    }
}