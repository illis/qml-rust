use qvariant::QVariant;

pub trait QSignalEmitter {
    fn emit_signal(&self, name: &str, args: Vec<QVariant>);
}