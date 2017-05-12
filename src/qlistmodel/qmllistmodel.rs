use std::marker::PhantomData;
use libc::c_void;
use internal::QQmlObjectSignalEmitter;
use qlistmodel::{QListModel, QListModelContent, QListModelContentConstructor};
use qobject::QObjectContent;

pub struct QQmlListModel<T>
    where T: QObjectContent {
    _phantom: PhantomData<T>,
}

impl<T> QQmlListModel<T>
    where T: QObjectContent + QListModelContent + QListModelContentConstructor {
    pub fn new(wrapper: *mut c_void) -> QListModel<T> {
        super::qlistmodel::new_with_signal_emitter(Box::new(QQmlObjectSignalEmitter::new(wrapper)))
    }
}
