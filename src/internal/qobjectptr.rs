use std::cell::RefCell;
use std::rc::Rc;
use libc::c_void;

pub(crate) struct QObjectPtr {
    ptr: *mut c_void,
}

pub(crate) type QObjectSharedPtr = Rc<RefCell<QObjectPtr>>;

impl QObjectPtr {
    pub fn new(ptr: *mut c_void) -> Self {
        QObjectPtr {
            ptr: ptr,
        }
    }

    pub fn as_mut(&mut self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for QObjectPtr {
    fn drop(&mut self) {
        unsafe { dos_qobject_delete(self.ptr) }
    }
}

extern "C" {
    fn dos_qobject_delete(vptr: *mut c_void);
}