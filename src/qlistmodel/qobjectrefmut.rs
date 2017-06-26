use qobject::{QObjectContent, QObjectRefMut};
use qobject::qobjectrefmut;
use qlistmodel::{QListModel, QListModelItem};

impl<'a, T, I> From<&'a mut QListModel<T, I>> for QObjectRefMut<'a>
    where T: QObjectContent, I: QListModelItem {
    fn from(value: &'a mut QListModel<T, I>) -> Self {
        qobjectrefmut::new(super::qlistmodel::get_mut(value))
    }
}