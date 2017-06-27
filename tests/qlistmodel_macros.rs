extern crate libc;
#[macro_use]
extern crate qml;

use std::collections::HashMap;
#[cfg(debug_assertions)]
use libc::c_void;
use qml::*;

q_listmodel! {
    pub struct TestListModel(signal_emitter: TestListModelSignals) {
        signal fn value_changed(value: i32);
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: value_changed;
        property value3: i32, read: get_value, write: set_value, notify: value_changed;
    }
}

q_listmodelitem! {
    pub struct TestListModelItem {
        number: i32,
        string: String,
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

impl QListModelContentConstructor<TestListModelItem> for TestListModel {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface<TestListModelItem>>) -> Self {
        TestListModel {
            signal_emitter: signal_emitter,
        }
    }
}

#[cfg(debug_assertions)]
#[test]
fn test_qobject_macro_creates_correct_metatype() {
    let mut qobject = QListModel::<TestListModel, TestListModelItem>::new();
    let mut qobjectref = QObjectRefMut::from(&mut qobject);
    assert!(unsafe { check_metatype(qobjectref.as_mut()) });
}

extern "C" {
    #[cfg(debug_assertions)]
    fn check_metatype(vptr: *const c_void) -> bool;
}