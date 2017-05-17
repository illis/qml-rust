use std::ffi::CString;
use libc::{c_char, c_int, c_void};
use qobject::QSignalEmitter;
use qvariant;
use qvariant::QVariant;

pub struct QQmlObjectSignalEmitter {
    ptr: *mut c_void,
}

impl QQmlObjectSignalEmitter {
    pub fn new(ptr: *mut c_void) -> QQmlObjectSignalEmitter {
        QQmlObjectSignalEmitter {
            ptr: ptr,
        }
    }
}

impl QSignalEmitter for QQmlObjectSignalEmitter {
    fn emit_signal(&self, name: &str, mut args: Vec<QVariant>) {
        let string = CString::new(name).unwrap();
        let mut args: Vec<*mut c_void> = args.iter_mut()
            .map(|item| qvariant::qvariant::get_mut(item) as *mut c_void)
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