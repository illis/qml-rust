use std::cell::{Ref, RefCell, RefMut};
use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_char, c_int, c_void};
use std::rc::Rc;

use internal::{
    invoke_slot,
    QListModelInterfaceImpl,
    QObjectPtr,
    QObjectSharedPtr,
    QObjectSignalEmitter,
};
use qlistmodel::{QListModelContentConstructor, QListModelItem};
use qobject::{QObjectContent, QSignalEmitter};

pub struct QListModel<T, I>
where
    T: QObjectContent,
    I: QListModelItem,
{
    ptr: QObjectSharedPtr,
    content: Box<RefCell<T>>,
    _phantom: PhantomData<I>,
}

impl<T, I> QListModel<T, I>
where
    T: QObjectContent,
    I: QListModelItem,
{
    pub fn content(&self) -> Ref<T> {
        self.content.deref().borrow()
    }

    pub fn content_mut(&mut self) -> RefMut<T> {
        self.content.deref().borrow_mut()
    }

    pub(crate) fn as_cref_mut(&mut self) -> &mut c_void {
        self.ptr.borrow_mut().as_cref_mut()
    }
}

impl<T, I> QListModel<T, I>
where
    T: QObjectContent + QListModelContentConstructor<I>,
    I: QListModelItem + 'static,
{
    pub fn new() -> Self {
        let ptr = QListModel::<T, I>::new_ptr(I::role_names());
        let interface = Box::new(QListModelInterfaceImpl::new(Rc::downgrade(&ptr)));
        let content = Box::new(RefCell::new(T::new(
            Box::new(QObjectSignalEmitter::new(Rc::downgrade(&ptr))),
            interface,
        )));
        QListModel::new_listmodel(&ptr, content)
    }

    pub(crate) fn new_with_signal_emitter(signal_emitter: Box<QSignalEmitter>) -> Self {
        let ptr = QListModel::<T, I>::new_ptr(I::role_names());
        let interface = Box::new(QListModelInterfaceImpl::new(Rc::downgrade(&ptr)));
        let content = Box::new(RefCell::new(T::new(signal_emitter, interface)));
        QListModel::new_listmodel(&ptr, content)
    }

    fn new_ptr(role_names: Vec<&str>) -> QObjectSharedPtr {
        let meta = T::metaobject();
        let role_name_wrapper: Vec<CString> = role_names
            .into_iter()
            .map(|role| CString::new(role).unwrap())
            .collect();
        let role_name_cstring: Vec<*const c_char> =
            role_name_wrapper.iter().map(|role| role.as_ptr()).collect();

        let ptr = unsafe {
            de_qlistmodel_create(
                meta.as_ptr(),
                role_name_cstring.as_ptr(),
                role_name_cstring.len() as c_int,
                QListModel::<T, I>::qslot_callback,
            )
        };

        Rc::new(RefCell::new(QObjectPtr::new(ptr)))
    }

    fn new_listmodel(ptr: &QObjectSharedPtr, content: Box<RefCell<T>>) -> Self {
        let content_ptr = Box::into_raw(content);
        unsafe {
            de_qlistmodel_set_dobject(ptr.borrow_mut().as_cref_mut(), content_ptr as *mut c_void);
        }

        QListModel {
            ptr: Rc::clone(ptr),
            content: unsafe { Box::from_raw(content_ptr) },
            _phantom: PhantomData,
        }
    }

    extern "C" fn qslot_callback(
        object: *mut c_void,
        slot_name: *mut c_void,
        argc: c_int,
        argv: *mut *mut c_void,
    ) {
        invoke_slot::<T>(object, slot_name, argc, argv);
    }
}

impl<T, I> Default for QListModel<T, I>
where
    T: QObjectContent + QListModelContentConstructor<I>,
    I: QListModelItem + 'static,
{
    fn default() -> Self {
        QListModel::new()
    }
}

type DObjectCallback = extern "C" fn(*mut c_void, *mut c_void, c_int, *mut *mut c_void);

extern "C" {
    fn de_qlistmodel_create(
        meta_object: *const c_void,
        role_array: *const *const c_char,
        role_array_length: c_int,
        callback: DObjectCallback,
    ) -> *mut c_void;
    fn de_qlistmodel_set_dobject(vptr: *mut c_void, content: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::QListModel;
    use qlistmodel::{QListModelContentConstructor, QListModelInterface, QListModelItem};
    use qmetaobject::QMetaObject;
    use qobject::{QObjectContent, QSignalEmitter};
    use qvariant::{QVariant, QVariantMap, QVariantRefMut};
    use std::collections::HashMap;

    struct Content {}

    struct Item {}

    impl QObjectContent for Content {
        fn metaobject() -> QMetaObject {
            QMetaObject::new_qlistmodel("Meta", Vec::new(), Vec::new(), Vec::new())
        }

        fn invoke_slot(&mut self, _: &str, _: Vec<QVariantRefMut>) -> Option<QVariant> {
            None
        }
    }

    impl QListModelContentConstructor<Item> for Content {
        fn new(_: Box<QSignalEmitter>, _: Box<QListModelInterface<Item>>) -> Self {
            Content {}
        }
    }

    impl QListModelItem for Item {
        fn role_names() -> Vec<&'static str> {
            vec!["test1", "test2"]
        }

        fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
            HashMap::new()
        }

        fn from_variant_map(_: QVariantMap) -> Self {
            Item {}
        }
    }

    #[test]
    fn test_qlistmodel_memory() {
        QListModel::<Content, Item>::new();
    }
}
