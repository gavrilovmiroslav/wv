use crate::core::{EntityId, Weave};
use crate::ds::set::{is_set, set_new};
use crate::shape::{hoist, hoist_one};
use crate::traverse::{arrows_in, down, down_half, to_tgt, up_half};

pub fn map_new(wv: &mut Weave) -> EntityId {
    let keys = set_new(wv);
    let values = set_new(wv);

    let map = wv.new_arrow(keys, values);
    assert!(is_map(wv, map));

    map
}

fn is_map(wv: &Weave, map: EntityId) -> bool {
    if wv.is_arrow(map) {
        is_set(wv, wv.src(map)) && is_set(wv, wv.tgt(map))
    } else {
        false
    }
}

pub fn map_add(wv: &mut Weave, map: EntityId, key: EntityId, values: &[EntityId]) {
    let (ks, vs) = (wv.src(map), wv.tgt(map));
    let key_entry = hoist_one(wv, ks, key);
    let values_rep = wv.new_tether(vs);
    hoist(wv, values_rep, values);
    let val_entry = hoist_one(wv, key, values_rep);
    wv.new_arrow(key_entry, val_entry);
}

pub fn map_get(wv: &mut Weave, map: EntityId, key: EntityId) -> Vec<EntityId> {
    let (ks, vs) = (wv.src(map), wv.tgt(map));
    if let Some(arr) = down_half(wv, key) {
        if let Some(kv_mapping) = arrows_in(wv, &[arr]).first() {
            let key_entry = wv.src(*kv_mapping);
            let val_entry = wv.tgt(*kv_mapping);
            if let Some(ks_candidate) = up_half(wv, key_entry) {
                if ks_candidate == ks {
                    if let Some(val_rep) = to_tgt(wv, &to_tgt(wv, &[val_entry])).first() {
                        return down(wv, *val_rep);
                    }
                }
            }
        }
    }

    vec![]
}