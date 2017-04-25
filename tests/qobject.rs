extern crate qml;
extern crate libc;

use libc::{c_int, c_void};
use qml::{QObject, QMetaObject, QObjectContent, QMetaType, QSignalEmitter, QVariant, QVariantView, SignalDefinition, SlotDefinition};

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qobject_set_value() {
    let mut qobject = QObject::<TestObject>::new();
    unsafe {
        set_value(qobject.as_mut(), 42)
    };

    assert_eq!(qobject.get_content().get_value(), 42);
}

#[test]
fn test_qobject_value_changed() {
    let mut qobject = QObject::<TestObject>::new();
    let spy = unsafe { create_value_changed_spy(qobject.as_mut()) };

    qobject.get_content_mut().set_value(42);
    assert_eq!(unsafe { value_changed_spy_get_value(spy) }, 42);
    unsafe {
        delete_value_changed_spy(spy);
    }
}

struct TestObject {
    signal_invoker: Box<QSignalEmitter>,
    value: i32,
}

trait QTestObjectSignals {
    fn value_changed(&mut self);
}

impl QTestObjectSignals for TestObject {
    fn value_changed(&mut self) {
        self.signal_invoker.emit_signal("valueChanged", vec![QVariant::from(self.value)]);
    }
}

impl TestObject {
    fn get_value(&self) -> i32 {
        self.value
    }

    fn set_value(&mut self, value: i32) {
        if self.value != value {
            self.value = value;
            self.value_changed();
        }
    }
}

impl QObjectContent for TestObject {
    fn new(signal_invoker: Box<QSignalEmitter>) -> Self {
        TestObject {
            signal_invoker: signal_invoker,
            value: 123,
        }
    }

    fn get_metatype() -> QMetaObject {
        let signal_definitions = vec![SignalDefinition::new("valueChanged", vec![QMetaType::Int])];
        let slot_definitions = vec![SlotDefinition::new("setValue", QMetaType::Void, vec![QMetaType::Int])];
        let properties_definitions = vec![];

        QMetaObject::new_qobject("QTestObject", signal_definitions, slot_definitions, properties_definitions)
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantView>) -> Option<QVariant> {
        if name != "setValue" {
            return None;
        }
        if args.len() != 1 {
            return None;
        }
        let arg0 = &args[0];
        self.set_value(arg0.into());
        None
    }
}

extern "C" {
    fn set_value(vptr: *mut c_void, value: c_int);
    fn create_value_changed_spy(vptr: *mut c_void) -> *mut c_void;
    fn delete_value_changed_spy(vptr: *mut c_void);
    fn value_changed_spy_get_value(ptr: *const c_void) -> c_int;
}