use crate::core::{EntityId, Weave};

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
      from                s                o
      --------------------------------------
      to                  s ======> m----> o
 */
pub fn hoist(wv: &mut Weave, subject: EntityId, objects: &[EntityId]) {
    for object in objects {
        let anchor = wv.new_mark(*object);
        wv.new_arrow(subject, anchor);
    }
}

/*
      from                s ------a----> o
                   S(a) = s ------a----> T(a) = o
                        S(a) -----a----> T(a) = o <---(m
                                      // change T(a) = m
                        S(a) -----a----> m)---> o
      --------------------------------------
      to                  s ======> m----> o
 */
pub fn lift(wv: &mut Weave, arrows: &[EntityId]) {
    for arrow in arrows {
        assert!(wv.is_arrow(*arrow));
        let tgt = wv.tgt(*arrow);
        let anchor = wv.new_mark(tgt);
        wv.change_tgt(*arrow, anchor);
    }
}

/*
      from                s ===a==> m----> o
                          s ---*a--------> o <----(m
                          s ----a--------> o <-x--(m
      --------------------------------------
      to                  s =============> o
 */
pub fn lower(wv: &mut Weave, arrows: &[EntityId]) {
    for arrow in arrows {
        let mark = wv.tgt(*arrow);
        assert!(wv.is_mark(mark));
        let tgt = wv.tgt(mark);
        wv.change_tgt(*arrow, tgt);
        wv.delete_cascade(mark);
    }
}