extern crate libc;
#[macro_use]
extern crate qml;

#[cfg(debug_assertions)]
use libc::c_void;
use qml::*;

q_object! {
    pub struct TestObject(signal_emitter: TestObjectSignals) {
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
            signal_emitter,
        }
    }
}

impl TestObject {
    fn callback(&mut self) {}

    fn send(&mut self) {
        self.event()
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "BorrowMutError")]
fn test_qobject_ownership() {
    let mut qobject = QObject::<TestObject>::new();
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        unsafe { connect_qobject_ownership(qobjectref.as_cref_mut()) };
    }
    {
        let mut qobjectref = QObjectRefMut::from(&mut qobject);
        unsafe { invoke_qobject_ownership_slot(qobjectref.as_cref_mut()) };
    }
}

extern "C" {
    #[cfg(debug_assertions)]
    fn connect_qobject_ownership(vptr: *mut c_void);
    #[cfg(debug_assertions)]
    fn invoke_qobject_ownership_slot(vptr: *mut c_void);
}