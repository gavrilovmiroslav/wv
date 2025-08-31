use std::collections::HashSet;
use crate::core::{EntityId, Weave};

pub fn primary(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h: HashSet<EntityId> = HashSet::new();
    for i in it {
        let di = wv.get_dependents(*i)
            .iter().filter(|&e| wv.is_knot(*e) || wv.is_arrow(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(&di);
    }

    h.into_iter().collect()
}

pub fn virtuals(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h: HashSet<EntityId> = HashSet::new();
    for i in it {
        let di = wv.get_dependents(*i)
            .iter().filter(|&e| wv.is_mark(*e) || wv.is_tether(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(&di);
    }

    h.into_iter().collect()
}

pub fn deps(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h: HashSet<EntityId> = HashSet::new();
    for i in it {
        let di = wv.get_dependents(*i);
        h.extend(&di);
    }

    let all = h.into_iter().collect();
    all
}

pub fn external_deps(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h: HashSet<EntityId> = HashSet::new();
    for i in it {
        let di = wv.get_external_dependents(*i);
        h.extend(&di);
    }

    let all = h.into_iter().collect();
    all
}

pub fn arrows(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h = HashSet::new();
    for i in it {
        let v = &wv.get_dependents(*i)
            .iter().filter(|&e| *e != *i && wv.is_arrow(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(v);
    }
    h.into_iter().collect()
}

pub fn arrows_between(wv: &Weave, from: &[EntityId], to: &[EntityId]) -> Vec<EntityId> {
    let mut h = HashSet::new();
    h.extend(arrows_out(wv, from));
    let in_arrows = arrows_in(wv, to);
    h.retain(|e| in_arrows.contains(e));
    h.into_iter().collect()
}

pub fn arrows_in(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h = HashSet::new();
    for i in it {
        let v = &wv.get_dependents_for_target(*i)
            .iter().filter(|&e| *e != *i && wv.is_arrow(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(v);
    }
    h.into_iter().collect()
}

pub fn marks(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h = HashSet::new();
    for i in it {
        let v = &wv.get_dependents_for_target(*i)
            .iter().filter(|&e| *e != *i && wv.is_mark(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(v);
    }
    h.into_iter().collect()
}

pub fn tether(wv: &Weave, id: EntityId) -> Option<EntityId> {
    wv.get_dependents_for_source(id)
        .iter().filter(|e| **e != id && wv.is_tether(**e))
        .cloned().collect::<Vec<_>>().first().copied()
}

pub fn tethers(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h = HashSet::new();
    for i in it {
        let v = &wv.get_dependents_for_source(*i)
            .iter().filter(|&e| *e != *i && wv.is_tether(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(v);
    }
    h.into_iter().collect()
}

pub fn arrows_out(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut h = HashSet::new();
    for i in it {
        let v = &wv.get_dependents_for_source(*i)
            .iter().filter(|&e| *e != *i && wv.is_arrow(*e))
            .cloned().collect::<Vec<_>>();
        h.extend(v);
    }
    h.into_iter().collect()
}

pub fn to_src(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    it.iter().map(|e| wv.src(*e)).collect()
}

pub fn to_tgt(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    it.iter().map(|e| {
        wv.tgt(*e)
    }).collect()
}

pub fn other_edge(wv: &Weave, e: EntityId) -> EntityId {
    wv.tgt(e)
}

pub fn hop(wv: &Weave, e: EntityId) -> EntityId {
    let t = wv.tgt(e);
    if wv.is_arrow(t) {
        wv.src(t)
    } else {
        t
    }
}

pub fn neighbors(wv: &Weave, e: EntityId) -> Vec<EntityId> {
    arrows_out(wv, &[e]).iter().map(|e| hop(wv, *e)).collect()
}

pub fn prev(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    let ds = external_deps(wv, &[it]);
    let mut ts = to_src(wv, &ds);
    ts.sort();
    ts.dedup();
    ts.iter().filter(|&e| *e != it).cloned().collect()
}

pub fn prev_n(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut c = it.iter().flat_map(|e| prev(wv, *e)).collect::<Vec<_>>();
    c.sort();
    c.dedup();
    c
}

pub fn next(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    let ds = external_deps(wv, &[it]);
    let mut ts = to_tgt(wv, &ds);
    ts.sort();
    ts.dedup();
    ts.iter().filter(|&e| *e != it).cloned().collect()
}

pub fn next_n(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut c = it.iter().flat_map(|e| next(wv, *e)).collect::<Vec<_>>();
    c.sort();
    c.dedup();
    c
}

//  s --> t ==> m--> o
//  tethers
//        arrows_out
//              to_tgt
//                  to_tgt
pub fn down(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    to_tgt(wv, &to_tgt(wv, &arrows_out(wv, &tethers(wv, &[it]))))
}

pub fn down_half(wv: &Weave, it: EntityId) -> Option<EntityId> {
    arrows_out(wv, &tethers(wv, &[it])).first().copied()
}

pub fn up_half(wv: &Weave, arr: EntityId) -> Option<EntityId> {
    to_src(wv, &to_src(wv, &[arr])).first().copied()
}

pub fn down_n(wv: &Weave, its: &[EntityId]) -> Vec<EntityId> {
    let mut c = its.iter().flat_map(|i| down(wv, *i)).collect::<Vec<_>>();
    c.sort();
    c.dedup();
    c
}

//  s --> t ==> m--> o
//               marks
//           arrows_in
//        to_src
//  to_src
pub fn up(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    let marks = marks(wv, &[it]);
    let arrows = arrows_in(wv, &marks);
    let tethers = to_src(wv, &arrows);
    let entities = to_src(wv, &tethers);
    entities
}

pub fn up_n(wv: &Weave, its: &[EntityId]) -> Vec<EntityId> {
    let mut c = its.iter().flat_map(|i| up(wv, *i)).collect::<Vec<_>>();
    c.sort();
    c.dedup();
    c
}