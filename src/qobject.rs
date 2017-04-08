use std::mem;
use std::slice::from_raw_parts_mut;
use libc::{c_int, c_void};
use qmetaobject::{QMetaObject, SignalDefinition, SlotDefinition, PropertyDefinition};

pub struct QObject<T> {
    ptr: *mut c_void,
    meta: QMetaObject,
    content: *mut T,
}

impl<T> QObject<T> {
    pub fn new(name: &str,
               signal_definitions: Vec<SignalDefinition>,
               slot_definitions: Vec<SlotDefinition>,
               property_definitions: Vec<PropertyDefinition>,
               content: T) -> QObject<T> {
        unsafe {
            let content = Box::new(content);
            let content: *mut T = Box::into_raw(content);
            let meta = QMetaObject::new_qobject(name, signal_definitions, slot_definitions,
                                                property_definitions);
            QObject {
                ptr: dos_qobject_create(content as *mut c_void, meta.as_ptr(), callback),
                meta: meta,
                content: content,
            }
        }
    }

    pub fn get_content(&self) -> &T {
        unsafe {
            &*self.content
        }
    }

    pub fn get_content_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.content
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl<T> Drop for QObject<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.content);
            dos_qobject_delete(self.ptr);
        }
    }
}

extern "C" fn callback(object: *mut c_void, slot_name: *mut c_void,
                       argc: c_int, argv: *mut *mut c_void) {
    unsafe {
        // let mut obj: Box<&mut QObjectImpl> = Box::from_raw(obj as *mut &mut QObjectImpl);
        // let slice = from_raw_parts_mut(argv, argc as usize);
        // let vec: Vec<QVariant> = slice.iter().skip(1).map(|&dq| dq.into()).collect();
        // let slotName: String = new_qvariant(slotName).into();
        // println!("Right before going in... name: {}, argc: {}",
        //  slotName,
        //  argc);
        /*if let Some(qvar) = obj.qslot_call(&slotName, vec) {
            let mut qv: QVariant = slice[0].into();
            qv.set(qvar);
        }
        forget(obj);*/
    }
}

type DObjectCallback = extern "C" fn(*mut c_void, *mut c_void, c_int, *mut *mut c_void);

extern "C" {
    fn dos_qobject_create(object: *mut c_void, meta_object: *mut c_void,
                          callback: DObjectCallback) -> *mut c_void;
    fn dos_qobject_delete(vptr: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::QObject;
    use std::ptr;

    #[test]
    fn test_qobject_memory() {
        let object = QObject::new("Meta", Vec::new(), Vec::new(), Vec::new(), 123);
        assert_ne!(object.as_ptr(), ptr::null_mut());
    }
}