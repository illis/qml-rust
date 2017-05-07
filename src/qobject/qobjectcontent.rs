use qmetaobject::QMetaObject;
use qobject::QSignalEmitter;
use qvariant::{QVariant, QVariantRefMut};

pub trait QObjectContent {
    fn get_metaobject() -> QMetaObject;
    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant>;
}

pub trait QObjectContentConstructor {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self;
}
