extern crate qml;

use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::os::raw::c_void;

use qml::*;

#[cfg(debug_assertions)]
#[test]
fn test_qobject_invoke_slot() {
    let mut qobject = QObject::<ObjectContent>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        assert!(unsafe { invoke_slot(qobjectref.as_cref_mut()) });
    }
    assert!(qobject.content().is_invoked());
}

#[cfg(debug_assertions)]
#[test]
fn test_qlistmdel_invoke_slot() {
    let mut qlistmodel = QListModel::<ListModelContent, ListModelItem>::new();
    {
        let mut qlistmodelref = QObjectRefMut::from(&mut qlistmodel);
        assert!(unsafe { invoke_slot(qlistmodelref.as_cref_mut()) });
    }
    assert!(qlistmodel.content().is_invoked());
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
    #[cfg(debug_assertions)]
    fn is_invoked(&self) -> bool {
        self.invoked
    }
}

impl ListModelContent {
    #[cfg(debug_assertions)]
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
    fn metaobject() -> QMetaObject {
        let paramters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot = SlotDefinition::new("test_slot", QMetaType::Int, paramters);
        QMetaObject::new_qobject("TestQObject", Vec::new(), vec![slot], Vec::new())
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, name, &args)
    }
}

impl QObjectContent for ListModelContent {
    fn metaobject() -> QMetaObject {
        let paramters = vec![ParameterDefinition::new("param", QMetaType::Int)];
        let slot = SlotDefinition::new("test_slot", QMetaType::Int, paramters);
        QMetaObject::new_qlistmodel("TestQObject", Vec::new(), vec![slot], Vec::new())
    }

    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
        do_invoke_slot(self, name, &args)
    }
}

impl QObjectContentConstructor for ObjectContent {
    fn new(_: Box<QSignalEmitter>) -> Self {
        ObjectContent { invoked: false }
    }
}

impl QListModelContentConstructor<ListModelItem> for ListModelContent {
    fn new(_: Box<QSignalEmitter>, _: Box<QListModelInterface<ListModelItem>>) -> Self {
        ListModelContent { invoked: false }
    }
}

impl QListModelItem for ListModelItem {
    fn role_names() -> Vec<&'static str> {
        vec![]
    }

    fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
        HashMap::new()
    }

    fn from_variant_map(_: QVariantMap) -> Self {
        ListModelItem {}
    }
}

fn do_invoke_slot<'a, T: InvokableContent>(
    instance: &mut T,
    name: &str,
    args: &[QVariantRefMut],
) -> Option<QVariant<'a>> {
    if name != "test_slot" {
        return None;
    }
    if args.len() != 1 {
        return None;
    }
    let arg0 = &args[0];
    let value: i32 = arg0.into();
    if value != 42 {
        return None;
    }
    instance.set_invoked();
    Some(QVariant::from(42))
}

extern "C" {
    #[cfg(debug_assertions)]
    fn invoke_slot(vptr: *mut c_void) -> bool;
}
