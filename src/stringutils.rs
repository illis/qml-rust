use std::ffi::CStr;
use libc::c_char;

pub struct CStringWrapper {
    ptr: *const c_char,
}

impl CStringWrapper {
    pub fn new(ptr: *const c_char) -> CStringWrapper {
        CStringWrapper {
            ptr: ptr,
        }
    }
}

impl Drop for CStringWrapper {
    fn drop(&mut self) {
        unsafe {
            de_delete_cstring(self.ptr)
        }
    }
}

impl<'a> From<&'a CStringWrapper> for String {
    fn from(value: &'a CStringWrapper) -> Self {
        let string = unsafe { CStr::from_ptr(value.ptr) };
        string.to_string_lossy().into_owned()
    }
}

extern "C" {
    fn de_delete_cstring(vptr: *const c_char);
}