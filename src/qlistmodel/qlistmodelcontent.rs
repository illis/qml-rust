use qlistmodel::QListModelInterface;
use qobject::QSignalEmitter;

pub trait QListModelContent {
    fn role_names() -> Vec<&'static str>;
}

pub trait QListModelContentConstructor {
    fn new(signal_emitter: Box<QSignalEmitter>, model_interface: Box<QListModelInterface>) -> Self;
}
