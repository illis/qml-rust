extern crate libc;
#[macro_use]
extern crate qml;

#[cfg(debug_assertions)]
use libc::c_void;
use qml::*;

q_object! {
    pub struct TestObject(signal_emitter: TestObjectSignals) {
        signal fn value_changed(value: i32);
        slot fn set_value(value: i32);
        slot fn value() -> i32;
        property value: i32, read: value;
        property value2: i32, read: value, notify: value_changed;
        property value3: i32, read: value, write: set_value, notify: value_changed;
    }
}

struct TestObject {
    signal_emitter: Box<QSignalEmitter>,
}

impl TestObject {
    fn set_value(&mut self, _: i32) {}
    fn value(&self) -> i32 {
        123
    }
}

impl QObjectContentConstructor for TestObject {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
        TestObject {
            signal_emitter: signal_emitter,
        }
    }
}

#[cfg(debug_assertions)]
#[test]
fn test_qobject_macro_creates_correct_metatype() {
    let mut qobject = QObject::<TestObject>::new();
    let mut qobjectref = QObjectRefMut::from(&mut qobject);
    assert!(unsafe { check_metatype(qobjectref.as_cref_mut()) });
}

extern "C" {
    #[cfg(debug_assertions)]
    fn check_metatype(vptr: *const c_void) -> bool;
}