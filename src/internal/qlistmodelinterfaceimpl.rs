use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_void};
use std::slice::from_raw_parts_mut;

use internal::{
    c_entries_to_c_map,
    entries_to_c_entries,
    static_variantmap_to_entries,
    CQVariantMap,
    CQVariantMapWrapper,
};
use internal::QObjectWeakPtr;
use qlistmodel::{QListModelInterface, QListModelItem};
use qvariant::{QVariant, QVariantMap};

pub(crate) struct QListModelInterfaceImpl<I>
where
    I: QListModelItem,
{
    ptr: QObjectWeakPtr,
    _phantom: PhantomData<I>,
}

impl<I> QListModelInterfaceImpl<I>
where
    I: QListModelItem,
{
    pub(crate) fn new(ptr: QObjectWeakPtr) -> Self {
        QListModelInterfaceImpl {
            ptr,
            _phantom: PhantomData,
        }
    }

    fn insert_list(&mut self, row: usize, items: Vec<I>) {
        let ptr = self.ptr.upgrade().unwrap();
        let ptr = ptr.borrow_mut().as_cref_mut();

        let variant_maps = items
            .iter()
            .map(|item| item.to_variant_map())
            .collect::<Vec<_>>();
        let entries = variant_maps
            .iter()
            .map(|item| static_variantmap_to_entries(item))
            .collect::<Vec<_>>();
        let mut c_entries = entries
            .iter()
            .map(|item| entries_to_c_entries(item))
            .collect::<Vec<_>>();
        let c_maps = c_entries
            .iter_mut()
            .map(|mut item| c_entries_to_c_map(&mut item))
            .collect::<Vec<_>>();
        let c_map_list = CQVariantMapList {
            count: c_maps.len() as c_int,
            values: c_maps.as_ptr(),
        };
        unsafe { de_qlistmodel_insert(ptr, row as c_int, &c_map_list) }
    }
}

impl<I> QListModelInterface<I> for QListModelInterfaceImpl<I>
where
    I: QListModelItem,
{
    fn len(&self) -> usize {
        let ptr = self.ptr.upgrade().unwrap();
        let ptr = ptr.borrow().as_cref();
        unsafe { de_qlistmodel_count(ptr) as usize }
    }

    fn is_empty(&self) -> bool {
        let ptr = self.ptr.upgrade().unwrap();
        let ptr = ptr.borrow().as_cref();
        unsafe { de_qlistmodel_empty(ptr) }
    }

    fn push(&mut self, item: I) {
        let len = self.len();
        self.insert_list(len, vec![item]);
    }

    fn append(&mut self, items: Vec<I>) {
        let len = self.len();
        self.insert_list(len, items);
    }

    fn insert(&mut self, row: usize, item: I) {
        self.insert_list(row, vec![item]);
    }

    fn remove(&mut self, row: usize) {
        self.drain(row, row + 1)
    }

    fn drain(&mut self, begin: usize, end: usize) {
        let ptr = self.ptr.upgrade().unwrap();
        let ptr = ptr.borrow_mut().as_cref_mut();
        unsafe { de_qlistmodel_remove(ptr, begin as c_int, (end - begin) as c_int) }
    }

    fn clear(&mut self) {
        let len = self.len();
        self.drain(0, len)
    }

    fn get(&self, index: usize) -> Option<I> {
        let ptr = self.ptr.upgrade().unwrap();
        let ptr = ptr.borrow().as_cref();
        CQVariantMapWrapper::from_model(&*ptr, index)
            .map(|wrapper| {
                let slice =
                    unsafe { from_raw_parts_mut(wrapper.ptr.values, wrapper.ptr.count as usize) };

                slice
                    .iter()
                    .map(|value| {
                        let key = unsafe { CStr::from_ptr(value.key) };
                        let key = key.to_string_lossy().into_owned();
                        let value = QVariant::new(unsafe {
                            dos_qvariant_create_qvariant(value.value).as_mut().unwrap()
                        });
                        (key, value)
                    })
                    .collect::<QVariantMap>()
            })
            .map(I::from_variant_map)
    }

    fn as_list(&self) -> Vec<I> {
        let len = self.len();
        (0..len)
            .map(|index| self.get(index).unwrap())
            .collect::<Vec<_>>()
    }
}

#[repr(C)]
pub(crate) struct CQVariantMapList {
    count: c_int,
    values: *const CQVariantMap,
}

impl<'a> CQVariantMapWrapper<'a> {
    fn from_model(model: &'a c_void, index: usize) -> Option<Self> {
        unsafe { de_qlistmodel_get(model, index as c_int).as_mut() }
            .map(|ptr| CQVariantMapWrapper { ptr })
    }
}

extern "C" {
    fn de_qlistmodel_count(vptr: *const c_void) -> c_int;
    fn de_qlistmodel_empty(vptr: *const c_void) -> bool;
    fn de_qlistmodel_insert(vptr: *mut c_void, row: c_int, values: *const CQVariantMapList);
    fn de_qlistmodel_remove(vptr: *mut c_void, row: c_int, count: c_int);
    fn de_qlistmodel_get(vptr: *const c_void, index: c_int) -> *mut CQVariantMap;
    fn dos_qvariant_create_qvariant(value: *const c_void) -> *mut c_void;
}

#[cfg(test)]
mod tests {
    use qlistmodel::{QListModel, QListModelContentConstructor, QListModelInterface, QListModelItem};
    use qmetaobject::QMetaObject;
    use qobject::{QObjectContent, QSignalEmitter};
    use qvariant::{QVariant, QVariantMap, QVariantRefMut};
    use std::collections::HashMap;

    struct Content {
        interface: Box<QListModelInterface<Item>>,
    }

    #[derive(Debug)]
    struct Item {
        number: i32,
        string: String,
    }

    impl Content {
        fn len(&self) -> usize {
            self.interface.len()
        }

        fn push(&mut self, item: Item) {
            self.interface.push(item)
        }

        fn append(&mut self, item: Vec<Item>) {
            self.interface.append(item)
        }

        fn insert(&mut self, row: usize, item: Item) {
            self.interface.insert(row, item)
        }

        fn remove(&mut self, row: usize) {
            self.interface.remove(row)
        }

        fn drain(&mut self, begin: usize, end: usize) {
            self.interface.drain(begin, end)
        }

        fn clear(&mut self) {
            self.interface.clear()
        }

        fn get(&self, index: usize) -> Option<Item> {
            self.interface.get(index)
        }

        fn as_list(&self) -> Vec<Item> {
            self.interface.as_list()
        }
    }

    impl Item {
        fn new(number: i32, string: String) -> Self {
            Item { number, string }
        }
    }

    impl PartialEq for Item {
        fn eq(&self, other: &Item) -> bool {
            self.number == other.number && self.string == other.string
        }
    }

    impl QObjectContent for Content {
        fn metaobject() -> QMetaObject {
            QMetaObject::new_qobject("Meta", Vec::new(), Vec::new(), Vec::new())
        }

        fn invoke_slot(&mut self, _: &str, _: Vec<QVariantRefMut>) -> Option<QVariant> {
            None
        }
    }

    impl QListModelContentConstructor<Item> for Content {
        fn new(_: Box<QSignalEmitter>, interface: Box<QListModelInterface<Item>>) -> Self {
            Content { interface }
        }
    }

    impl QListModelItem for Item {
        fn role_names() -> Vec<&'static str> {
            vec!["number", "string"]
        }

        fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
            let mut returned = HashMap::new();
            returned.insert("number", QVariant::from(self.number));
            returned.insert("string", QVariant::from(&self.string));
            returned
        }

        fn from_variant_map(input: QVariantMap) -> Self {
            Item::new(
                input.get("number").unwrap().into(),
                input.get("string").unwrap().into(),
            )
        }
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_len_0() {
        let model = QListModel::<Content, Item>::new();
        assert_eq!(model.content().len(), 0);
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_len_1() {
        let mut model = QListModel::<Content, Item>::new();
        model
            .content_mut()
            .append(vec![Item::new(123, String::from("abc"))]);
        assert_eq!(model.content().len(), 1);
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_len_3() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
            Item::new(345, String::from("ghi")),
        ]);
        assert_eq!(model.content().len(), 3);
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_clear() {
        let mut model = QListModel::<Content, Item>::new();
        model
            .content_mut()
            .append(vec![Item::new(123, String::from("abc"))]);
        model.content_mut().clear();
        assert_eq!(model.content().len(), 0);
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_get() {
        let mut model = QListModel::<Content, Item>::new();
        model
            .content_mut()
            .append(vec![Item::new(123, String::from("abc"))]);
        assert_eq!(
            model.content().get(0),
            Some(Item::new(123, String::from("abc")))
        );
        assert_eq!(model.content().get(1), None);
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_push() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model
            .content_mut()
            .push(Item::new(345, String::from("ghi")));

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(123, String::from("abc")),
                Item::new(234, String::from("def")),
                Item::new(345, String::from("ghi")),
            ]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_insert_middle() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model
            .content_mut()
            .insert(1, Item::new(345, String::from("ghi")));

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(123, String::from("abc")),
                Item::new(345, String::from("ghi")),
                Item::new(234, String::from("def")),
            ]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_insert_beginning() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model
            .content_mut()
            .insert(0, Item::new(345, String::from("ghi")));

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(345, String::from("ghi")),
                Item::new(123, String::from("abc")),
                Item::new(234, String::from("def")),
            ]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_insert_end() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model
            .content_mut()
            .insert(2, Item::new(345, String::from("ghi")));

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(123, String::from("abc")),
                Item::new(234, String::from("def")),
                Item::new(345, String::from("ghi")),
            ]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_insert_after_end() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model
            .content_mut()
            .insert(3, Item::new(345, String::from("ghi")));

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(123, String::from("abc")),
                Item::new(234, String::from("def")),
            ]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_remove_0() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model.content_mut().remove(0);

        assert_eq!(
            model.content().as_list(),
            vec![Item::new(234, String::from("def"))]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_remove_1() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model.content_mut().remove(1);

        assert_eq!(
            model.content().as_list(),
            vec![Item::new(123, String::from("abc"))]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_remove_after_end() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model.content_mut().remove(2);

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(123, String::from("abc")),
                Item::new(234, String::from("def")),
            ]
        );
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_drain() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model.content_mut().drain(0, 2);

        assert_eq!(model.content().as_list(), vec![]);
    }

    #[test]
    fn test_qlistmodelinterfaceimpl_drain_after_end() {
        let mut model = QListModel::<Content, Item>::new();
        model.content_mut().append(vec![
            Item::new(123, String::from("abc")),
            Item::new(234, String::from("def")),
        ]);
        model.content_mut().drain(0, 3);

        assert_eq!(
            model.content().as_list(),
            vec![
                Item::new(123, String::from("abc")),
                Item::new(234, String::from("def")),
            ]
        );
    }
}
