use std::collections::HashMap;
use crate::core::{DatatypeId, EntityId, Weave};
use crate::shape::hoist;
use crate::traverse::{down, next_n, tethers, virtuals};

fn serialize_entity(wv: &Weave, id: EntityId, memory: &mut Vec<u8>) {
    memory.extend((id as u64).to_ne_bytes());
    memory.extend((wv.src(id) as u64).to_ne_bytes());
    memory.extend((wv.tgt(id) as u64).to_ne_bytes());

    let archetype = wv.get_archetype(id);
    memory.extend((archetype.len() as u64).to_ne_bytes());

    for datatype in &archetype {
        let type_name = wv.type_names.get(datatype).unwrap();
        if let Some(values) = wv.data.get(datatype) {
            if let Some(val) = values.get(&id) {
                let name_as_bytes = type_name.as_bytes();
                memory.extend((name_as_bytes.len() as u64).to_ne_bytes());
                memory.extend(name_as_bytes);
                memory.extend(datatype.to_ne_bytes());
                memory.extend((val.len() as u64).to_ne_bytes());
                memory.extend(val);
            }
        }
    }
}

pub fn serialize(wv: &Weave, hoisted_env: EntityId) -> Vec<u8> {
    let ignore_hoist_marks = next_n(wv, &next_n(wv, &tethers(wv, &[hoisted_env])));
    let env = down(wv, hoisted_env);

    let mut next_up = env.iter()
        .filter(|&n| ignore_hoist_marks.contains(n))
        .cloned().collect::<Vec<_>>();
    let mut memory = vec![];

    while let Some(n) = next_up.pop() {
        serialize_entity(wv, n, &mut memory);
        for id in virtuals(wv, &env) {
            if ignore_hoist_marks.contains(&id) { continue; }
            next_up.push(id);
        }
    }

    memory
}

fn deserialize_entity(wv: &mut Weave, memory: &[u8], index: &mut usize, mapping: &mut HashMap<EntityId, EntityId>) {
    fn get_u64(memory: &[u8], index: &mut usize) -> u64 {
        let bytes: &[u8; 8] = memory[*index..(*index + 8)].try_into().unwrap();
        *index += 8;
        u64::from_ne_bytes(*bytes)
    }

    let id = get_u64(memory, index) as EntityId;
    let src = get_u64(memory, index) as EntityId;
    let tgt = get_u64(memory, index) as EntityId;

    if !mapping.contains_key(&id) {
        mapping.insert(id, wv.new_knot());
    }

    if !mapping.contains_key(&src) {
        mapping.insert(src, wv.new_knot());
    }

    if !mapping.contains_key(&tgt) {
        mapping.insert(tgt, wv.new_knot());
    }

    let eid = *mapping.get(&id).unwrap();
    wv.change_ends(eid, *mapping.get(&src).unwrap(), *mapping.get(&tgt).unwrap());

    let archetype_len = get_u64(memory, index) as usize;
    for _ in 0..archetype_len {
        let name_len = get_u64(memory, index) as usize;
        let name_mem = memory[*index..(*index + name_len)].to_vec();
        let name = std::str::from_utf8(&name_mem).unwrap();
        *index += name_len;
        let datatype_id = get_u64(memory, index) as DatatypeId;
        let val_len = get_u64(memory, index) as usize;
        let val_mem = memory[*index..(*index + val_len)].to_vec();
        let val = std::str::from_utf8(&val_mem).unwrap();
        *index += val_len;

        assert_eq!(wv.get_datatype_id(name), datatype_id);
        wv.add_component_raw(eid, name, val.as_bytes());
    }
}

pub fn deserialize(wv: &mut Weave, serialized: &[u8]) -> EntityId {
    let mut mapping = HashMap::new();

    let mut i = 0;
    let len = serialized.len();

    let parent = wv.new_knot();

    while i < len {
        deserialize_entity(wv, &serialized, &mut i, &mut mapping);
    }

    hoist(wv, parent, &mapping.values().cloned()
        .filter(|&e| wv.is_knot(e) || wv.is_arrow(e)).collect::<Vec<_>>());

    parent
}