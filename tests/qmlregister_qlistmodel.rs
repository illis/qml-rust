extern crate libc;
#[macro_use]
extern crate qml;

use libc::c_void;
use qml::*;

q_listmodel! {
    pub TestListModel(signal_emitter: TestListModelSignals, role_names: first, second) {
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

impl TestListModel {
    fn set_value(&mut self, value: i32) {
        self.value = value;
        self.valueChanged(value);
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

impl QListModelContentConstructor for TestListModel {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface>) -> Self {
        TestListModel {
            signal_emitter: signal_emitter,
            value: 0,
        }
    }
}

qml_register_qlistmodel!(TestListModel as QTestListModel, "test.submodule", 1, 0);

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qmlregister_qlistmodel() {
    qml_register_type::<TestListModel>();
    unsafe { init_testresources(); }

    let mut view = QQuickView::new();
    let url = QUrl::new("qrc:///qml/tst_qmlregister_qlistmodel.qml");


    view.load_url(url);
    view.exec();
}

extern "C" {
    fn init_testresources();
}
