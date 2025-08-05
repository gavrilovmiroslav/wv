use std::collections::HashSet;
use crate::core::{EntityId, Weave};

pub fn deps(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
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

pub fn prev(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    let ds = deps(wv, &[it]);
    let mut ts = to_src(wv, &ds);
    ts.sort();
    ts.dedup();
    ts.iter().filter(|&e| *e != it).cloned().collect()
}

pub fn prev_n(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut c = it.iter().flat_map(|e| prev(wv, *e)).collect::<Vec<_>>();;
    c.sort();
    c.dedup();
    c
}

pub fn next(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    let ds = deps(wv, &[it]);
    let mut ts = to_tgt(wv, &ds);
    ts.sort();
    ts.dedup();
    ts.iter().filter(|&e| *e != it).cloned().collect()
}

pub fn next_n(wv: &Weave, it: &[EntityId]) -> Vec<EntityId> {
    let mut c = it.iter().flat_map(|e| next(wv, *e)).collect::<Vec<_>>();;
    c.sort();
    c.dedup();
    c
}

//  arrows_out
//         to_tgt
//                to_tgt
// s ======> m----> o
pub fn down(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    to_tgt(wv, &to_tgt(wv, &arrows_out(wv, &[it])))
}

pub fn down_n(wv: &Weave, its: &[EntityId]) -> Vec<EntityId> {
    let mut c = its.iter().flat_map(|i| down(wv, *i)).collect::<Vec<_>>();
    c.sort();
    c.dedup();
    c
}

// to_src
//  arrows_in
//         to_src
//                marks
// s ======> m----> o
pub fn up(wv: &Weave, it: EntityId) -> Vec<EntityId> {
    to_src(wv, &arrows_in(wv, &to_src(wv, &marks(wv, &[it]))))
}

pub fn up_n(wv: &Weave, its: &[EntityId]) -> Vec<EntityId> {
    let mut c = its.iter().flat_map(|i| up(wv, *i)).collect::<Vec<_>>();
    c.sort();
    c.dedup();
    c
}