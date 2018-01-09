use std::ffi::CString;
use std::os::raw::{c_int, c_void};

use errors::Result;
use internal::ffi;
use internal::ffi::{de_qvariant_to_qvariantmap, de_qvariantmap_delete};
use qvariant::QVariant;
use qvariantmap::QVariantMap;

pub(crate) struct QVariantMapEntry {
    key: CString,
    value: *const c_void,
}

impl QVariantMapEntry {
    fn from_key_value(key: &str, value: &QVariant) -> Result<Self> {
        let key = CString::new(key)?;

        Ok(QVariantMapEntry {
            key,
            value: value.as_raw(),
        })
    }
}

pub(crate) fn as_entries(value: &QVariantMap) -> Result<Vec<QVariantMapEntry>> {
    value
        .iter()
        .map(|(key, value)| QVariantMapEntry::from_key_value(key, value))
        .collect::<Result<Vec<_>>>()
}

/*
pub(crate) fn static_variantmap_to_entries(
    value: &HashMap<&'static str, QVariant>,
) -> Vec<QVariantMapEntry> {
    value
        .iter()
        .map(|(key, value)| QVariantMapEntry {
            key: CString::new(*key).unwrap(),
            value: value.as_raw(),
        })
        .collect::<Vec<_>>()
}
*/

pub(crate) fn as_ffi_entries(value: &[QVariantMapEntry]) -> Vec<ffi::QVariantMapEntry> {
    value
        .iter()
        .map(|entry| ffi::QVariantMapEntry {
            key: entry.key.as_ptr(),
            value: entry.value,
        })
        .collect::<Vec<_>>()
}

pub(crate) fn as_ffi_map(value: &mut Vec<ffi::QVariantMapEntry>) -> ffi::QVariantMap {
    ffi::QVariantMap {
        count: value.len() as c_int,
        values: value.as_mut_ptr(),
    }
}

pub(crate) struct FfiQVariantMapWrapper {
    pub(crate) ptr: *mut ffi::QVariantMap,
}

impl FfiQVariantMapWrapper {
    pub(crate) fn from_variant_ptr(value: *const c_void) -> Self {
        FfiQVariantMapWrapper {
            ptr: unsafe { de_qvariant_to_qvariantmap(value) },
        }
    }
}

impl Drop for FfiQVariantMapWrapper {
    fn drop(&mut self) {
        unsafe { de_qvariantmap_delete(self.ptr) }
    }
}
