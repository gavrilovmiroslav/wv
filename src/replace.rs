use std::collections::HashMap;
use crate::core::{DataValue, EntityId, Weave};
use crate::search::{generate_products, prepare_search_space};
use crate::shape::get_annotation;
use crate::traverse::down;

#[derive(Debug)]
pub enum ReplaceError {
    FailedToMatchUniqueGoal(Vec<HashMap<EntityId, EntityId>>),
    FailedToFindUniqueTarget,
}

pub(crate) fn get_match_mapping(wv: &Weave, hoisted_pattern: EntityId, hoisted_goal: EntityId) -> Result<HashMap<EntityId, EntityId>, ReplaceError> {
    let mut result = HashMap::default();

    let goal = down(wv, hoisted_goal);
    for motif in &goal {
        let ann = get_annotation(wv, *motif, "Identity");
        if let Some(ann) = ann {
            if let DataValue::Int(eid) = wv.get_component(ann, "Identity").first().unwrap() {
                result.insert(*eid as EntityId, *motif);
            }
        }
    }

    if let Some(search_space) = prepare_search_space(wv, hoisted_pattern, hoisted_goal, &result) {
        let prod = generate_products(wv, &search_space, result);
        if prod.len() != 1 {
            return Err(ReplaceError::FailedToMatchUniqueGoal(prod));
        }

        return Ok(prod.first().cloned().unwrap());
    }

    Err(ReplaceError::FailedToFindUniqueTarget)
}

pub fn replace(wv: &mut Weave,
               hoisted_pattern: EntityId,
               hoisted_goal: EntityId,
               hoisted_target: EntityId) -> Result<HashMap<EntityId, EntityId>, ReplaceError> {

    let pattern_to_goal = get_match_mapping(wv, hoisted_pattern, hoisted_goal);
    if let Ok(matching_goal) = pattern_to_goal {
        println!("PATTERN <-> GOAL: {:?}", matching_goal);
        let goal_matched = matching_goal.values().collect::<Vec<_>>();
        let goal_entities = down(wv, hoisted_goal);

        let new_entities = goal_entities.iter()
            .filter(|e| !goal_matched.contains(e))
            .collect::<Vec<_>>();

        let pattern_to_target =
            get_match_mapping(wv, hoisted_pattern, hoisted_target);

        if let Ok(matching_target) = pattern_to_target {
            println!("PATTERN <-> TARGET: {:?}", matching_target);
            let mut gt = HashMap::new();
            for (p, g) in &matching_goal {
                gt.insert(*g, matching_target.get(p).unwrap().clone());
            }

            for new_entity in new_entities {
                let spawned = wv.new_knot();
                gt.insert(*new_entity, spawned);
            }

            for goal in goal_entities {
                let (goal_src, goal_tgt) =
                    (wv.src(goal), wv.tgt(goal));

                let func = *gt.get(&goal).unwrap();
                let (func_src, func_tgt) =
                    (*gt.get(&goal_src).unwrap(), *gt.get(&goal_tgt).unwrap());

                wv.change_ends(func, func_src, func_tgt);
            }
            return Ok(gt);
        }

        Err(ReplaceError::FailedToFindUniqueTarget)
    } else {
        pattern_to_goal
    }
}