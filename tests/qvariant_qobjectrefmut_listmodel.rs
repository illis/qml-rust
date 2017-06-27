extern crate libc;
#[macro_use]
extern crate qml;

use std::collections::HashMap;
use qml::*;

q_listmodel! {
    pub struct TestListModel(signal_emitter: TestObjectSignals) {
        signal fn value_changed();
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: value_changed;
        property value3: i32, read: get_value, write: set_value, notify: value_changed;
    }
}

struct TestListModel {
    signal_emitter: Box<QSignalEmitter>,
    value: i32,
}
struct TestListModelItem {}

impl TestListModel {
    fn set_value(&mut self, value: i32) {
        if self.value != value {
            self.value = value;
            self.value_changed();
        }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

impl QListModelItem for TestListModelItem {
    fn role_names() -> Vec<&'static str> {
        vec![]
    }

    fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
        HashMap::new()
    }

    fn from_variant_map<'a>(_: QVariantMap<'a>) -> Self {
        TestListModelItem {}
    }
}

impl QListModelContentConstructor<TestListModelItem> for TestListModel {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface<TestListModelItem>>) -> Self {
        TestListModel {
            signal_emitter: signal_emitter,
            value: 0,
        }
    }
}

#[test]
fn test_qvariant_qobjectrefmut_listmodel_memory() {
    let mut qobject = QListModel::<TestListModel, TestListModelItem>::new();
    QVariant::from(QObjectRefMut::from(&mut qobject));
}

#[test]
fn test_qvariant_qobjectrefmut_listmodel_conversion() {
    let mut qobject = QListModel::<TestListModel, TestListModelItem>::new();
    qobject.get_content_mut().set_value(123);

    let variant = QVariant::from(QObjectRefMut::from(&mut qobject));
    let mut qobjectref = QObjectRefMut::from(&variant);

    let qobject2_refcell = qobjectref.as_content::<TestListModel>().unwrap();
    let qobject2 = qobject2_refcell.borrow();
    assert_eq!(qobject2.get_value(), 123);
}