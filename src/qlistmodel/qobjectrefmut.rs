use qlistmodel::{QListModel, QListModelItem};
use qobject::{QObjectContent, QObjectRefMut};

impl<'a, T, I> From<&'a mut QListModel<T, I>> for QObjectRefMut<'a>
where
    T: QObjectContent,
    I: QListModelItem,
{
    fn from(value: &'a mut QListModel<T, I>) -> Self {
        QObjectRefMut::new(value.as_cref_mut())
    }
}
