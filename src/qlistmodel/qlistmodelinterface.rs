use qlistmodel::QListModelItem;

pub trait QListModelInterface<I>
where
    I: QListModelItem,
{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, item: I);
    fn append(&mut self, items: &[I]);
    fn insert(&mut self, row: usize, item: I);
    fn remove(&mut self, row: usize);
    fn drain(&mut self, begin: usize, end: usize);
    fn clear(&mut self);
    fn get(&self, index: usize) -> Option<I>;
    fn as_list(&self) -> Vec<I>;
}
