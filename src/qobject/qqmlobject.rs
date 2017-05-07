use std::ffi::CString;
use std::marker::PhantomData;
use libc::{c_char, c_int, c_void};
use qobject::{QObject, QObjectContent, QObjectContentConstructor, QSignalEmitter};
use qvariant;
use qvariant::QVariant;

pub struct QQmlObject<T: QObjectContent + QObjectContentConstructor> {
    _phantom: PhantomData<T>,
}

impl<T: QObjectContent + QObjectContentConstructor> QQmlObject<T> {
    pub fn new(wrapper: *mut c_void) -> QObject<T> {
        super::qobject::new_with_signal_emitter(Box::new(SignalEmitter::new(wrapper)))
    }
}

struct SignalEmitter {
    ptr: *mut c_void,
}

impl SignalEmitter {
    pub fn new(ptr: *mut c_void) -> SignalEmitter {
        SignalEmitter {
            ptr: ptr,
        }
    }
}

impl QSignalEmitter for SignalEmitter {
    fn emit_signal(&mut self, name: &str, mut args: Vec<QVariant>) {
        let string = CString::new(name).unwrap();
        let mut args: Vec<*mut c_void> = args.iter_mut()
            .map(|item| qvariant::qvariant::get_mut(item))
            .collect();

        unsafe {
            dos_qobject_signal_emit(self.ptr, string.as_ptr(), args.len() as c_int,
                                    args.as_mut_ptr());
        }
    }
}

extern "C" {
    fn dos_qobject_signal_emit(vptr: *mut c_void, name: *const c_char,
                               argc: c_int, argv: *mut *mut c_void);
}