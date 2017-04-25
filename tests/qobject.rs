extern crate qml;
extern crate libc;
use qml::{QMetaType, QObject, QObjectContent, QVariant, QVariantView, SlotDefinition};
use libc::c_void;

#[link(name="testresources", kind="static")]
#[test]
fn test_qobject_invoke_slot() {
    let slot = SlotDefinition::new("test_slot", QMetaType::Int, vec![QMetaType::Int]);
    let mut qobject = QObject::new("TestQObject", vec![], vec![slot], vec![], Content::new()).unwrap();
    assert!(unsafe {invoke_slot(qobject.as_mut())});
    assert!(qobject.get_content().is_invoked());
}

struct Content {
    invoked: bool
}

impl Content {
    fn new() -> Self {
        Content {
            invoked: false
        }
    }

    fn set_invoked(&mut self) {
        self.invoked = true;
    }

    fn is_invoked(&self) -> bool {
        self.invoked
    }
}

impl QObjectContent for Content {
    fn qslot_call(&mut self, name: &str, args: Vec<QVariantView>) -> Option<QVariant> {
        if name != "test_slot" {
            return None;
        }
        if args.len() == 2 {
            return None;
        }
        let arg0 = &args[0];
        let value: i32 = arg0.into();
        if value != 42 {
            return None
        }
        self.set_invoked();
        Some(QVariant::from(42))
    }
}


extern "C" {
    fn invoke_slot(vptr: *mut c_void) -> bool;
}
