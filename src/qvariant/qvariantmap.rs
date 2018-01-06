use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::slice::from_raw_parts_mut;

use internal::{
    c_entries_to_c_map,
    entries_to_c_entries,
    variantmap_to_entries,
    CQVariantMap,
    CQVariantMapWrapper,
};
use qvariant::QVariant;
use qvariant::QVariantRefMut;

pub type QVariantMap<'a> = HashMap<String, QVariant<'a>>;

impl<'a, 'b> From<QVariantMap<'a>> for QVariant<'b> {
    fn from(value: QVariantMap<'a>) -> Self {
        let entries = variantmap_to_entries(&value);
        let mut c_entries = entries_to_c_entries(&entries);
        let c_map = c_entries_to_c_map(&mut c_entries);
        QVariant::new(unsafe { de_qvariant_create_qvariantmap(&c_map).as_mut().unwrap() })
    }
}

impl<'a, 'b> From<QVariant<'a>> for QVariantMap<'b> {
    fn from(value: QVariant) -> Self {
        from_ptr(value.as_cref())
    }
}

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

fn from_ptr<'a, 'b>(ptr: &'a c_void) -> QVariantMap<'b> {
    let wrapper = CQVariantMapWrapper::from_variant_ptr(ptr);
    let slice = unsafe { from_raw_parts_mut(wrapper.ptr.values, wrapper.ptr.count as usize) };

    slice
        .iter()
        .map(|value| {
            let key = unsafe { CStr::from_ptr(value.key) };
            let key = key.to_string_lossy().into_owned();
            let value = QVariant::new(unsafe {
                dos_qvariant_create_qvariant(value.value).as_mut().unwrap()
            });
            (key, value)
        })
        .collect::<QVariantMap>()
}

impl<'a> CQVariantMapWrapper<'a> {
    fn from_variant_ptr(value: &'a c_void) -> Self {
        CQVariantMapWrapper {
            ptr: unsafe { de_qvariant_to_qvariantmap(value).as_mut().unwrap() },
        }
    }
}

extern "C" {
    fn dos_qvariant_create_qvariant(value: *const c_void) -> *mut c_void;
    fn de_qvariant_create_qvariantmap(value: *const CQVariantMap) -> *mut c_void;
    fn de_qvariant_to_qvariantmap(value: *const c_void) -> *mut CQVariantMap;
}

#[cfg(test)]
mod tests {
    use super::QVariantMap;
    use qvariant::QVariant;

    #[test]
    fn test_qvariant_qvariantmap_memory() {
        let mut map = QVariantMap::new();
        map.insert("First".to_string(), QVariant::from(123));
        map.insert("Second".to_string(), QVariant::from("123"));

        let variant = QVariant::from(map);
        QVariantMap::from(variant);
    }
}
