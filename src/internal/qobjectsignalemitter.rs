use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Weak;
use libc::{c_char, c_int, c_void};
use internal::QObjectPtr;
use qobject::QSignalEmitter;
use qvariant;
use qvariant::QVariant;

pub(crate) struct QObjectSignalEmitter {
    ptr: QObjectWeakPtr,
}

impl QObjectSignalEmitter {
    pub fn new(ptr: QObjectWeakPtr) -> Self {
        QObjectSignalEmitter {
            ptr: ptr,
        }
    }
}

impl QSignalEmitter for QObjectSignalEmitter {
    fn emit_signal(&self, name: &str, mut args: Vec<QVariant>) {
        let string = CString::new(name).unwrap();
        let mut args: Vec<*mut c_void> = args.iter_mut()
            .map(|item| qvariant::qvariant::get_mut(item) as *mut c_void)
            .collect();

        self.ptr.upgrade().and_then::<(), _>(|ptr| {
            let ptr = ptr.borrow_mut().as_mut();

            unsafe {
                dos_qobject_signal_emit(ptr, string.as_ptr(), args.len() as c_int,
                                        args.as_mut_ptr());
            }
            None
        });
    }
}

type QObjectWeakPtr = Weak<RefCell<QObjectPtr>>;

extern "C" {
    fn dos_qobject_signal_emit(vptr: *mut c_void, name: *const c_char,
                               argc: c_int, argv: *mut *mut c_void);
}