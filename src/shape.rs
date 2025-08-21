use crate::core::{DataValue, EntityId, Weave};
use crate::traverse::{marks, primary};

/*
    Add a component directly onto an entity
 */
pub fn markup(wv: &mut Weave, target: EntityId, name: &str, fields: &[DataValue]) {
    wv.add_component(target, name, fields);
}

/*
    Add a mark onto a target and mark it up
 */
pub fn annotate(wv: &mut Weave, target: EntityId, name: &str, fields: &[DataValue]) -> EntityId {
    let am = wv.new_mark(target);
    markup(wv, am, name, fields);
    am
}

pub fn get_annotation(wv: &Weave, target: EntityId, name: &str) -> Option<EntityId> {
    for mark in marks(wv, &[target]) {
        if wv.has_component(mark, name) {
            return Some(mark);
        }
    }

    None
}

/*
      Motif kind before  |    Motif kind after
    ---------------------+----------------------
      Knot   a = (a, a)  |    Tether a = (p, a)
      Mark   a = (a, b)  |    Arrow  a = (p, b)
      Arrow  a = (b, c)  |    Arrow  a = (p, c)
      Tether a = (b, a)  |    Tether a = (p, a)
*/
pub fn parent(wv: &mut Weave, root: EntityId, children: &[EntityId]) {
    for child in children {
        wv.change_src(*child, root);
    }
}

/*
      Motif kind before  |    Motif kind after
    ---------------------+----------------------
      Knot   a = (a, a)  |    Mark   a = (a, f)
      Mark   a = (a, b)  |    Mark   a = (a, f)
      Arrow  a = (b, c)  |    Arrow  a = (b, f)
      Tether a = (b, a)  |    Arrow  a = (b, f)
*/
pub fn pivot(wv: &mut Weave, center: EntityId, children: &[EntityId]) {
    for observer in children {
        wv.change_tgt(*observer, center);
    }
}

/*
      from                s                t
      --------------------------------------
      to                  s =============> t
 */
pub fn connect(wv: &mut Weave, source: EntityId, targets: &[EntityId]) {
    for target in targets {
        wv.new_arrow(source, *target);
    }
}

/*
      from                s                  o
      ----------------------------------------
      to                  s ---> t ==> m---> o
 */
pub fn hoist(wv: &mut Weave, subject: EntityId, objects: &[EntityId]) {
    for object in primary(wv, objects) {
        let anchor = wv.new_tether(subject);
        let guide = wv.new_mark(object);
        wv.new_arrow(anchor, guide);
    }
}

/*
      from                s ------a----> o
                   S(a) = s ------a----> T(a) = o
                        S(a) --a)    T(a) = o <---(m
                        S(a) --a)    (m --> o
                        S(a) --a) => (m --> o
      --------------------------------------
      to                  s --> t ==> m--> o
 */
pub fn lift(wv: &mut Weave, arrows: &[EntityId]) {
    for a in arrows {
        assert!(wv.is_arrow(*a));
        let tgt = wv.tgt(*a);
        wv.change_tgt(*a, *a);
        assert!(wv.is_tether(*a));
        let guide = wv.new_mark(tgt);
        wv.new_arrow(*a, guide);
    }
}

/*
      from                s --a) =A=> (g-> o
                          s --a)      (g-> o
                          \                ^
                           --------A------/

                          s x-a)      (g-x o
      --------------------------------------
      to                  s =======A=====> o
 */
pub fn lower(wv: &mut Weave, arrows: &[EntityId]) {
    for arrow in arrows {
        let anchor = wv.src(*arrow);
        let guide = wv.tgt(*arrow);
        assert!(wv.is_tether(anchor));
        assert!(wv.is_mark(guide));
        let source = wv.src(anchor);
        let target = wv.tgt(guide);
        wv.change_ends(*arrow, source, target);
        wv.delete_cascade(anchor);
        wv.delete_cascade(guide);
    }
}