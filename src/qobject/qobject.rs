use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;
use libc::{c_int, c_void};
use internal::{QObjectPtr, QObjectSharedPtr, QObjectSignalEmitter, invoke_slot};
use qobject::{QObjectContent, QObjectContentConstructor, QSignalEmitter};

pub struct QObject<T>
    where T: QObjectContent {
    ptr: QObjectSharedPtr,
    content: Box<RefCell<T>>,
}

impl<T> QObject<T>
    where T: QObjectContent {

    pub fn get_content(&self) -> Ref<T> {
        self.content.deref().borrow()
    }

    pub fn get_content_mut(&mut self) -> RefMut<T> {
        self.content.deref().borrow_mut()
    }

    pub(crate) fn get_mut(&mut self) -> &mut c_void
        where T: QObjectContent {
        self.ptr.borrow_mut().as_mut()
    }
}

impl<T> QObject<T>
    where T: QObjectContent + QObjectContentConstructor {
    pub fn new() -> Self {
        let ptr = QObject::<T>::new_ptr();
        let content = Box::new(RefCell::new(T::new(Box::new(QObjectSignalEmitter::new(Rc::downgrade(&ptr))))));
        QObject::new_qobject(ptr, content)
    }

    pub(crate) fn new_with_signal_emitter(signal_emitter: Box<QSignalEmitter>) -> Self {
        let ptr = QObject::<T>::new_ptr();
        let content = Box::new(RefCell::new(T::new(signal_emitter)));
        QObject::new_qobject(ptr, content)
    }

    fn new_ptr() -> QObjectSharedPtr {
        let mut meta = T::get_metaobject();
        let ptr = unsafe {
            de_qobject_create(meta.get_mut(), QObject::<T>::qslot_callback)
        };

        Rc::new(RefCell::new(QObjectPtr::new(ptr)))
    }

    fn new_qobject(ptr: QObjectSharedPtr, content: Box<RefCell<T>>) -> Self {
        let content_ptr = Box::into_raw(content);
        unsafe { de_qobject_set_dobject(ptr.borrow_mut().as_mut(), content_ptr as *mut c_void); }

        let returned = QObject {
            ptr: ptr.clone(),
            content: unsafe { Box::from_raw(content_ptr) },
        };
        returned
    }

    extern "C" fn qslot_callback(object: *mut c_void, slot_name: *mut c_void,
                                 argc: c_int, argv: *mut *mut c_void) {
        invoke_slot::<T>(object, slot_name, argc, argv);
    }
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