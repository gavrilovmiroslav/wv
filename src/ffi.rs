use std::ffi::{c_char, c_void, CStr, CString};
use std::slice;
use crate::core::{DataField, DataValue, Datatype, EntityId, Weave};
use crate::io;
use crate::search::{find_all, find_one};
use crate::traverse::{arrows, arrows_in, arrows_out, external_deps, down, down_n, marks, next, next_n, prev, prev_n, tethers, to_src, to_tgt, up, up_n};
use crate::shape::{connect, hoist, lift, lower, parent, pivot};

#[repr(C)]
pub struct WvDataField {
    name: *const c_char,
    datatype: Datatype,
}

impl WvDataField {
    pub fn parse(value: DataField) -> Self {
        WvDataField {
            name: CString::new(value.name).unwrap().into_raw(),
            datatype: value.datatype,
        }
    }
}

impl Into<DataField> for WvDataField {
    fn into(self) -> DataField {
        let cstr = unsafe { CStr::from_ptr(self.name) }.to_str().expect("CString to_str failed");
        DataField {
            name: cstr.to_string(),
            datatype: self.datatype,
        }
    }
}

#[no_mangle]
pub static NIL: usize = Weave::NIL;

#[no_mangle]
pub extern "C" fn wv_new_weave() -> *mut Weave {
    Box::into_raw(Box::new(Weave::new()))
}

#[no_mangle]
extern "C" fn wv_free_weave(weave: *mut Weave) {
    drop(unsafe { Box::from_raw(weave) });
}

#[no_mangle]
extern "C" fn wv_new_knot(wv: &mut Weave) -> usize {
    (&mut *wv).new_knot()
}

#[no_mangle]
extern "C" fn wv_new_arrow(wv: &mut Weave, src: usize, tgt: usize) -> usize {
    (&mut *wv).new_arrow(src, tgt)
}

#[no_mangle]
extern "C" fn wv_new_mark(wv: &mut Weave, tgt: usize) -> usize {
    (&mut *wv).new_mark(tgt)
}

#[no_mangle]
extern "C" fn wv_new_tether(wv: &mut Weave, src: usize) -> usize {
    (&mut *wv).new_tether(src)
}

#[no_mangle]
extern "C" fn wv_src(wv: &Weave, id: usize) -> usize {
    (&*wv).src(id)
}

#[no_mangle]
extern "C" fn wv_tgt(wv: &Weave, id: usize) -> usize {
    (&*wv).tgt(id)
}

#[no_mangle]
extern "C" fn wv_change_src(wv: &mut Weave, id: usize, src: usize) {
    (&mut *wv).change_src(id, src)
}

#[no_mangle]
extern "C" fn wv_change_tgt(wv: &mut Weave, id: usize, tgt: usize) {
    (&mut *wv).change_tgt(id, tgt)
}

#[no_mangle]
extern "C" fn wv_change_ends(wv: &mut Weave, id: usize, src: usize, tgt: usize) {
    (&mut *wv).change_ends(id, src, tgt)
}

#[no_mangle]
extern "C" fn wv_is_knot(wv: &Weave, id: usize) -> bool {
    (&*wv).is_knot(id)
}

#[no_mangle]
extern "C" fn wv_is_arrow(wv: &Weave, id: usize) -> bool {
    (&*wv).is_arrow(id)
}

#[no_mangle]
extern "C" fn wv_is_mark(wv: &Weave, id: usize) -> bool {
    (&*wv).is_mark(id)
}

#[no_mangle]
extern "C" fn wv_is_tether(wv: &Weave, id: usize) -> bool {
    (&*wv).is_tether(id)
}

#[no_mangle]
extern "C" fn wv_is_valid(wv: &Weave, id: usize) -> bool {
    (&*wv).is_valid(id)
}

#[no_mangle]
extern "C" fn wv_is_nil(wv: &Weave, id: usize) -> bool {
    (&*wv).is_nil(id)
}

#[no_mangle]
extern "C" fn wv_delete_cascade(wv: &mut Weave, id: &mut usize) {
    (&mut *wv).delete_cascade(*id);
    *id = NIL;
}

#[no_mangle]
extern "C" fn wv_delete_orphan(wv: &mut Weave, id: &mut usize) {
    (&mut *wv).delete_orphan(*id);
    *id = NIL;
}

#[no_mangle]
extern "C" fn wv_def_data(wv: &mut Weave, name: *const c_char, datatype: *const WvDataField, len: usize) -> u64 {
    let fields: Vec<DataField> = unsafe { slice::from_raw_parts(datatype, len) }
        .iter().map(|v| {
        let cstr = unsafe { CStr::from_ptr(v.name.clone()) }.to_str().expect("CString to_str failed");
        DataField {
            name: cstr.to_string(),
            datatype: v.datatype.clone(),
        }
    }).collect();

    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    (&mut *wv).def_datatype(cstr, fields.as_slice())
}

#[no_mangle]
extern "C" fn wv_get_data_id(wv: &Weave, name: *const c_char) -> u64 {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    (&*wv).get_datatype_id(cstr)
}

#[no_mangle]
extern "C" fn wv_get_data_field_count(wv: &Weave, name: *const c_char) -> usize {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    (&*wv).get_datatype_field_count(cstr)
}

#[no_mangle]
extern "C" fn wv_get_data_field(wv: &Weave, name: *const c_char, index: usize) -> WvDataField {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    WvDataField::parse((&*wv).get_datatype_field(cstr, index))
}

#[no_mangle]
extern "C" fn wv_add_component(wv: &mut Weave, entity: usize, name: *const c_char, fields: *const *const c_void) {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    let count = wv.get_datatype_field_count(cstr);
    let fields = unsafe { slice::from_raw_parts(fields, count) };
    let mut values = vec![];
    for i in 0..count {
        let df = wv.get_datatype_field(cstr, i);
        let value = match df.datatype {
            Datatype::Entity => DataValue::Entity(unsafe { *(fields[i] as *const usize).as_ref().unwrap() }),
            Datatype::Int => DataValue::Int(unsafe { *(fields[i] as *const i64).as_ref().unwrap() }),
            Datatype::Float => DataValue::Float(unsafe { *(fields[i] as *const f64).as_ref().unwrap() }),
            Datatype::Bool => DataValue::Bool(unsafe { *(fields[i] as *const bool).as_ref().unwrap() }),
            Datatype::String => {
                let v = unsafe { CStr::from_ptr(fields[i] as *const c_char) }.to_str().expect("CString to_str failed");
                DataValue::String(v.to_string())
            }
        };
        values.push(value.clone());
    }
    (&mut *wv).add_component(entity, cstr, &values);
}

#[no_mangle]
extern "C" fn wv_has_component(wv: &Weave, entity: usize, name: *const c_char) -> bool {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    (&*wv).has_component(entity, cstr)
}

#[no_mangle]
extern "C" fn wv_get_component_field(wv: &Weave, entity: usize, name: *const c_char, index: usize) -> *const c_void {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    let v = (&*wv).get_component(entity, cstr);
    match &v[index] {
        DataValue::Entity(e) => e as *const _ as *const c_void,
        DataValue::Int(i) => i as *const _ as *const c_void,
        DataValue::Float(f) => f as *const _ as *const c_void,
        DataValue::Bool(b) => b as *const _ as *const c_void,
        DataValue::String(s) => CString::new(s.clone()).unwrap().into_raw() as *const c_void,
    }
}

#[no_mangle]
extern "C" fn wv_remove_component(wv: &mut Weave, entity: usize, name: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(name) }.to_str().expect("CString to_str failed");
    (&mut *wv).remove_component(entity, cstr);
}



#[no_mangle]
extern "C" fn wv_shape__parent(wv: &mut Weave, root: usize, len: usize, children: *const usize) {
    let children: &[usize] = unsafe { slice::from_raw_parts(children, len) };
    parent(&mut *wv, root, children);
}

#[no_mangle]
extern "C" fn wv_shape__pivot(wv: &mut Weave, center: usize, len: usize, children: *const usize) {
    let observers: &[usize] = unsafe { slice::from_raw_parts(children, len) };
    pivot(&mut *wv, center, observers);
}

#[no_mangle]
extern "C" fn wv_shape__connect(wv: &mut Weave, source: usize, len: usize, targets: *const usize) {
    let targets: &[usize] = unsafe { slice::from_raw_parts(targets, len) };
    connect(&mut *wv, source, targets);
}

#[no_mangle]
extern "C" fn wv_shape__hoist(wv: &mut Weave, subject: usize, len: usize, objects: *const usize) {
    let objects: &[usize] = unsafe { slice::from_raw_parts(objects, len) };
    hoist(&mut *wv, subject, objects);
}

#[no_mangle]
extern "C" fn wv_shape__lift(wv: &mut Weave, len: usize, arrows: *const usize) {
    let arrows: &[usize] = unsafe { slice::from_raw_parts(arrows, len) };
    lift(&mut *wv, arrows);
}

#[no_mangle]
extern "C" fn wv_shape__lower(wv: &mut Weave, len: usize, arrows: *const usize) {
    let arrows: &[usize] = unsafe { slice::from_raw_parts(arrows, len) };
    lower(&mut *wv, arrows);
}

#[repr(C)]
pub struct WvEntityArray {
    pub len: usize,
    pub ptr: *const usize,
}

impl Into<WvEntityArray> for Vec<usize> {
    fn into(self) -> WvEntityArray {
        WvEntityArray {
            len: self.len(),
            ptr: Box::into_raw(self.into_boxed_slice()) as *const usize,
        }
    }
}

#[repr(C)]
pub struct WvByteArray {
    pub len: usize,
    pub ptr: *const u8,
}

impl Into<WvByteArray> for Vec<u8> {
    fn into(self) -> WvByteArray {
        WvByteArray {
            len: self.len(),
            ptr: Box::into_raw(self.into_boxed_slice()) as *const u8,
        }
    }
}

#[no_mangle]
extern "C" fn wv_move__deps(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    external_deps(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__arrows(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    arrows(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__arrows_in(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    arrows_in(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__arrows_out(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    arrows_out(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__marks(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    marks(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__tethers(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    tethers(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__to_src(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    to_src(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__to_tgt(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    to_tgt(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__prev(wv: &mut Weave, it: usize) -> WvEntityArray {
    prev(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__prev_n(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    prev_n(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__next(wv: &mut Weave, it: usize) -> WvEntityArray {
    next(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__next_n(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    next_n(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__down(wv: &mut Weave, it: usize) -> WvEntityArray  {
    down(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__down_n(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    down_n(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__up(wv: &mut Weave, it: usize) -> WvEntityArray {
    up(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_move__up_n(wv: &mut Weave, len: usize, it: *const usize) -> WvEntityArray {
    let it: &[usize] = unsafe { slice::from_raw_parts(it, len) };
    up_n(&*wv, it).into()
}

#[no_mangle]
extern "C" fn wv_search__find_one(wv: &Weave, hoisted_pattern: usize, hoisted_target: usize, size: &mut usize, count: &mut usize) -> WvEntityArray {
    let result = find_one(wv, hoisted_pattern, hoisted_target);
    if let Some(hash) = result {
        *count = 1;
        *size = hash.len();
        let mut key_values = vec![];
        for (k, v) in hash {
            key_values.push(k);
            key_values.push(v);
        }
        key_values.into()
    } else {
        *count = 0;
        vec![].into()
    }
}

#[no_mangle]
extern "C" fn wv_search__find_all(wv: &Weave, hoisted_pattern: usize, hoisted_target: usize, size: &mut usize, count: &mut usize) -> WvEntityArray {
    let result = find_all(wv, hoisted_pattern, hoisted_target);
    *count = result.len();
    let mut key_values = vec![];

    for solution in result {
        *size = solution.len();
        for (k, v) in solution {
            key_values.push(k);
            key_values.push(v);
        }
    }

    key_values.into()
}

#[no_mangle]
extern "C" fn wv_serialize(wv: &mut Weave, id : usize) -> WvByteArray
{
    io::serialize(wv, id).into()
}

#[no_mangle]
extern "C" fn wv_deserialize(wv: &mut Weave, len: usize, it: *const u8) -> EntityId
{
    let it: &[u8] = unsafe { slice::from_raw_parts(it, len) };
    io::deserialize(wv, it)
}
