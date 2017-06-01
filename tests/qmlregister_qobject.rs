extern crate libc;
#[macro_use]
extern crate qml;

use libc::c_void;
use qml::*;

q_object! {
    pub TestObject(signal_emitter:TestObjectSignals) {
        signal fn valueChanged(value: i32);
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: valueChanged;
        property value3: i32, read: get_value, write: set_value, notify: valueChanged;
    }
}

struct TestObject {
    signal_emitter: Box<QSignalEmitter>,
    value: i32,
}

impl TestObject {
    fn set_value(&mut self, value: i32) {
        self.value = value;
        self.valueChanged(value);
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

impl QObjectContentConstructor for TestObject {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
        TestObject {
            signal_emitter: signal_emitter,
            value: 0,
        }
    }
}

qml_register_qobject!(TestObject as QTestObject, "test.submodule", 1, 0);

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qmlregister_qobject() {
    qml_register_type::<TestObject>();
    unsafe { init_testresources(); }

    let mut view = QQuickView::new();
    let url = QUrl::new("qrc:///qml/tst_qmlregister_qobject.qml");


    view.load_url(url);
    view.exec();
}

extern "C" {
    fn init_testresources();
}
