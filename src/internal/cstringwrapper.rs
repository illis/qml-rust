use std::ffi::CStr;
use std::os::raw::c_char;

use errors::Result;
use internal::ffi::dos_chararray_delete;

pub(crate) struct CStringWrapper {
    ptr: *const c_char,
}

impl CStringWrapper {
    pub(crate) fn from_raw(ptr: *const c_char) -> CStringWrapper {
        CStringWrapper { ptr }
    }

    pub(crate) fn into_str(self) -> Result<String> {
        let string = unsafe { CStr::from_ptr(self.ptr) };
        let string = string.to_str()?;
        Ok(String::from(string))
    }
}

impl Drop for CStringWrapper {
    fn drop(&mut self) {
        unsafe { dos_chararray_delete(self.ptr) }
    }
}
