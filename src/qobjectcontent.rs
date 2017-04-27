use qmetaobject::QMetaObject;
use qsignalemitter::QSignalEmitter;
use qvariant::QVariant;
use qvariantview::QVariantView;

pub trait QObjectContent {
    fn get_metatype() -> QMetaObject;
    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantView>) -> Option<QVariant>;
}

pub trait QObjectContentConstructor {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self;
}
