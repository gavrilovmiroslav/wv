use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{ Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use multimap::MultiMap;
use serde::{Deserialize, Serialize};

pub type EntityId = usize;
pub type DatatypeId = u64;

#[repr(C)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Datatype {
    Entity,
    Int,
    Float,
    Bool,
    String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum DataValue {
    Entity(EntityId),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DataField {
    pub name: String,
    pub datatype: Datatype,
}

pub struct Weave {
    pub(crate) available: usize,
    pub(crate) freelist: Vec<usize>,
    pub(crate) identities: Vec<usize>,
    pub(crate) sources: Vec<usize>,
    pub(crate) targets: Vec<usize>,
    pub(crate) source_ids: HashMap<usize, HashSet<usize>>,
    pub(crate) target_ids: HashMap<usize, HashSet<usize>>,
    pub(crate) type_names: HashMap<DatatypeId, String>,
    pub(crate) types: HashMap<DatatypeId, Vec<DataField>>,
    pub(crate) archetypes: MultiMap<EntityId, DatatypeId>,
    pub(crate) data: HashMap<DatatypeId, HashMap<usize, Vec<u8>>>,
}

impl Weave {

    pub(crate) fn new() -> Self {
        let mut wv = Self {
            available: 1024,
            freelist: Vec::new(),
            identities: vec![Self::NIL; 1024],
            sources: vec![Self::NIL; 1024],
            targets: vec![Self::NIL; 1024],
            source_ids: Default::default(),
            target_ids: Default::default(),
            types: Default::default(),
            type_names: Default::default(),
            archetypes: Default::default(),
            data: Default::default(),
        };

        wv.def_datatype("Identity", &[ DataField{ name: "id".to_string(), datatype: Datatype::Entity }]);
        wv.def_datatype("With", &[ DataField{ name: "name".to_string(), datatype: Datatype::String }]);
        wv.def_datatype("Without", &[ DataField{ name: "name".to_string(), datatype: Datatype::String }]);

        wv
    }

    pub(crate) const NIL: EntityId = usize::MAX;

    pub(crate) fn get_next_id(&mut self) -> EntityId {
        if let Some(value) = self.freelist.pop() {
            value
        } else {
            if self.available == 0 {
                let added = self.identities.len();
                self.identities.resize(2 * added, Self::NIL);
                self.sources.resize(2 * added, Self::NIL);
                self.targets.resize(2 * added, Self::NIL);
                self.available = added;
            }

            self.identities.len() - self.available
        }
    }

    pub(crate) fn add_source(&mut self, src: EntityId, id: EntityId) {
        self.sources[id] = src;

        if !self.source_ids.contains_key(&src) {
            self.source_ids.insert(src, Default::default());
        }

        self.source_ids.get_mut(&src).unwrap().insert(id);
    }

    pub(crate) fn add_target(&mut self, tgt: EntityId, id: EntityId) {
        self.targets[id] = tgt;

        if !self.target_ids.contains_key(&tgt) {
            self.target_ids.insert(tgt, Default::default());
        }

        self.target_ids.get_mut(&tgt).unwrap().insert(id);
    }

    pub(crate) fn remove_source(&mut self, src: EntityId, id: EntityId) {
        self.sources[id] = Self::NIL;
        if self.source_ids.contains_key(&src) {
            self.source_ids.get_mut(&src).unwrap().remove(&id);
        }
    }

    pub(crate) fn remove_target(&mut self, tgt: EntityId, id: EntityId) {
        self.targets[id] = Self::NIL;
        if self.target_ids.contains_key(&tgt) {
            self.target_ids.get_mut(&tgt).unwrap().remove(&id);
        }
    }

    pub(crate) fn get_external_dependents(&self, id: EntityId) -> Vec<EntityId> {
        let entities = self.get_dependents(id);
        entities.iter().filter(|&i| *i != id).cloned().collect()
    }

    pub(crate) fn get_dependents(&self, id: EntityId) -> Vec<EntityId> {
        let mut entities = self.get_dependents_for_source(id);
        entities.extend_from_slice(&self.get_dependents_for_target(id));
        entities.sort();
        entities.dedup();
        entities
    }

    pub(crate) fn get_dependents_for_source(&self, src: EntityId) -> Vec<EntityId> {
        let mut v = self.source_ids.get(&src).unwrap_or(&HashSet::new()).clone().into_iter().collect::<Vec<_>>();
        v.sort();
        v
    }

    pub(crate) fn get_dependents_for_target(&self, tgt: EntityId) -> Vec<EntityId> {
        let mut v = self.target_ids.get(&tgt).unwrap_or(&HashSet::new()).clone().into_iter().collect::<Vec<_>>();
        v.sort();
        v
    }

    #[allow(dead_code)]
    pub(crate) fn any_free_entities(&self) -> bool {
        !self.freelist.is_empty()
    }

    pub fn new_knot(&mut self) -> EntityId {
        let id = self.get_next_id();
        assert_eq!(self.identities[id], Self::NIL);
        self.identities[id] = id;
        self.add_source(id, id);
        self.add_target(id, id);

        self.available -= 1;
        id
    }

    pub fn new_arrow(&mut self, src: EntityId, tgt: EntityId) -> EntityId {
        assert_eq!(self.identities[src], src);
        assert_eq!(self.identities[tgt], tgt);

        let id = self.get_next_id();
        assert_eq!(self.identities[id], Self::NIL);

        self.identities[id] = id;
        self.add_source(src, id);
        self.add_target(tgt, id);

        self.available -= 1;
        id
    }

    pub fn new_tether(&mut self, src: EntityId) -> EntityId {
        assert_eq!(self.identities[src], src);

        let id = self.get_next_id();
        assert_eq!(self.identities[id], Self::NIL);

        self.identities[id] = id;
        self.add_source(src, id);
        self.add_target(id, id);
        self.available -= 1;
        id
    }

    pub fn new_mark(&mut self, tgt: EntityId) -> EntityId {
        assert_eq!(self.identities[tgt], tgt);

        let id = self.get_next_id();
        assert_eq!(self.identities[id], Self::NIL);

        self.identities[id] = id;
        self.add_source(id, id);
        self.add_target(tgt, id);

        self.available -= 1;
        id
    }

    pub fn src(&self, id: EntityId) -> EntityId {
        assert_eq!(self.identities[id], id);
        self.sources[id]
    }

    pub fn tgt(&self, id: EntityId) -> EntityId {
        assert_eq!(self.identities[id], id);
        self.targets[id]
    }

    pub fn change_src(&mut self, id: EntityId, src: EntityId) {
        assert_eq!(self.identities[id], id);

        let old_source = self.sources[id];
        self.remove_source(old_source, id);
        self.add_source(src, id);
    }

    pub fn change_tgt(&mut self, id: EntityId, tgt: EntityId) {
        assert_eq!(self.identities[id], id);

        let old_target = self.targets[id];
        self.remove_target(old_target, id);
        self.add_target(tgt, id);
    }

    pub fn change_ends(&mut self, id: EntityId, src: EntityId, tgt: EntityId) {
        self.change_src(id, src);
        self.change_tgt(id, tgt);
    }

    pub fn is_knot(&self, id: EntityId) -> bool {
        self.src(id) == id && self.tgt(id) == id
    }

    pub fn is_arrow(&self, id: EntityId) -> bool {
        self.src(id) != id && self.tgt(id) != id
    }

    pub fn is_mark(&self, id: EntityId) -> bool {
        self.src(id) == id && self.tgt(id) != id
    }

    pub fn is_tether(&self, id: EntityId) -> bool {
        self.src(id) != id && self.tgt(id) == id
    }

    pub fn delete_orphan(&mut self, id: EntityId) {
        enum OrphanKind {
            Src(usize), Tgt(usize),
        }

        let mut unfinished = VecDeque::new();

        if self.identities[id] != id {
            return;
        }

        self.identities[id] = Self::NIL;
        self.freelist.push(id);

        if let Some(sources) = self.source_ids.get(&id) {
            for src in sources {
                if *src != id {
                    unfinished.push_back(OrphanKind::Src(*src));
                }
            }
        }

        if let Some(targets) = self.target_ids.get(&id) {
            for tgt in targets {
                if *tgt != id {
                    unfinished.push_back(OrphanKind::Tgt(*tgt));
                }
            }
        }

        self.source_ids.remove(&id);
        self.target_ids.remove(&id);

        while let Some(next) = unfinished.pop_front() {
            match next {
                OrphanKind::Src(src) => self.change_src(src, src),
                OrphanKind::Tgt(tgt) => self.change_tgt(tgt, tgt),
            }
        }
    }

    pub fn delete_cascade(&mut self, id: EntityId) {
        let mut unfinished = VecDeque::new();
        unfinished.push_back(id);

        while let Some(next) = unfinished.pop_front() {
            if self.identities[next] != next {
                continue;
            }

            self.identities[next] = Self::NIL;
            self.freelist.push(next);

            if let Some(sources) = self.source_ids.get(&next) {
                for src in sources {
                    unfinished.push_back(*src);
                }
            }

            if let Some(targets) = self.target_ids.get(&next) {
                for tgt in targets {
                    unfinished.push_back(*tgt);
                }
            }

            self.source_ids.remove(&next);
            self.target_ids.remove(&next);
        }
    }

    pub fn is_valid(&self, id: EntityId) -> bool {
        self.identities[id] == id
    }

    pub fn is_nil(&self, id: EntityId) -> bool {
        id == Self::NIL
    }

    fn get_type_id(name: &str) -> DatatypeId {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish()
    }

    pub fn def_datatype(&mut self, name: &str, datatype: &[DataField]) -> DatatypeId {
        let id = Self::get_type_id(name);
        self.types.entry(id).or_insert(datatype.to_vec());
        self.type_names.entry(id).or_insert(name.to_string());

        id
    }

    pub fn get_datatype_id(&self, name: &str) -> DatatypeId {
        let id = Self::get_type_id(name);
        if self.types.contains_key(&id) {
            id
        } else {
            Self::NIL as u64
        }
    }

    pub fn get_datatype_field_count(&self, name: &str) -> usize {
        let id = Self::get_type_id(name);
        if self.types.contains_key(&id) {
            self.types[&id].len()
        } else {
            0
        }
    }

    pub fn get_datatype_field(&self, name: &str, index: usize) -> DataField {
        let id = Self::get_type_id(name);
        assert!(self.types.contains_key(&id));
        assert!(index < self.types[&id].len());
        self.types[&id][index].clone()
    }

    pub(crate) fn add_component_raw(&mut self, entity: EntityId, name: &str, dat: &[u8]) {
        let id = Self::get_type_id(name);

        if !self.data.contains_key(&id) {
            self.data.insert(id, Default::default());
            self.archetypes.insert(entity, id);
        }

        self.data.get_mut(&id).unwrap().entry(entity)
            .or_insert(dat.to_vec());
    }

    pub fn add_component(&mut self, entity: EntityId, name: &str, fields: &[DataValue]) {
        self.add_component_raw(entity, name, serde_json::to_string(&fields)
            .expect("Fields can't stringify").as_bytes());
    }

    pub fn has_component(&self, entity: EntityId, name: &str) -> bool {
        let id = Self::get_type_id(name);
        if let Some(attachments) = self.data.get(&id) {
            attachments.contains_key(&entity)
        } else {
            false
        }
    }

    pub fn get_component(&self, entity: EntityId, name: &str) -> Vec<DataValue> {
        let id = Self::get_type_id(name);
        if let Some(attachments) = self.data.get(&id) {
            let v= attachments.get(&entity).unwrap();
            let s = std::str::from_utf8(v).unwrap().to_owned();
            serde_json::from_str(&s).unwrap()
        } else {
            vec![]
        }
    }

    pub fn remove_component(&mut self, entity: EntityId, name: &str) {
        let id = Self::get_type_id(name);
        if let Some(attachments) = self.data.get_mut(&id) {
            attachments.remove(&entity);
            if let Some(archetypes) = self.archetypes.get_vec_mut(&entity) {
                if let Some(index) = archetypes.iter().position(|e| *e == id) {
                    archetypes.remove(index);
                }
            }
        }
    }

    pub(crate) fn get_archetype(&self, entity: EntityId) -> Vec<DatatypeId> {
        if let Some(archetypes) = self.archetypes.get_vec(&entity) {
            archetypes.clone()
        } else {
            vec![]
        }
    }
}
