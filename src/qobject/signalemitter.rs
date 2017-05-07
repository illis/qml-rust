use qvariant::QVariant;

pub trait QSignalEmitter {
    fn emit_signal(&mut self, name: &str, args: Vec<QVariant>);
}