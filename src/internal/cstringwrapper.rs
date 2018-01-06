use std::ffi::CStr;
use std::os::raw::c_char;

pub(crate) struct CStringWrapper {
    ptr: *const c_char,
}

impl CStringWrapper {
    pub(crate) fn new(ptr: *const c_char) -> CStringWrapper {
        CStringWrapper {
            ptr,
        }
    }
}

impl Drop for CStringWrapper {
    fn drop(&mut self) {
        unsafe { dos_chararray_delete(self.ptr) }
    }
}

impl<'a> From<&'a CStringWrapper> for String {
    fn from(value: &'a CStringWrapper) -> Self {
        let string = unsafe { CStr::from_ptr(value.ptr) };
        string.to_string_lossy().into_owned()
    }
}

extern "C" {
    fn dos_chararray_delete(vptr: *const c_char);
}