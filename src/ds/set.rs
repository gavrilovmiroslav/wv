use std::collections::BTreeSet;
use itertools::Itertools;
use crate::core::{EntityId, Weave};
use crate::shape::{hoist, unhoist, unhoist_all_from};
use crate::traverse::{down};

pub fn set_new(wv: &mut Weave) -> EntityId {
    let set = wv.new_knot();
    let sentinel = wv.new_tether(set);
    wv.change_tgt(set, sentinel);

    assert!(is_set(wv, set));
    set
}

pub(crate) fn is_set(wv: &Weave, set: EntityId) -> bool {
    let sentinel = wv.tgt(set);
    wv.src(sentinel) == set && sentinel != set
}

pub fn set_add(wv: &mut Weave, set: EntityId, els: &[EntityId]) {
    assert!(is_set(wv, set));

    let to_remove = BTreeSet::from_iter(set_members(wv, set));
    let mut es = els.to_owned();
    es.retain(|e| !to_remove.contains(e));
    hoist(wv, wv.tgt(set), &es);
}

fn set_members(wv: &Weave, set: EntityId) -> Vec<EntityId> {
    assert!(is_set(wv, set));

    down(wv, wv.tgt(set))
}

pub fn set_contains(wv: &Weave, set: EntityId, e: EntityId) -> bool {
    assert!(is_set(wv, set));

    down(wv, wv.tgt(set)).iter().contains(&e)
}

pub fn set_remove(wv: &mut Weave, set: EntityId, e: EntityId) {
    assert!(is_set(wv, set));

    unhoist(wv, wv.tgt(set), &[e]);
}

pub fn set_delete(wv: &mut Weave, set: EntityId) {
    assert!(is_set(wv, set));

    unhoist_all_from(wv, wv.tgt(set));
    wv.delete_cascade(set);
}