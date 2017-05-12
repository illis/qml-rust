use qobject::{QObjectContent, QObjectRefMut};
use qobject::qobjectrefmut;
use qlistmodel::{QListModel, QListModelContent};

impl<'a, T> From<&'a mut QListModel<T>> for QObjectRefMut<'a>
    where T: QObjectContent + QListModelContent {
    fn from(value: &'a mut QListModel<T>) -> Self {
        qobjectrefmut::new(super::qlistmodel::get_mut(value))
    }
}