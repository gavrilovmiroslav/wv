use std::collections::{HashMap, HashSet};
use multimap::MultiMap;
use crate::core::{EntityId, Weave};
use crate::traverse::{arrows_in, arrows_out, down};

pub fn pattern_match(wv: &Weave, hoist_pattern: EntityId, hoist_target: EntityId) -> Vec<HashMap<EntityId, EntityId>> {
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

    fn generate_product(wv: &Weave, entities: &[EntityId], candidates: MultiMap<EntityId, EntityId>) -> Vec<HashMap<EntityId, EntityId>> {
        fn rec_generate_product(wv: &Weave, index: usize, entities: &[EntityId], used: &mut Vec<EntityId>,
                                collected: &mut HashMap<EntityId, EntityId>,
                                candidates: &MultiMap<EntityId, EntityId>,
                                ret: &mut Vec<HashMap<EntityId, EntityId>>) {

            let next = entities[index];

            for candidate in candidates.get_vec(&next).unwrap() {
                if used.contains(candidate) {
                    continue;
                }

                used.push(*candidate);
                collected.insert(next, *candidate);
                if index < entities.len() - 1 {
                    rec_generate_product(wv,index + 1, entities, used, collected, candidates, ret);
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
        rec_generate_product(wv, 0, entities, &mut vec![], &mut HashMap::new(), &candidates, &mut ret);

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
                candidates.insert(candidate, *entity);
            }
        }
    }

    let mut entities = vec![];
    entities.extend(in_pattern);
    entities.sort_by(|a, b| {
        let za = candidates.get_vec(a).unwrap().len();
        let zb = candidates.get_vec(b).unwrap().len();
        za.cmp(&zb)
    });

    generate_product(wv, &entities, candidates)
}