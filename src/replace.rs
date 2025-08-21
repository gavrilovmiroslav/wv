use std::cmp::Ordering;
use std::collections::{HashMap};
use multimap::MultiMap;
use crate::core::{DataValue, EntityId, Weave};
use crate::search::{find_one, prepare_search_space, SearchSpace};
use crate::shape::get_annotation;
use crate::traverse::down;

#[derive(Debug)]
pub enum ReplaceError {
    FailedToMatchUniqueGoal(Vec<HashMap<EntityId, EntityId>>),
    FailedToFindUniqueTarget,
}

pub(crate) fn generate_incomplete_products(wv: &Weave, search_space: &SearchSpace, seed: HashMap<EntityId, EntityId>)
    -> Vec<HashMap<EntityId, Option<EntityId>>> {

    fn rec_generate_incomplete_products(wv: &Weave, index: usize,
                                        seed: &Vec<EntityId>,
                                        search_space: &SearchSpace,
                                        used: &mut Vec<EntityId>,
                                        collected: &mut HashMap<EntityId, Option<EntityId>>,
                                        ret: &mut Vec<HashMap<EntityId, Option<EntityId>>>) {

        let next = search_space.entities[index];
        if let Some(v) = search_space.candidates.get_vec(&next) {
            for candidate in v {
                if used.contains(candidate) {
                    continue;
                }

                used.push(*candidate);
                collected.insert(next, Some(*candidate));
                if index < search_space.entities.len() - 1 {
                    rec_generate_incomplete_products(wv, index + 1, seed, search_space, used, collected, ret);
                } else {
                    println!("COMPLETE {:?}", collected);
                    if check_incomplete_solution(wv, &collected) {
                        ret.push(collected.clone());
                    }
                }

                collected.remove(&next);
                used.remove(used.iter().position(|e| e == candidate).unwrap());
            }

            if !seed.contains(&next) {
                if index < search_space.entities.len() - 1 {
                    collected.insert(next, None);
                    rec_generate_incomplete_products(wv, index + 1, seed, search_space, used, collected, ret);
                    collected.remove(&next);
                } else {
                    if check_incomplete_solution(wv, &collected) {
                        ret.push(collected.clone());
                    }
                }
            }
        }
    }

    let mut ret = Vec::new();
    let mut seed: HashMap<EntityId, Option<EntityId>> = seed.iter().map(|(k, v)| (*k, Some(*v))).collect();
    let mut seeded_search_space = SearchSpace { entities: search_space.entities.clone(), candidates: search_space.candidates.clone() };
    seeded_search_space.entities.sort_by(|a, b| {
        if seed.contains_key(a) && !seed.contains_key(b) {
            return Ordering::Less;
        } else if seed.contains_key(b) && !seed.contains_key(a) {
            return Ordering::Greater;
        }

        if search_space.candidates.get_vec(a).is_none() || search_space.candidates.get_vec(b).is_none() {
            return Ordering::Equal;
        }

        let za = search_space.candidates.get_vec(a).unwrap().len();
        let zb = search_space.candidates.get_vec(b).unwrap().len();
        za.cmp(&zb)
    });

    let seed_keys = seed.keys().cloned().collect::<Vec<_>>();
    rec_generate_incomplete_products(wv, 0, &seed_keys, &seeded_search_space, &mut vec![], &mut seed, &mut ret);
    ret.iter().filter(|&h| h.len() == search_space.entities.len()).cloned().collect()
}

pub(crate) fn check_incomplete_solution(wv: &Weave, solution: &HashMap<EntityId, Option<EntityId>>) -> bool {
    println!("  CHECKING {:?}", solution);
    for (node, cand) in solution {
        if let Some(cand) = cand {
            let (ls, lt) = (wv.src(*node), wv.tgt(*node));
            let (rs, rt) = (wv.src(*cand), wv.tgt(*cand));

            println!("    P: {} = {} -> {}", node, ls, lt);
            println!("    G: {} = {} -> {}", cand, rs, rt);
            let (candl, candr) = (solution.get(&ls), solution.get(&lt));
            println!("    D: {} = {:?} -> {:?}", cand, candl, candr);
            if candl.is_none() || candr.is_none()
            {
                return false;
            }
            else {
                let candl = candl.unwrap();
                let candr = candr.unwrap();
                if candl.is_none() || candr.is_none() {
                    return false;
                }

                if candl.unwrap() != rs || candr.unwrap() != rt {
                    return false;
                }
            }
        }
    }

    true
}

pub(crate) fn get_match_mapping(wv: &Weave, hoisted_pattern: EntityId, hoisted_goal: EntityId)
    -> Result<HashMap<EntityId, Option<EntityId>>, ReplaceError> {

    let mut annotated_identities = HashMap::default();

    let goal = down(wv, hoisted_goal);
    for motif in &goal {
        let ann = get_annotation(wv, *motif, "Identity");
        if let Some(ann) = ann {
            if let DataValue::Entity(eid) = wv.get_component(ann, "Identity").first().unwrap() {
                annotated_identities.insert(*eid as EntityId, *motif);
            }
        }
    }

    if let Some(search_space) = prepare_search_space(wv, hoisted_pattern, hoisted_goal, &annotated_identities) {
        println!("{:?}", search_space);
        println!("{:?}", annotated_identities);
        let prod = generate_incomplete_products(wv, &search_space, annotated_identities);
        println!("PROD {:?}", prod);

        if let Some(r) = prod.first() {
            return Ok(r.clone());
        }
    }

    Err(ReplaceError::FailedToFindUniqueTarget)
}

pub fn replace(wv: &mut Weave,
               hoisted_pattern: EntityId,
               hoisted_goal: EntityId,
               hoisted_target: EntityId) -> Result<MultiMap<Option<EntityId>, Option<EntityId>>, ReplaceError> {

    let pattern_to_goal = get_match_mapping(wv, hoisted_pattern, hoisted_goal);
    if let Ok(matching_goal) = pattern_to_goal {
        println!("1. PATTERN <-> GOAL: {:?}", matching_goal);
        let goal_matched = matching_goal.values().collect::<Vec<_>>();
        let goal_entities = down(wv, hoisted_goal);

        let new_entities = goal_entities.iter()
            .filter(|e| !goal_matched.contains(&&Some(**e)))
            .collect::<Vec<_>>();

        let pattern_to_target = find_one(wv, hoisted_pattern, hoisted_target);

        println!("PT {:?}", pattern_to_target);

        if let Some(matching_target) = pattern_to_target {
            println!("2. PATTERN <-> TARGET: {:?}", matching_target);
            let mut gt: MultiMap<Option<EntityId>, Option<EntityId>> = MultiMap::new();
            for (p, g) in &matching_goal {
                gt.insert(*g, matching_target.get(p).cloned());
            }

            for new_entity in new_entities {
                let spawned = wv.new_knot();
                gt.insert(Some(*new_entity), Some(spawned));
            }

            for goal in goal_entities {
                let (goal_src, goal_tgt) = (wv.src(goal), wv.tgt(goal));
                let func = *gt.get(&Some(goal)).unwrap();
                let (func_src, func_tgt) =
                    (*gt.get(&Some(goal_src)).unwrap(), *gt.get(&Some(goal_tgt)).unwrap());

                wv.change_ends(func.unwrap(), func_src.unwrap(), func_tgt.unwrap());
            }

            if let Some(nones) = gt.get_vec(&None) {
                for none in nones {
                    if let Some(e) = none {
                        println!("DELETE {:?}", e);
                        wv.delete_cascade(*e);
                    }
                }
            }

            return Ok(gt);
        }

        Err(ReplaceError::FailedToFindUniqueTarget)
    } else {
        Err(ReplaceError::FailedToFindUniqueTarget)
    }
}