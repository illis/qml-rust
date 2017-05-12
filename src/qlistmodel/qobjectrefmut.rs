use qobject::{QObjectContent, QObjectRefMut};
use qobject::qobjectrefmut;
use qlistmodel::QListModel;

impl<'a, T> From<&'a mut QListModel<T>> for QObjectRefMut<'a>
    where T: QObjectContent {
    fn from(value: &'a mut QListModel<T>) -> Self {
        qobjectrefmut::new(super::qlistmodel::get_mut(value))
    }
}