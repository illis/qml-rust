use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use libc::{c_char, c_int, c_void};
use internal::{QObjectPtr, QObjectSharedPtr, QObjectSignalEmitter, invoke_slot};
use qobject::{QObjectContent, QSignalEmitter};
use qlistmodel::{QListModelContentConstructor, QListModelInterface};

pub struct QListModel<T>
    where T: QObjectContent {
    ptr: QObjectSharedPtr,
    content: Box<T>,
}

impl<T> QListModel<T>
    where T: QObjectContent + QListModelContentConstructor {
    pub fn new(role_names: Vec<&str>) -> Self {
        let ptr = QListModel::<T>::new_ptr(role_names);
        let interface = Box::new(ListModelInterface::new());
        let content = Box::new(T::new(Box::new(QObjectSignalEmitter::new(Rc::downgrade(&ptr))), interface));
        QListModel::new_listmodel(ptr, content)
    }

    fn new_with_signal_emitter(role_names: Vec<&str>, signal_emitter: Box<QSignalEmitter>) -> Self {
        let ptr = QListModel::<T>::new_ptr(role_names);
        let interface = Box::new(ListModelInterface::new());
        let content = Box::new(T::new(signal_emitter, interface));
        QListModel::new_listmodel(ptr, content)
    }

    fn new_ptr(role_names: Vec<&str>) -> QObjectSharedPtr {
        let mut meta = T::get_metaobject();
        let role_name_wrapper: Vec<CString> = role_names.into_iter()
            .map(|role| CString::new(role).unwrap())
            .collect();
        let role_name_cstring: Vec<*const c_char> = role_name_wrapper.iter()
            .map(|role| role.as_ptr())
            .collect();

        let ptr = unsafe {
            de_qlistmodel_create(::qmetaobject::get_mut(&mut meta), role_name_cstring.as_ptr(),
                                 role_name_cstring.len() as c_int, QListModel::<T>::qslot_callback)
        };

        Rc::new(RefCell::new(QObjectPtr::new(ptr)))
    }

    fn new_listmodel(ptr: QObjectSharedPtr, content: Box<T>) -> Self {
        let content_ptr = Box::into_raw(content);
        unsafe { de_qlistmodel_set_dobject(ptr.borrow_mut().as_mut(), content_ptr as *mut c_void); }

        let returned = QListModel {
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

struct ListModelInterface {}

impl ListModelInterface {
    fn new() -> Self {
        ListModelInterface {}
    }
}

impl QListModelInterface for ListModelInterface {}

pub fn get_mut<'a, T>(instance: &'a mut QListModel<T>) -> &'a mut c_void
    where T: QObjectContent {
    let ptr = instance.ptr.borrow_mut().as_mut();
    unsafe { ptr.as_mut().unwrap() }
}

pub fn new_with_signal_emitter<T>(role_names: Vec<&str>, signal_emitter: Box<QSignalEmitter>) -> QListModel<T>
    where T: QObjectContent + QListModelContentConstructor {
    QListModel::new_with_signal_emitter(role_names, signal_emitter)
}

type DObjectCallback = extern "C" fn(*mut c_void, *mut c_void, c_int, *mut *mut c_void);

extern "C" {
    fn de_qlistmodel_create(meta_object: *const c_void, role_array: *const *const c_char,
                            role_array_length: c_int, callback: DObjectCallback) -> *mut c_void;
    fn de_qlistmodel_set_dobject(vptr: *mut c_void, content: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::{QListModel};
    use qmetaobject::QMetaObject;
    use qobject::{QObjectContent, QSignalEmitter};
    use qlistmodel::{QListModelContentConstructor, QListModelInterface};
    use qvariant::{QVariant, QVariantRefMut};

    struct Content {}

    impl QObjectContent for Content {
        fn get_metaobject() -> QMetaObject {
            QMetaObject::new_qlistmodel("Meta", Vec::new(), Vec::new(), Vec::new())
        }

        fn invoke_slot(&mut self, _: &str, _: Vec<QVariantRefMut>) -> Option<QVariant> {
            None
        }
    }

    impl QListModelContentConstructor for Content {
        fn new(_: Box<QSignalEmitter>, _: Box<QListModelInterface>) -> Self {
            Content {}
        }
    }

    #[test]
    fn test_qlistmodel_memory() {
        QListModel::<Content>::new(vec!["test1", "test2"]);
    }
}