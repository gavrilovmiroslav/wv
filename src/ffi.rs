use std::ffi::{c_char, c_void, CStr, CString};
use std::slice;
use crate::core::{DataField, DataValue, Datatype, Weave};
use crate::shape::{connect, dip, focus, hoist, lift, parent};

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
unsafe extern "C" fn wv_free_weave(weave: *mut Weave) {
    drop(Box::from_raw(weave));
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
unsafe extern "C" fn wv_def_data(wv: &mut Weave, name: *const c_char, datatype: *const WvDataField, len: usize) -> u64 {
    let fields: Vec<DataField> = slice::from_raw_parts(datatype, len)
        .iter().map(|v| {
        let cstr = unsafe { CStr::from_ptr(v.name.clone()) }.to_str().expect("CString to_str failed");
        DataField {
            name: cstr.to_string(),
            datatype: v.datatype.clone(),
        }
    }).collect();

    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    (&mut *wv).def_datatype(cstr, fields.as_slice())
}

#[no_mangle]
unsafe extern "C" fn wv_get_data_id(wv: &Weave, name: *const c_char) -> u64 {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    (&*wv).get_datatype_id(cstr)
}

#[no_mangle]
unsafe extern "C" fn wv_get_data_field_count(wv: &Weave, name: *const c_char) -> usize {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    (&*wv).get_datatype_field_count(cstr)
}

#[no_mangle]
unsafe extern "C" fn wv_get_data_field_type(wv: &Weave, name: *const c_char, index: usize) -> WvDataField {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    WvDataField::parse((&*wv).get_datatype_field_type(cstr, index))
}

#[no_mangle]
unsafe extern "C" fn wv_add_component(wv: &mut Weave, entity: usize, name: *const c_char, fields: *const *const c_void) {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    let count = wv.get_datatype_field_count(cstr);
    let fields = slice::from_raw_parts(fields, count);
    let mut values = vec![];
    for i in 0..count {
        let df = wv.get_datatype_field_type(cstr, i);
        let value = match df.datatype {
            Datatype::Int => DataValue::Int(*(fields[i] as *const i64).as_ref_unchecked()),
            Datatype::Float => DataValue::Float(*(fields[i] as *const f64).as_ref_unchecked()),
            Datatype::Bool => DataValue::Bool(*(fields[i] as *const bool).as_ref_unchecked()),
            Datatype::String => {
                let v = CStr::from_ptr(fields[i] as *const c_char).to_str().expect("CString to_str failed");
                DataValue::String(v.to_string())
            }
        };
        values.push(value.clone());
    }
    (&mut *wv).add_component(entity, cstr, &values);
}

#[no_mangle]
unsafe extern "C" fn wv_has_component(wv: &Weave, entity: usize, name: *const c_char) -> bool {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    (&*wv).has_component(entity, cstr)
}

#[no_mangle]
unsafe extern "C" fn wv_get_component(wv: &Weave, entity: usize, name: *const c_char, index: usize) -> *const c_void {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    let v = (&*wv).get_component(entity, cstr);
    match &v[index] {
        DataValue::Int(i) => i as *const _ as *const c_void,
        DataValue::Float(f) => f as *const _ as *const c_void,
        DataValue::Bool(b) => b as *const _ as *const c_void,
        DataValue::String(s) => CString::new(s.clone()).unwrap().into_raw() as *const c_void,
    }
}

#[no_mangle]
unsafe extern "C" fn wv_remove_component(wv: &mut Weave, entity: usize, name: *const c_char) {
    let cstr = CStr::from_ptr(name).to_str().expect("CString to_str failed");
    (&mut *wv).remove_component(entity, cstr);
}

/// SHAPE

#[no_mangle]
unsafe extern "C" fn wv_shape__parent(wv: &mut Weave, root: usize, len: usize, children: *const usize) {
    let children: &[usize] = slice::from_raw_parts(children, len);
    parent(&mut *wv, root, children);
}

#[no_mangle]
unsafe extern "C" fn wv_shape__focus(wv: &mut Weave, lens: usize, len: usize, observers: *const usize) {
    let observers: &[usize] = slice::from_raw_parts(observers, len);
    focus(&mut *wv, lens, observers);
}

#[no_mangle]
unsafe extern "C" fn wv_shape__connect(wv: &mut Weave, source: usize, len: usize, targets: *const usize) {
    let targets: &[usize] = slice::from_raw_parts(targets, len);
    connect(&mut *wv, source, targets);
}

#[no_mangle]
unsafe extern "C" fn wv_shape__hoist(wv: &mut Weave, subject: usize, len: usize, objects: *const usize) {
    let objects: &[usize] = slice::from_raw_parts(objects, len);
    hoist(&mut *wv, subject, objects);
}

#[no_mangle]
unsafe extern "C" fn wv_shape__lift(wv: &mut Weave, len: usize, arrows: *const usize) {
    let arrows: &[usize] = slice::from_raw_parts(arrows, len);
    lift(&mut *wv, arrows);
}

#[no_mangle]
unsafe extern "C" fn wv_shape__dip(wv: &mut Weave, len: usize, arrows: *const usize) {
    let arrows: &[usize] = slice::from_raw_parts(arrows, len);
    dip(&mut *wv, arrows);
}
