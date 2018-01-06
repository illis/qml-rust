use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

use internal::QObjectWeakPtr;
use qobject::QSignalEmitter;
use qvariant::QVariant;

pub(crate) struct QObjectSignalEmitter {
    ptr: QObjectWeakPtr,
}

impl QObjectSignalEmitter {
    pub(crate) fn new(ptr: QObjectWeakPtr) -> Self {
        QObjectSignalEmitter { ptr }
    }
}

impl QSignalEmitter for QObjectSignalEmitter {
    fn emit_signal(&self, name: &str, mut args: Vec<QVariant>) {
        let string = CString::new(name).unwrap();
        let mut args: Vec<*mut c_void> = args.iter_mut()
            .map(|item| item.as_cref_mut() as *mut c_void)
            .collect();

        self.ptr.upgrade().map(|ptr| {
            let ptr = ptr.borrow_mut().as_cref_mut();

            unsafe {
                dos_qobject_signal_emit(
                    ptr,
                    string.as_ptr(),
                    args.len() as c_int,
                    args.as_mut_ptr(),
                );
            }
        });
    }
}

extern "C" {
    fn dos_qobject_signal_emit(
        vptr: *mut c_void,
        name: *const c_char,
        argc: c_int,
        argv: *mut *mut c_void,
    );
}
