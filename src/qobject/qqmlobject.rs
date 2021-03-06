use std::marker::PhantomData;
use std::os::raw::c_void;

use internal::QQmlObjectSignalEmitter;
use qobject::{QObject, QObjectContent, QObjectContentConstructor};

pub struct QQmlObject<T>
where
    T: QObjectContent,
{
    _phantom: PhantomData<T>,
}

impl<T> QQmlObject<T>
where
    T: QObjectContent + QObjectContentConstructor,
{
    pub fn new(wrapper: *mut c_void) -> QObject<T> {
        QObject::new_with_signal_emitter(Box::new(QQmlObjectSignalEmitter::new(wrapper)))
    }
}
