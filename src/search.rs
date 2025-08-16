use std::collections::{HashMap, HashSet};
use multimap::MultiMap;
use crate::core::{DataValue, EntityId, Weave};
use crate::traverse::{arrows_in, arrows_out, down, marks};

#[derive(Debug)]
struct SearchSpace {
    entities: Vec<EntityId>,
    candidates: MultiMap<EntityId, EntityId>,
}

fn generate_single_product(wv: &Weave, search_space: &SearchSpace) -> Option<HashMap<EntityId, EntityId>> {
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

    rec_generate_product(wv, 0, search_space, &mut vec![], &mut HashMap::new())
}

fn generate_products(wv: &Weave, search_space: &SearchSpace) -> Vec<HashMap<EntityId, EntityId>> {
    fn rec_generate_products(wv: &Weave, index: usize,
                             search_space: &SearchSpace,
                             used: &mut Vec<EntityId>,
                             collected: &mut HashMap<EntityId, EntityId>,
                             ret: &mut Vec<HashMap<EntityId, EntityId>>) {

        let next = search_space.entities[index];

        for candidate in search_space.candidates.get_vec(&next).unwrap() {
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

    let mut ret = Vec::new();
    rec_generate_products(wv, 0, search_space, &mut vec![], &mut HashMap::new(), &mut ret);

    ret
}

fn check_solution(wv: &Weave, solution: &HashMap<EntityId, EntityId>) -> bool {
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

fn prepare_search_space(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Option<SearchSpace> {
    fn get_component_name<'s>(wv: &Weave, e: EntityId) -> String {
        if let DataValue::String(s) = wv.get_component(e, "With").first().unwrap() {
            s.clone()
        } else {
            panic!("Component name isn't a string!");
        }
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

                candidates.insert(candidate, *entity);
                has_candidates = true;
            }
        }

        if !has_candidates {
            return None;
        }
    }

    in_pattern.sort_by(|a, b| {
        let za = candidates.get_vec(a).unwrap().len();
        let zb = candidates.get_vec(b).unwrap().len();
        za.cmp(&zb)
    });

    Some(SearchSpace {
        entities: in_pattern,
        candidates,
    })
}

pub fn find_all(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Vec<HashMap<EntityId, EntityId>> {
    if let Some(search_space) = prepare_search_space(wv, hoist_pattern, hoist_target) {
        generate_products(wv, &search_space)
    } else {
        vec![]
    }
}

pub fn find_one(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Option<HashMap<EntityId, EntityId>> {
    if let Some(search_space) = prepare_search_space(wv, hoist_pattern, hoist_target) {
        generate_single_product(wv, &search_space)
    } else {
        None
    }
}