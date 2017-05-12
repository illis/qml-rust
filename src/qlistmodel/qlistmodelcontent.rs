use qlistmodel::QListModelInterface;
use qobject::QSignalEmitter;

pub trait QListModelContentConstructor {
    fn new(signal_emitter: Box<QSignalEmitter>, model_interface: Box<QListModelInterface>) -> Self;
}
