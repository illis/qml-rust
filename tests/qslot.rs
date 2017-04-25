extern crate qml;
extern crate libc;

use libc::c_void;
use qml::{QMetaType, QMetaObject, QObject, QObjectContent, QSignalEmitter, QVariant, QVariantView, SlotDefinition};

#[link(name = "testresources", kind = "static")]
#[test]
fn test_qobject_invoke_slot() {
    let mut qobject = QObject::<Content>::new();
    assert!(unsafe { invoke_slot(qobject.as_mut()) });
    assert!(qobject.get_content().is_invoked());
}

struct Content {
    invoked: bool
}

impl Content {
    fn set_invoked(&mut self) {
        self.invoked = true;
    }
    fn is_invoked(&self) -> bool {
        self.invoked
    }
}

impl QObjectContent for Content {
    fn new(_: Box<QSignalEmitter>) -> Self {
        Content {
            invoked: false
        }
    }

    fn get_metatype() -> QMetaObject {
        let slot = SlotDefinition::new("test_slot", QMetaType::Int, vec![QMetaType::Int]);
        QMetaObject::new_qobject("TestQObject", Vec::new(), vec![slot], Vec::new())
    }
    fn invoke_slot(&mut self, name: &str, args: Vec<QVariantView>) -> Option<QVariant> {
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
        self.set_invoked();
        Some(QVariant::from(42))
    }
}


extern "C" {
    fn invoke_slot(vptr: *mut c_void) -> bool;
}
