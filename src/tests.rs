
#[cfg(test)]
mod tests {
    use crate::core::{DataValue, Weave};
    use crate::traverse::{arrows_out, down, down_n, marks, next, prev, tethers, to_tgt, up, up_n};
    use crate::search::{find_all, find_one};
    use crate::shape::{annotate, hoist, markup};

    #[test]
    fn delete_becomes_nil() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();
        assert_ne!(Weave::NIL, a);
        w.delete_cascade(a);
        assert!(!w.is_valid(a));
    }

    #[test]
    fn delete_reuses_entities() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();
        w.delete_cascade(a);
        let b = w.new_knot();
        let c = w.new_knot();
        w.delete_cascade(c);
        let d = w.new_arrow(b, b);
        assert_eq!(a, b);
        assert_eq!(c, d);
    }

    #[test]
    fn delete_makes_things_invalid() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();
        let b = w.new_knot();
        let c = w.new_arrow(a, b);
        let d = w.new_mark(c);
        let e = w.new_tether(d);
        assert!(w.is_valid(a));
        assert!(w.is_valid(b));
        assert!(w.is_valid(c));
        assert!(w.is_valid(d));
        assert!(w.is_valid(e));
        w.delete_cascade(b);
        assert!(w.is_valid(a));
        assert!(!w.is_valid(b));
        assert!(!w.is_valid(c));
        assert!(!w.is_valid(d));
        assert!(!w.is_valid(e));
        assert!(w.any_free_entities());
        assert_eq!(w.freelist.len(), 4);
    }

    #[test]
    fn changing_endpoints_caches_dependents() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();
        let b = w.new_knot();
        let c = w.new_knot();
        assert_eq!(w.src(c), c);
        assert_eq!(w.tgt(c), c);
        assert_eq!(w.get_dependents_for_source(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_target(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_source(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_target(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_source(c), vec![ 2 ]);
        assert_eq!(w.get_dependents_for_target(c), vec![ 2 ]);
        w.change_ends(c, a, b);
        assert_eq!(w.src(c), a);
        assert_eq!(w.tgt(c), b);
        assert_eq!(w.get_dependents_for_source(a), vec![ 0, 2 ]);
        assert_eq!(w.get_dependents_for_target(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_source(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_target(b), vec![ 1, 2 ]);
        assert!(w.get_dependents_for_source(c).is_empty());
        assert!(w.get_dependents_for_target(c).is_empty());
        w.change_ends(c, b, a);
        assert_eq!(w.src(c), b);
        assert_eq!(w.tgt(c), a);
        assert_eq!(w.get_dependents_for_source(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_target(a), vec![ 0, 2 ]);
        assert_eq!(w.get_dependents_for_source(b), vec![ 1, 2 ]);
        assert_eq!(w.get_dependents_for_target(b), vec![ 1 ]);
        assert!(w.get_dependents_for_source(c).is_empty());
        assert!(w.get_dependents_for_target(c).is_empty());
        w.change_ends(c, c, c);
        assert_eq!(w.src(c), c);
        assert_eq!(w.tgt(c), c);
        assert_eq!(w.get_dependents_for_source(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_target(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_source(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_target(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_source(c), vec![ 2 ]);
        assert_eq!(w.get_dependents_for_target(c), vec![ 2 ]);
    }

    #[test]
    fn orphaning_by_deletion() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();
        let b = w.new_knot();
        let c = w.new_arrow(a, b);
        assert_eq!(w.get_dependents_for_source(a), vec![ 0, 2 ]);
        assert_eq!(w.get_dependents_for_source(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_target(a), vec![ 0 ]);
        assert_eq!(w.get_dependents_for_target(b), vec![ 1, 2 ]);
        w.delete_orphan(a);
        assert!(!w.is_valid(a));
        assert!(w.is_valid(b));
        assert!(w.is_valid(c));
        assert_eq!(w.src(c), c);
        assert_eq!(w.tgt(c), b);
        assert_eq!(w.get_dependents_for_source(b), vec![ 1 ]);
        assert_eq!(w.get_dependents_for_target(b), vec![ 1, 2 ]);
    }

    #[test]
    fn quick_searching_by_dependents() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();
        let b = w.new_knot();
        let c = w.new_knot();
        let d = w.new_knot();
        let _ = w.new_arrow(a, b);
        let _ = w.new_arrow(a, c);
        let _ = w.new_arrow(a, d);
        assert_eq!(w.get_dependents_for_source(a), vec![ 0, 4, 5, 6 ]);

        let mut arr = arrows_out(&w, &[a]);
        arr.sort();
        assert_eq!(arr, vec![ 4, 5, 6 ]);

        let mut targets = to_tgt(&w, &arr);
        targets.sort();
        assert_eq!(targets, vec![ 1, 2, 3 ]);
    }

    #[test]
    fn marks_tethers_and_moves() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();       // 0 = 0 -> 0
        let x = w.new_knot();       // 1 = 1 -> 1
        let y = w.new_knot();       // 2 = 2 -> 2
        let b = w.new_tether(a);    // 3 = 0 -> 3
        let c = w.new_tether(a);    // 4 = 0 -> 4
        let _ = w.new_arrow(a, x);  // 5 = 0 -> 1
        let _ = w.new_arrow(y, a);  // 6 = 2 -> 0
        let d = w.new_mark(a);      // 7 = 7 -> 0
        let e = w.new_mark(a);      // 8 = 8 -> 0

        let mut m = marks(&w, &[a]);
        m.sort();
        assert_eq!(m, vec![ d, e ]);

        let mut t = tethers(&w, &[a]);
        t.sort();
        assert_eq!(t, vec![ b, c ]);

        let mut n = next(&w, a);
        n.sort();
        assert_eq!(n, vec![ x, b, c ]);

        let mut p = prev(&w, a);
        p.sort();
        assert_eq!(p, vec![ y, d, e ]);
    }

    #[test]
    fn down_and_up_the_hoist() {
        let mut w: Weave = Weave::new();
        let a = w.new_knot();       // 0 = 0 -> 0
        let x = w.new_knot();       // 1 = 1 -> 1
        let y = w.new_knot();       // 2 = 2 -> 2
        // hoist 1: a ===> h1 ---> x        // 3 = 3 -> 1
                                            // 4 = 0 -> 3
        // hoist 2: a ===> h1 ---> x        // 5 = 5 -> 1
                                            // 6 = 0 -> 5
        hoist(&mut w, a, &[x, y]);
        let mut d = down(&w, a);
        d.sort();
        assert_eq!(d, vec![ x, y ]);

        let mut u = up(&w, x);
        u.sort();
        assert_eq!(u, vec![ a ]);

        let mut dn = down_n(&w, &[ a ]);
        dn.sort();
        assert_eq!(dn, vec![ x, y ]);

        let mut un = up_n(&w, &[ x, y ]);
        un.sort();
        assert_eq!(un, vec![ a ]);
    }

    #[test]
    fn test_pattern_match() {
        let mut w: Weave = Weave::new();
        // define pattern
        let a = w.new_knot();
        annotate(&mut w, a, "With", &[ DataValue::String("With".to_string()) ]);

        let b = w.new_knot();
        let c = w.new_knot();
        w.new_arrow(a, b);
        w.new_arrow(a, c);
        w.new_arrow(b, c);
        let p = w.new_knot();
        hoist(&mut w, p, &[ a, b, c ]);

        // define target
        let d = w.new_knot();
        markup(&mut w, d, "With", &[ DataValue::String("With".to_string()) ]);
        let e = w.new_knot();
        let f = w.new_knot();
        let g = w.new_knot();
        w.new_arrow(d, e);
        w.new_arrow(d, f);
        w.new_arrow(e, f);
        w.new_arrow(f, e);
        w.new_arrow(g, e);
        w.new_arrow(g, d);
        let t = w.new_knot();
        hoist(&mut w, t, &[d, e, f, g ]);

        // pattern match
        use std::time::Instant;
        let now = Instant::now();
        let matching = find_all(&w, p, t);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        assert_eq!(matching.len(), 2);
        println!("{:?}", matching);
    }

    #[test]
    fn test_pattern_match_one() {
        let mut w: Weave = Weave::new();
        // define pattern
        let a = w.new_knot();
        annotate(&mut w, a, "With", &[ DataValue::String("With".to_string()) ]);

        let b = w.new_knot();
        let c = w.new_knot();
        w.new_arrow(a, b);
        w.new_arrow(a, c);
        w.new_arrow(b, c);
        let p = w.new_knot();
        hoist(&mut w, p, &[ a, b, c ]);

        // define target
        let d = w.new_knot();
        markup(&mut w, d, "With", &[ DataValue::String("With".to_string()) ]);
        let e = w.new_knot();
        let f = w.new_knot();
        let g = w.new_knot();
        w.new_arrow(d, e);
        w.new_arrow(d, f);
        w.new_arrow(e, f);
        w.new_arrow(f, e);
        w.new_arrow(g, e);
        w.new_arrow(g, d);
        let t = w.new_knot();
        hoist(&mut w, t, &[d, e, f, g ]);

        // pattern match
        use std::time::Instant;
        let now = Instant::now();
        let matching = find_one(&w, p, t);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        println!("{:?}", matching);
    }
}