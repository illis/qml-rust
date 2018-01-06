extern crate qml;

use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::os::raw::{c_int, c_void};
use qml::*;

#[cfg(debug_assertions)]
#[test]
fn test_qobject_set_value() {
    let mut qobject = QObject::<TestObject>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        unsafe { set_value(qobjectref.as_cref_mut(), 42) };
    }
    assert_eq!(qobject.content().value(), 42);
}

#[cfg(debug_assertions)]
#[test]
fn test_qlistmodel_set_value() {
    let mut qlistmodel = QListModel::<TestListModel, TestListModelItem>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qlistmodel);
        unsafe { set_value(qobjectref.as_cref_mut(), 42) };
    }
    assert_eq!(qlistmodel.content().value(), 42);
}

#[cfg(debug_assertions)]
#[test]
fn test_qobject_value_changed() {
    let mut qobject = QObject::<TestObject>::new();
    let ptr = {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        qobjectref.as_cref_mut() as *mut c_void
    };
    let spy = unsafe { create_value_changed_spy(ptr) };

    qobject.content_mut().set_value(42);
    assert_eq!(unsafe { value_changed_spy_get_value(spy) }, 42);
    unsafe { delete_value_changed_spy(spy); }
}

#[cfg(debug_assertions)]
#[test]
fn test_qlistmodel_value_changed() {
    let mut qlistmodel = QListModel::<TestListModel, TestListModelItem>::new();
    let ptr = {
        let mut qobjectref = QObjectRefMut::from(&mut qlistmodel);
        qobjectref.as_cref_mut() as *mut c_void
    };
    let spy = unsafe { create_value_changed_spy(ptr) };

    qlistmodel.content_mut().set_value(42);
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

struct TestListModelItem {}

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
    #[cfg(debug_assertions)]
    fn value(&self) -> i32 {
        self.value
    }
}

impl TestListModel {
    #[cfg(debug_assertions)]
    fn value(&self) -> i32 {
        self.value
    }
}

impl QObjectContent for TestObject {
    fn metaobject() -> QMetaObject {
        let signal_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let signal_definitions = vec![SignalDefinition::new("valueChanged", signal_parameters)];
        let slot_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot_definitions = vec![SlotDefinition::new("setValue", QMetaType::Void, slot_parameters)];
        let properties_definitions = vec![];

        QMetaObject::new_qobject("QTestObject", signal_definitions, slot_definitions, properties_definitions)
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, name, &args)
    }
}

impl QObjectContent for TestListModel {
    fn metaobject() -> QMetaObject {
        let signal_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let signal_definitions = vec![SignalDefinition::new("valueChanged", signal_parameters)];
        let slot_parameters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot_definitions = vec![SlotDefinition::new("setValue", QMetaType::Void, slot_parameters)];
        let properties_definitions = vec![];

        QMetaObject::new_qlistmodel("QTestListModel", signal_definitions, slot_definitions, properties_definitions)
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, name, &args)
    }
}

impl QObjectContentConstructor for TestObject {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
        TestObject {
            signal_emitter,
            value: 123,
        }
    }
}

impl QListModelContentConstructor<TestListModelItem> for TestListModel {
    fn new(signal_emitter: Box<QSignalEmitter>, _: Box<QListModelInterface<TestListModelItem>>) -> Self {
        TestListModel {
            signal_emitter,
            value: 123,
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

fn do_invoke_slot<'a, T: InvokableContent>(instance: &mut T, name: &str, args: &[QVariantRefMut]) -> Option<QVariant<'a>> {
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
    #[cfg(debug_assertions)]
    fn set_value(vptr: *mut c_void, value: c_int);
    #[cfg(debug_assertions)]
    fn create_value_changed_spy(vptr: *mut c_void) -> *mut c_void;
    #[cfg(debug_assertions)]
    fn delete_value_changed_spy(vptr: *mut c_void);
    #[cfg(debug_assertions)]
    fn value_changed_spy_get_value(ptr: *const c_void) -> c_int;
}