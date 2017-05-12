use std::cell::RefCell;
use std::rc::Rc;
use libc::{c_int, c_void};
use internal::{QObjectPtr, QObjectSharedPtr, QObjectSignalEmitter, invoke_slot};
use qobject::{QObjectContent, QObjectContentConstructor, QSignalEmitter};

pub struct QObject<T>
    where T: QObjectContent {
    ptr: QObjectSharedPtr,
    content: Box<T>,
}

impl<T> QObject<T>
    where T: QObjectContent + QObjectContentConstructor {
    pub fn new() -> Self {
        let ptr = QObject::<T>::new_ptr();
        let content = Box::new(T::new(Box::new(QObjectSignalEmitter::new(Rc::downgrade(&ptr)))));
        QObject::new_qobject(ptr, content)
    }

    fn new_with_signal_emitter(signal_emitter: Box<QSignalEmitter>) -> Self {
        let ptr = QObject::<T>::new_ptr();
        let content = Box::new(T::new(signal_emitter));
        QObject::new_qobject(ptr, content)
    }

    fn new_ptr() -> QObjectSharedPtr {
        let mut meta = T::get_metaobject();
        let ptr = unsafe {
            de_qobject_create(::qmetaobject::get_mut(&mut meta), QObject::<T>::qslot_callback)
        };

        Rc::new(RefCell::new(QObjectPtr::new(ptr)))
    }

    fn new_qobject(ptr: QObjectSharedPtr, content: Box<T>) -> Self {
        let content_ptr = Box::into_raw(content);
        unsafe { de_qobject_set_dobject(ptr.borrow_mut().as_mut(), content_ptr as *mut c_void); }

        let returned = QObject {
            ptr: ptr.clone(),
            content: unsafe { Box::from_raw(content_ptr) },
        };
        returned
    }

    pub fn get_content(&self) -> &T {
        &*self.content
    }

    pub fn get_content_mut(&mut self) -> &mut T {
        &mut *self.content
    }

    extern "C" fn qslot_callback(object: *mut c_void, slot_name: *mut c_void,
                                 argc: c_int, argv: *mut *mut c_void) {
        invoke_slot::<T>(object, slot_name, argc, argv)
    }
}

pub fn get_mut<'a, T>(instance: &'a mut QObject<T>) -> &'a mut c_void
    where T: QObjectContent {
    let ptr = instance.ptr.borrow_mut().as_mut();
    unsafe { ptr.as_mut().unwrap() }
}

pub fn new_with_signal_emitter<T>(signal_emitter: Box<QSignalEmitter>) -> QObject<T>
    where T: QObjectContent + QObjectContentConstructor {
    QObject::new_with_signal_emitter(signal_emitter)
}

type DObjectCallback = extern "C" fn(*mut c_void, *mut c_void, c_int, *mut *mut c_void);

extern "C" {
    fn de_qobject_create(meta_object: *const c_void, callback: DObjectCallback) -> *mut c_void;
    fn de_qobject_set_dobject(vptr: *mut c_void, content: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::{QObject, QSignalEmitter};
    use qmetaobject::QMetaObject;
    use qobject::{QObjectContent, QObjectContentConstructor};
    use qvariant::{QVariant, QVariantRefMut};

    struct Content {}

    impl QObjectContent for Content {
        fn get_metaobject() -> QMetaObject {
            QMetaObject::new_qobject("Meta", Vec::new(), Vec::new(), Vec::new())
        }

        fn invoke_slot(&mut self, _: &str, _: Vec<QVariantRefMut>) -> Option<QVariant> {
            None
        }
    }

    impl QObjectContentConstructor for Content {
        fn new(_: Box<QSignalEmitter>) -> Self {
            Content {}
        }
    }

    #[test]
    fn test_qobject_memory() {
        QObject::<Content>::new();
    }
}