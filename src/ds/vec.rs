use crate::core::{EntityId, Weave};
use crate::shape::{hoist, unhoist_all_from};
use crate::traverse::{down_n, tether};

pub fn vec_new(wv: &mut Weave) -> EntityId {
    let vec = wv.new_knot();
    let head = wv.new_mark(vec);
    wv.change_tgt(vec, head);

    assert!(is_vec(wv, vec));
    vec
}

pub(crate)  fn is_vec(wv: &Weave, vec: EntityId) -> bool {
    let sentinel = wv.tgt(vec);
    wv.tgt(sentinel) == vec && sentinel != vec
}

pub fn vec_add_first(wv: &mut Weave, vec: EntityId, el: EntityId) {
    assert!(is_vec(wv, vec));

    if let Some(tail) = tether(wv, vec) {
        let new = wv.new_tether(vec);
        hoist(wv, new, &[el]);
        wv.change_src(tail, new);
    }
}

pub fn vec_add_last(wv: &mut Weave, vec: EntityId, el: EntityId) {
    assert!(is_vec(wv, vec));

    let head = wv.tgt(vec);
    let last = wv.tgt(head);
    let next = wv.new_tether(last);
    hoist(wv, next, &[el]);
    wv.change_tgt(head, next);
}

fn vec_is_empty(wv: &Weave, vec: EntityId) -> bool {
    assert!(is_vec(wv, vec));

    let head = wv.tgt(vec);
    let last = wv.tgt(head);
    last == vec
}

fn vec_members(wv: &Weave, vec: EntityId) -> Vec<EntityId> {
    assert!(is_vec(wv, vec));

    let mut entities = vec![];
    let mut current = vec;

    while let Some(next) = tether(wv, current) {
        entities.push(next);
        current = next;
    }

    entities
}

pub fn vec_elements(wv: &Weave, vec: EntityId) -> Vec<EntityId> {
    assert!(is_vec(wv, vec));

    down_n(wv, &vec_members(wv, vec))
}

pub fn vec_remove(wv: &mut Weave, vec: EntityId, e: EntityId) {
    assert!(is_vec(wv, vec));

    let mut current = vec;
    let head = wv.tgt(vec);
    let last = wv.tgt(head);
    while let Some(next) = tether(wv, current) {
        if next == e {
            unhoist_all_from(wv, next);
            if last == next {
                wv.change_tgt(head, current);
            } else {
                let after = tether(wv, next).unwrap();
                wv.change_src(after, current);
            }
            wv.delete_cascade(next);
        }
        current = next;
    }
}

pub fn vec_delete(wv: &mut Weave, vec: EntityId) {
    assert!(is_vec(wv, vec));

    vec_members(wv, vec).iter().for_each(|e| unhoist_all_from(wv, *e));
    wv.delete_cascade(vec);
}