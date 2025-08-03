
#[cfg(test)]
mod tests {
    use crate::core::Weave;

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
}