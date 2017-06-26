use std::marker::PhantomData;
use libc::c_void;
use internal::QQmlObjectSignalEmitter;
use qlistmodel::{QListModel, QListModelContentConstructor, QListModelItem};
use qobject::QObjectContent;

pub struct QQmlListModel<T, I>
    where T: QObjectContent, I: QListModelItem {
    _phantom_t: PhantomData<T>,
    _phantom_i: PhantomData<I>,
}

impl<T, I> QQmlListModel<T, I>
    where T: QObjectContent + QListModelContentConstructor, I: QListModelItem {
    pub fn new(wrapper: *mut c_void) -> QListModel<T, I> {
        QListModel::new_with_signal_emitter(Box::new(QQmlObjectSignalEmitter::new(wrapper)))
    }
}
