use qlistmodel::{QListModelInterface, QListModelItem};
use qobject::QSignalEmitter;

pub trait QListModelContentConstructor<I>
where
    I: QListModelItem,
{
    fn new(
        signal_emitter: Box<QSignalEmitter>,
        model_interface: Box<QListModelInterface<I>>,
    ) -> Self;
}
