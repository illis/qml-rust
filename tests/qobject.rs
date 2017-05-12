extern crate qml;
extern crate libc;

use libc::{c_int, c_void};
use qml::*;

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qobject_set_value() {
    let mut qobject = QObject::<TestObject>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        unsafe { set_value(qobjectref.as_mut(), 42) };
    }
    assert_eq!(qobject.get_content().get_value(), 42);
}

#[test]
fn test_qlistmodel_set_value() {
    let mut qlistmodel = QListModel::<TestListModel>::new(vec!["first", "second"]);
    {
        let mut qobjectref = QObjectRefMut::from(&mut qlistmodel);
        unsafe { set_value(qobjectref.as_mut(), 42) };
    }
    assert_eq!(qlistmodel.get_content().get_value(), 42);
}

#[test]
fn test_qobject_value_changed() {
    let mut qobject = QObject::<TestObject>::new();
    let ptr = {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        qobjectref.as_mut() as *mut c_void
    };
    let spy = unsafe { create_value_changed_spy(ptr) };

    qobject.get_content_mut().set_value(42);
    assert_eq!(unsafe { value_changed_spy_get_value(spy) }, 42);
    unsafe { delete_value_changed_spy(spy); }
}

#[test]
fn test_qlistmodel_value_changed() {
    let mut qlistmodel = QListModel::<TestListModel>::new(vec!["first", "second"]);
    let ptr = {
        let mut qobjectref = QObjectRefMut::from(&mut qlistmodel);
        qobjectref.as_mut() as *mut c_void
    };
    let spy = unsafe { create_value_changed_spy(ptr) };

    qlistmodel.get_content_mut().set_value(42);
    assert_eq!(unsafe { value_changed_spy_get_value(spy) }, 42);
    unsafe { delete_value_changed_spy(spy); }
}

struct TestObject {
    signal_emitter: Box<QSignalEmitter>,
    value: i32,
}

struct TestListModel {
    signal_emitter: Box<QSignalEmitter>,
    value: i32,
}

trait QTestObjectSignals {
    fn value_changed(&mut self);
}

trait InvokableContent {
    fn set_value(&mut self, value: i32);
}

impl QTestObjectSignals for TestObject {
    fn value_changed(&mut self) {
        self.signal_emitter.emit_signal("valueChanged", vec![QVariant::from(self.value)]);
    }
}

impl QTestObjectSignals for TestListModel {
    fn value_changed(&mut self) {
        self.signal_emitter.emit_signal("valueChanged", vec![QVariant::from(self.value)]);
    }
}

impl InvokableContent for TestObject {
    fn set_value(&mut self, value: i32) {
        if self.value != value {
            self.value = value;
            self.value_changed();
        }
    }
}

impl InvokableContent for TestListModel {
    fn set_value(&mut self, value: i32) {
        if self.value != value {
            self.value = value;
            self.value_changed();
        }
    }
}

impl TestObject {
    fn get_value(&self) -> i32 {
        self.value
    }
}

impl TestListModel {
    fn get_value(&self) -> i32 {
        self.value
    }
}

impl QObjectContent for TestObject {
    fn get_metaobject() -> QMetaObject {
        let signal_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let signal_definitions = vec![SignalDefinition::new("valueChanged", signal_parameters)];
        let slot_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot_definitions = vec![SlotDefinition::new("setValue", QMetaType::Void, slot_parameters)];
        let properties_definitions = vec![];

        QMetaObject::new_qobject("QTestObject", signal_definitions, slot_definitions, properties_definitions)
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, &name, args)
    }
}

impl QObjectContent for TestListModel {
    fn get_metaobject() -> QMetaObject {
        let signal_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let signal_definitions = vec![SignalDefinition::new("valueChanged", signal_parameters)];
        let slot_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot_definitions = vec![SlotDefinition::new("setValue", QMetaType::Void, slot_parameters)];
        let properties_definitions = vec![];

        QMetaObject::new_qlistmodel("QTestListModel", signal_definitions, slot_definitions, properties_definitions)
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, &name, args)
    }
}

impl QObjectContentConstructor for TestObject {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
        TestObject {
            signal_emitter: signal_emitter,
            value: 123,
        }
    }
}

impl QListModelContentConstructor for TestListModel {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface>) -> Self {
        TestListModel {
            signal_emitter: signal_emitter,
            value: 123,
        }
    }
}

fn do_invoke_slot<'a, T: InvokableContent>(instance: &mut T, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant<'a>> {
    if name != "setValue" {
        return None;
    }
    if args.len() != 1 {
        return None;
    }
    let arg0 = &args[0];
    instance.set_value(arg0.into());
    None
}

extern "C" {
    fn set_value(vptr: *mut c_void, value: c_int);
    fn create_value_changed_spy(vptr: *mut c_void) -> *mut c_void;
    fn delete_value_changed_spy(vptr: *mut c_void);
    fn value_changed_spy_get_value(ptr: *const c_void) -> c_int;
}