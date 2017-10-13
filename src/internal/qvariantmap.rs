use libc::{c_char, c_int, c_void};
use std::collections::HashMap;
use std::ffi::CString;
use qvariant::{QVariant, QVariantMap};

pub(crate) struct QVariantMapEntry {
    key: CString,
    value: *const c_void,
}

#[repr(C)]
pub(crate) struct CQVariantMapEntry {
    pub(crate) key: *const c_char,
    pub(crate) value: *const c_void,
}

#[repr(C)]
pub(crate) struct CQVariantMap {
    pub(crate) count: c_int,
    pub(crate) values: *mut CQVariantMapEntry,
}

pub(crate) fn variantmap_to_entries<'a>(value: &QVariantMap<'a>) -> Vec<QVariantMapEntry> {
    value.iter()
        .map(|(key, value)| {
            QVariantMapEntry {
                key: CString::new(key.as_str()).unwrap(),
                value: value.as_cref(),
            }
        }).collect::<Vec<_>>()
}

pub(crate) fn static_variantmap_to_entries<'a>(value: &HashMap<&'static str, QVariant<'a>>) -> Vec<QVariantMapEntry> {
    value.iter()
        .map(|(key, value)| {
            QVariantMapEntry {
                key: CString::new(*key).unwrap(),
                value: value.as_cref(),
            }
        }).collect::<Vec<_>>()
}

pub(crate) fn entries_to_c_entries(value: &[QVariantMapEntry]) -> Vec<CQVariantMapEntry> {
    value.iter()
        .map(|entry| {
            CQVariantMapEntry {
                key: entry.key.as_ptr(),
                value: entry.value,
            }
        }).collect::<Vec<_>>()
}

pub(crate) fn c_entries_to_c_map(value: &mut Vec<CQVariantMapEntry>) -> CQVariantMap {
    CQVariantMap {
        count: value.len() as c_int,
        values: value.as_mut_ptr(),
    }
}

pub(crate) struct CQVariantMapWrapper<'a> {
    pub(crate) ptr: &'a mut CQVariantMap,
}

impl<'a> Drop for CQVariantMapWrapper<'a> {
    fn drop(&mut self) {
        unsafe { de_qvariantmap_delete(self.ptr) }
    }
}

extern "C" {
    fn de_qvariantmap_delete(vptr: *const CQVariantMap);
}
