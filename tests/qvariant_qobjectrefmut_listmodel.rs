extern crate libc;
#[macro_use]
extern crate qml;

use qml::*;

q_listmodel! {
    pub TestObject() => TestObjectSignals {
        signal fn value_changed();
        slot fn set_value(value: i32);
        slot fn get_value() -> i32;
        property value: i32, read: get_value;
        property value2: i32, read: get_value, notify: value_changed;
        property value3: i32, read: get_value, write: set_value, notify: value_changed;
    }
}

struct TestObject {
    signal_emitter: Box<QSignalEmitter>,
    value: i32,
}

impl TestObject {
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

impl QListModelContentConstructor for TestObject {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface>) -> Self {
        TestObject {
            signal_emitter: signal_emitter,
            value: 0,
        }
    }
}

#[test]
fn test_qvariant_qobjectrefmut_memory() {
    let mut qobject = QListModel::<TestObject>::new();
    QVariant::from(QObjectRefMut::from(&mut qobject));
}

#[test]
fn test_qvariant_qobjectrefmut_conversion() {
    let mut qobject = QListModel::<TestObject>::new();
    qobject.get_content_mut().set_value(123);

    let variant = QVariant::from(QObjectRefMut::from(&mut qobject));
    let mut qobjectref = QObjectRefMut::from(&variant);

    let qobject2 = qobjectref.as_content::<TestObject>().unwrap();
    assert_eq!(qobject2.get_value(), 123);
}