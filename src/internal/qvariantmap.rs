use libc::{c_char, c_int, c_void};
use std::ffi::CString;

pub(crate) struct QVariantMapEntry {
    pub(crate) key: CString,
    pub(crate) value: *const c_void,
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