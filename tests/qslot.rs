extern crate qml;
extern crate libc;

use std::collections::hash_map::HashMap;
use libc::c_void;
use qml::*;

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qobject_invoke_slot() {
    let mut qobject = QObject::<ObjectContent>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        assert!(unsafe { invoke_slot(qobjectref.as_mut()) });
    }
    assert!(qobject.get_content().is_invoked());
}

#[test]
fn test_qlistmdel_invoke_slot() {
    let mut qlistmodel = QListModel::<ListModelContent, ListModelItem>::new();
    {
        let mut qlistmodelref = QObjectRefMut::from(&mut qlistmodel);
        assert!(unsafe { invoke_slot(qlistmodelref.as_mut()) });
    }
    assert!(qlistmodel.get_content().is_invoked());
}

struct ObjectContent {
    invoked: bool,
}

struct ListModelContent {
    invoked: bool,
}

trait InvokableContent {
    fn set_invoked(&mut self);
}

impl ObjectContent {
    fn is_invoked(&self) -> bool {
        self.invoked
    }
}

impl ListModelContent {
    fn is_invoked(&self) -> bool {
        self.invoked
    }
}

struct ListModelItem {}

impl InvokableContent for ObjectContent {
    fn set_invoked(&mut self) {
        self.invoked = true;
    }
}

impl InvokableContent for ListModelContent {
    fn set_invoked(&mut self) {
        self.invoked = true;
    }
}

impl QObjectContent for ObjectContent {
    fn get_metaobject() -> QMetaObject {
        let paramters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot = SlotDefinition::new("test_slot", QMetaType::Int, paramters);
        QMetaObject::new_qobject("TestQObject", Vec::new(), vec![slot], Vec::new())
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, &name, args)
    }
}

impl QObjectContent for ListModelContent {
    fn get_metaobject() -> QMetaObject {
        let paramters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot = SlotDefinition::new("test_slot", QMetaType::Int, paramters);
        QMetaObject::new_qlistmodel("TestQObject", Vec::new(), vec![slot], Vec::new())
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, &name, args)
    }
}

impl QObjectContentConstructor for ObjectContent {
    fn new(_: Box<QSignalEmitter>) -> Self {
        ObjectContent {
            invoked: false,
        }
    }
}

impl QListModelContentConstructor for ListModelContent {
    fn new(_: Box<QSignalEmitter>, _: Box<QListModelInterface>) -> Self {
        ListModelContent {
            invoked: false,
        }
    }
}

impl QListModelItem for ListModelItem {
    fn role_names() -> Vec<&'static str> {
        vec![]
    }

    fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
        HashMap::new()
    }

    fn from_variant_map<'a>(_: HashMap<&'static str, QVariant<'a>>) -> Self {
        ListModelItem {}
    }
}

fn do_invoke_slot<'a, T: InvokableContent>(instance: &mut T, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant<'a>> {
    if name != "test_slot" {
        return None;
    }
    if args.len() != 1 {
        return None;
    }
    let arg0 = &args[0];
    let value: i32 = arg0.into();
    if value != 42 {
        return None
    }
    instance.set_invoked();
    Some(QVariant::from(42))
}

extern "C" {
    fn invoke_slot(vptr: *mut c_void) -> bool;
}