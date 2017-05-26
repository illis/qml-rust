extern crate libc;
#[macro_use]
extern crate qml;

use libc::c_void;
use qml::*;

q_object! {
    pub TestObject => TestObjectSignals {
        signal fn event();
        slot fn callback();
        slot fn send();
    }
}

struct TestObject {
    signal_emitter: Box<QSignalEmitter>,
}

impl QObjectContentConstructor for TestObject {
    fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
        TestObject {
            signal_emitter: signal_emitter,
        }
    }
}

impl TestObject {
    fn callback(&mut self) {}

    fn send(&mut self) {
        self.event()
    }
}

#[test]
#[should_panic(expected = "BorrowMutError")]
fn test_qobject_ownership() {
    let mut qobject = QObject::<TestObject>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        unsafe { connect_qobject_ownership(qobjectref.as_mut()) };
    }
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        unsafe { invoke_qobject_ownership_slot(qobjectref.as_mut()) };
    }
}

extern "C" {
    fn connect_qobject_ownership(vptr: *mut c_void);
    fn invoke_qobject_ownership_slot(vptr: *mut c_void);
}