use libc::{c_char, c_int, c_void};
use std::ffi::CString;

pub(crate) struct QVariantMapEntry {
    pub key: CString,
    pub value: *const c_void,
}

#[repr(C)]
pub(crate) struct CQVariantMapEntry {
    pub key: *const c_char,
    pub value: *const c_void,
}

#[repr(C)]
pub(crate) struct CQVariantMap {
    pub count: c_int,
    pub values: *mut CQVariantMapEntry,
}