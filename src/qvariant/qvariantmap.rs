use libc::{c_int, c_void};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::slice::from_raw_parts_mut;
use qvariant::QVariant;
use qvariant::QVariantRefMut;
use internal::{CQVariantMap, CQVariantMapEntry, QVariantMapEntry};

pub type QVariantMap<'a> = HashMap<String, QVariant<'a>>;

impl<'a, 'b> From<QVariantMap<'a>> for QVariant<'b> {
    fn from(value: QVariantMap<'a>) -> Self {
        let wrappers = value.iter()
            .map(|(key, value)| {
                QVariantMapEntry {
                    key: CString::new(key.as_str()).unwrap(),
                    value: value.get_ptr(),
                }
            }).collect::<Vec<_>>();

        let mut entries = wrappers.iter()
            .map(|entry| {
                CQVariantMapEntry {
                    key: entry.key.as_ptr(),
                    value: entry.value,
                }
            }).collect::<Vec<_>>();

        let map = CQVariantMap {
            count: entries.len() as c_int,
            values: entries.as_mut_ptr(),
        };

        QVariant::new(unsafe { de_qvariant_create_qvariantmap(&map).as_mut().unwrap() })
    }
}

impl<'a, 'b> From<QVariant<'a>> for QVariantMap<'b> {
    fn from(value: QVariant) -> Self {
        from_ptr(value.get_ptr())
    }
}

impl<'a, 'b> From<QVariantRefMut<'a>> for QVariantMap<'b> {
    fn from(value: QVariantRefMut) -> Self {
        from_ptr(value.get_ptr())
    }
}

impl<'a, 'b: 'a, 'c> From<&'b QVariantRefMut<'a>> for QVariantMap<'c> {
    fn from(value: &QVariantRefMut) -> Self {
        from_ptr(value.get_ptr())
    }
}

fn from_ptr<'a, 'b>(ptr: &'a c_void) -> QVariantMap<'b> {
    let wrapper = QVariantMapWrapper::new(ptr);
    let slice = unsafe { from_raw_parts_mut(wrapper.ptr.values, wrapper.ptr.count as usize) };

    slice.iter().map(|value| {
        let key = unsafe { CStr::from_ptr(value.key) };
        let key = key.to_string_lossy().into_owned();
        let value = QVariant::new(unsafe { dos_qvariant_create_qvariant(value.value).as_mut().unwrap() });
        (key, value)
    }).collect::<QVariantMap>()
}

struct QVariantMapWrapper<'a> {
    ptr: &'a mut CQVariantMap,
}

impl<'a> QVariantMapWrapper<'a> {
    fn new(value: &'a c_void) -> Self {
        QVariantMapWrapper {
            ptr: unsafe { de_qvariant_to_qvariantmap(value).as_mut().unwrap() },
        }
    }
}

impl<'a> Drop for QVariantMapWrapper<'a> {
    fn drop(&mut self) {
        unsafe { de_qvariantmap_delete(self.ptr) }
    }
}

extern "C" {
    fn dos_qvariant_create_qvariant(value: *const c_void) -> *mut c_void;
    fn de_qvariant_create_qvariantmap(value: *const CQVariantMap) -> *mut c_void;
    fn de_qvariant_to_qvariantmap(value: *const c_void) -> *mut CQVariantMap;
    fn de_qvariantmap_delete(vptr: *const CQVariantMap);
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