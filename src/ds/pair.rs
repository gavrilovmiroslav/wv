use crate::core::{EntityId, Weave};
use crate::shape::{hoist, unhoist_all_from};
use crate::traverse::{down, tethers};

pub fn pair_new(wv: &mut Weave, l: &[EntityId], r: &[EntityId]) -> EntityId {
    let pair = wv.new_knot();
    let left = wv.new_tether(pair);
    let right = wv.new_tether(pair);
    let ordering = wv.new_arrow(left, right);
    hoist(wv, left, l);
    hoist(wv, right, r);
    wv.change_tgt(pair, ordering);

    assert!(is_pair(wv, pair));
    pair
}

pub(crate) fn is_pair(wv: &Weave, pair: EntityId) -> bool {
    let branches = tethers(wv, &[pair]);
    if branches.len() == 2 && wv.is_arrow(pair) {
        return branches.contains(&wv.src(pair))
            && branches.contains(&wv.tgt(pair));
    }

    false
}

pub fn pair_left(wv: &mut Weave, pair: EntityId) -> Vec<EntityId>  {
    assert!(is_pair(wv, pair));

    let arr = wv.tgt(pair);
    down(wv, wv.src(arr))
}

pub fn pair_right(wv: &mut Weave, pair: EntityId) -> Vec<EntityId>  {
    assert!(is_pair(wv, pair));

    let arr = wv.tgt(pair);
    down(wv, wv.tgt(arr))
}

pub fn pair_delete(wv: &mut Weave, pair: EntityId) {
    assert!(is_pair(wv, pair));

    let arr = wv.tgt(pair);
    let (src, tgt) = (wv.src(arr), wv.tgt(arr));
    unhoist_all_from(wv, src);
    unhoist_all_from(wv, tgt);
    wv.delete_cascade(pair);
}