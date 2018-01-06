use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::os::raw::c_void;

pub(crate) type QObjectWeakPtr = Weak<RefCell<QObjectPtr>>;

pub(crate) struct QObjectPtr {
    ptr: *mut c_void,
}

pub(crate) type QObjectSharedPtr = Rc<RefCell<QObjectPtr>>;

impl QObjectPtr {
    pub(crate) fn new(ptr: *mut c_void) -> Self {
        QObjectPtr {
            ptr,
        }
    }

    pub(crate) fn as_cref<'a>(&self) -> &'a c_void {
        unsafe {self.ptr.as_ref() }.unwrap()
    }

    pub(crate) fn as_cref_mut<'a>(&mut self) -> &'a mut c_void {
        unsafe {self.ptr.as_mut() }.unwrap()
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