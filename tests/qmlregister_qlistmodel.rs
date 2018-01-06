/*
#[macro_use]
extern crate qml;

use std::collections::HashMap;
use std::os::raw::c_void;

use qml::*;

q_listmodel! {
    pub struct TestListModel(signal_emitter: TestListModelSignals) {
        signal fn valueChanged(value: i32);
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: valueChanged;
        property value3: i32, read: get_value, write: set_value, notify: valueChanged;
    }
}

struct TestListModel {
    signal_emitter: Box<QSignalEmitter>,
    value: i32,
}

struct TestListModelItem {}

impl TestListModel {
    fn set_value(&mut self, value: i32) {
        self.value = value;
        self.valueChanged(value);
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

impl QListModelContentConstructor<TestListModelItem> for TestListModel {
    fn new(
        signal_emitter: Box<QSignalEmitter>,
        _: Box<QListModelInterface<TestListModelItem>>,
    ) -> Self {
        TestListModel {
            signal_emitter,
            value: 0,
        }
    }
}

impl QListModelItem for TestListModelItem {
    fn role_names() -> Vec<&'static str> {
        vec![]
    }

    fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
        HashMap::new()
    }

    fn from_variant_map(_: QVariantMap) -> Self {
        TestListModelItem {}
    }
}

qml_register_qlistmodel!(TestListModel<TestListModelItem> as QTestListModel, "test.submodule", 1, 0);

#[cfg(debug_assertions)]
#[test]
fn test_qmlregister_qlistmodel() {
    qml_register_type::<TestListModel>();
    unsafe {
        init_testresources();
    }

    let mut view = QQuickView::new();
    let url = QUrl::new("qrc:///qml/tst_qmlregister_qlistmodel.qml").unwrap();

    view.load_url(url);
    view.exec();
}

extern "C" {
    #[cfg(debug_assertions)]
    fn init_testresources();
}
*/
