use std::slice::from_raw_parts_mut;
use libc::{c_int, c_void};
use qmetaobject::{QMetaObject, SignalDefinition, SlotDefinition, PropertyDefinition};
use qvariant::QVariant;
use qvariantview::QVariantView;

use std::ffi::CString;

pub trait QObjectContent {
    fn qslot_call(&mut self, name: &str, args: Vec<QVariantView>) -> Option<QVariant>;
}

pub struct QObject<'a, T: QObjectContent> {
    ptr: &'a mut c_void,
    meta: QMetaObject,
    content: Box<T>
}

impl<'a, T: QObjectContent> QObject<'a, T> {
    pub fn new(name: &str,
               signal_definitions: Vec<SignalDefinition>,
               slot_definitions: Vec<SlotDefinition>,
               property_definitions: Vec<PropertyDefinition>,
               content: T) -> Option<Self> {
        let content = Box::new(content);
        let content = Box::into_raw(content) as *mut T;
        let mut meta = QMetaObject::new_qobject(name, signal_definitions, slot_definitions,
                                                property_definitions);
        let ptr = unsafe {
            de_qobject_create(content as *mut c_void, ::qmetaobject::get_mut(&mut meta),
                              QObject::<'a, T>::qslot_callback).as_mut()
        };
        ptr.map(|ptr| {
            QObject {
                ptr: ptr,
                meta: meta,
                content: unsafe {Box::from_raw(content)},
            }
        })
    }

    pub fn get_content(&self) -> &T {
        &self.content
    }

    pub fn get_content_mut(&mut self) -> &mut T {
        &mut self.content
    }

    pub fn as_mut(&mut self) -> *mut c_void {
        self.ptr
    }

    extern "C" fn qslot_callback(object: *mut c_void, slot_name: *mut c_void,
                                 argc: c_int, argv: *mut *mut c_void) {
        let mut object_ptr = object as *mut T;
        let object = unsafe {object_ptr.as_mut()}.unwrap();
        let slice = unsafe {from_raw_parts_mut(argv, argc as usize)};
        let vec: Vec<QVariantView> = slice.iter()
            .skip(1)
            .map(|&variant| QVariantView::from_ptr(variant))
            .collect();
        let slot_name: String = QVariantView::from_ptr(slot_name).into();

        if let Some(returned) = object.qslot_call(&slot_name, vec) {
            let mut output = QVariantView::from_ptr(slice[0]);
            output.set(&returned);
        }
    }

}

impl<'a, T: QObjectContent> Drop for QObject<'a, T> {
    fn drop(&mut self) {
        unsafe {
            dos_qobject_delete(self.ptr);
        }
    }
}

type DObjectCallback = extern "C" fn(*mut c_void, *mut c_void, c_int, *mut *mut c_void);

extern "C" {
    fn de_qobject_create(object: *mut c_void, meta_object: *const c_void,
                         callback: DObjectCallback) -> *mut c_void;
    fn dos_qobject_delete(vptr: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::{QObject, QObjectContent};
    use qvariant::QVariant;
    use qvariantview::QVariantView;

    impl QObjectContent for i32 {
        fn qslot_call(&mut self, name: &str, args: Vec<QVariantView>) -> Option<QVariant> {
            None
        }
    }

    #[test]
    fn test_qobject_memory() {
        QObject::new("Meta", Vec::new(), Vec::new(), Vec::new(), 123);
    }
}