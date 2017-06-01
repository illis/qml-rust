extern crate libc;
#[macro_use]
extern crate qml;

use qml::*;

q_object! {
    pub TestObject(signal_emitter: TestObjectSignals) {
        signal fn value_changed(value: i32);
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: value_changed;
        property value3: i32, read: get_value, write: set_value, notify: value_changed;
    }
}

struct TestObject {
    signal_emitter: Box<QSignalEmitter>,
}

impl TestObject {
    fn set_value(&mut self, _: i32) {}
    fn get_value(&self) -> i32 {
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

q_object! {
    pub TestObject2(signal_emitter: TestObject2Signals) {}
}

struct TestObject2 {
    signal_emitter: Box<QSignalEmitter>,
}

impl QObjectContentConstructor for TestObject2 {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
        TestObject2 {
            signal_emitter: signal_emitter,
        }
    }
}

#[test]
fn test_qobjectrefmut_can_convert_to_content() {
    let mut qobject = QObject::<TestObject>::new();
    let mut qobjectref = QObjectRefMut::from(&mut qobject);

    let content = qobjectref.as_content::<TestObject>();
    assert!(content.is_some());
}

#[test]
fn test_qobjectrefmut_cannot_convert_to_different_content() {
    let mut qobject = QObject::<TestObject>::new();
    let mut qobjectref = QObjectRefMut::from(&mut qobject);

    let content = qobjectref.as_content::<TestObject2>();
    assert!(content.is_none());
}