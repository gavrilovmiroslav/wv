use crate::core::{EntityId, Weave};
use crate::traverse::{arrows_out, hop};

pub fn fsm_new(wv: &mut Weave, hoisted_graph: EntityId, start: EntityId) -> EntityId {
    assert_ne!(hoisted_graph, start);
    assert_eq!(wv.src(start), hoisted_graph);

    wv.change_src(hoisted_graph, start);
    wv.change_tgt(hoisted_graph, start);
    hoisted_graph
}

fn is_fsm(wv: &Weave, fsm: EntityId) -> bool {
    let start = wv.src(fsm);
    let end = wv.tgt(fsm);
    wv.is_arrow(fsm) && wv.src(start) == fsm && wv.src(end) == fsm
}

pub fn fsm_start(wv: &mut Weave, fsm: EntityId) -> EntityId {
    assert!(is_fsm(wv, fsm));
    wv.src(fsm)
}

pub fn fsm_current(wv: &mut Weave, fsm: EntityId) -> EntityId {
    assert!(is_fsm(wv, fsm));
    wv.tgt(fsm)
}

pub fn fsm_transition(wv: &mut Weave, fsm: EntityId, trigger: String) -> Option<(EntityId, String, EntityId)> {
    assert!(is_fsm(wv, fsm));
    let current = fsm_current(wv, fsm);
    let candidates = arrows_out(wv, &[current]).iter()
        .filter(|&a| wv.has_component(*a, &trigger)).cloned().collect::<Vec<_>>();

    if candidates.is_empty() {
        None
    } else if candidates.len() > 1 {
        panic!("FSM has more than one available transition for trigger {} at #{}", trigger, current);
    } else {
        let transition = *candidates.first().unwrap();
        let new_current = hop(wv, transition);
        assert_eq!(wv.src(new_current), fsm);

        wv.change_tgt(fsm, new_current);
        Some((current, trigger, new_current))
    }
}