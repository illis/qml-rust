extern crate libc;
#[macro_use]
extern crate qml;

use libc::c_void;
use qml::*;

q_listmodel! {
    pub TestListModel() => TestListModelSignals {
        signal fn value_changed(value: i32);
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: value_changed;
        property value3: i32, read: get_value, write: set_value, notify: value_changed;
    }
}

struct TestListModel {
    signal_emitter: Box<QSignalEmitter>,
}

impl TestListModel {
    fn set_value(&mut self, _: i32) {}
    fn get_value(&self) -> i32 {
        123
    }
}

impl QListModelContentConstructor for TestListModel {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface>) -> Self {
        TestListModel {
            signal_emitter: signal_emitter,
        }
    }
}

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qobject_macro_creates_correct_metatype() {
    let mut qobject = QListModel::<TestListModel>::new();
    let mut qobjectref = QObjectRefMut::from(&mut qobject);
    assert!(unsafe { check_metatype(qobjectref.as_mut()) });
}

extern "C" {
    fn check_metatype(vptr: *const c_void) -> bool;
}