use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::slice::from_raw_parts_mut;

use conversions::TryFrom;
use errors::{Error, ErrorKind, Result};
use internal::ffi;
use internal::ffi::{de_qvariant_create_qvariantmap, dos_qvariant_create_qvariant};
use internal::qvariantmap::{as_entries, as_ffi_entries, as_ffi_map, FfiQVariantMapWrapper};
use qvariant::QVariant;
// use qvariant::QVariantRefMut;

pub type QVariantMap = HashMap<String, QVariant>;

impl TryFrom<QVariant> for QVariantMap {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self> {
        from_variant_ptr(value.as_raw())
    }
}

impl<'a> TryFrom<&'a QVariant> for QVariantMap {
    type Error = Error;

    fn try_from(value: &QVariant) -> Result<Self> {
        from_variant_ptr(value.as_raw())
    }
}

impl TryFrom<QVariantMap> for QVariant {
    type Error = Error;

    fn try_from(value: QVariantMap) -> Result<QVariant> {
        let entries = as_entries(&value)?;
        let mut ffi_entries = as_ffi_entries(&entries);
        let ffi_map = as_ffi_map(&mut ffi_entries);

        Ok(QVariant::from_raw(unsafe {
            de_qvariant_create_qvariantmap(&ffi_map)
        }))
    }
}

/*
impl<'a, 'b> From<QVariantRefMut<'a>> for QVariantMap<'b> {
    fn from(value: QVariantRefMut) -> Self {
        from_ptr(value.as_cref())
    }
}

impl<'a, 'b: 'a, 'c> From<&'b QVariantRefMut<'a>> for QVariantMap<'c> {
    fn from(value: &QVariantRefMut) -> Self {
        from_ptr(value.as_cref())
    }
}
*/

fn from_raw_entry(entry: &ffi::QVariantMapEntry) -> Result<(String, QVariant)> {
    let key = unsafe { CStr::from_ptr(entry.key) };
    let key = key.to_str()?;

    let value = QVariant::from_raw(unsafe { dos_qvariant_create_qvariant(entry.value) });

    Ok((String::from(key), value))
}

fn from_variant_ptr(ptr: *const c_void) -> Result<QVariantMap> {
    let wrapper = FfiQVariantMapWrapper::from_variant_ptr(ptr);
    let ptr = unsafe { wrapper.ptr.as_ref() };
    let ptr = ptr.ok_or(ErrorKind::NullPointerError)?;
    let slice = unsafe { from_raw_parts_mut(ptr.values, ptr.count as usize) };

    slice
        .iter()
        .map(from_raw_entry)
        .collect::<Result<QVariantMap>>()
}

#[cfg(test)]
mod tests {
    use super::QVariantMap;
    use qvariant::QVariant;
    use conversions::TryFrom;

    #[test]
    fn test_qvariant_qvariantmap_memory() {
        let mut map = QVariantMap::new();
        map.insert("First".to_string(), QVariant::from(123));
        map.insert("Second".to_string(), QVariant::try_from("123").unwrap());

        let variant = QVariant::try_from(map).unwrap();
        QVariantMap::try_from(variant).unwrap();
    }
}
