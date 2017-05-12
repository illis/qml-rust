use std::ffi::CString;
use libc::{c_char, c_int, c_void};

pub struct QHashIntByteArray {
    ptr: *mut c_void,
}

impl QHashIntByteArray {
    pub fn new() -> Self {
        QHashIntByteArray {
            ptr: unsafe { dos_qhash_int_qbytearray_create() },
        }
    }

    pub fn add(&self, key: i32, value: &str) {
        let string = CString::new(value).unwrap();
        unsafe { dos_qhash_int_qbytearray_insert(self.ptr, key, string.as_ptr()) }
    }
}

impl Drop for QHashIntByteArray {
    fn drop(&mut self) {
        unsafe { dos_qhash_int_qbytearray_delete(self.ptr) }
    }
}

pub fn get_mut(instance: &mut QHashIntByteArray) -> *mut c_void {
    instance.ptr
}

extern "C" {
    fn dos_qhash_int_qbytearray_create() -> *mut c_void;
    fn dos_qhash_int_qbytearray_delete(vptr: *mut c_void);
    fn dos_qhash_int_qbytearray_insert(vptr: *mut c_void, key: c_int, value: *const c_char);
}