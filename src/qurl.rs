use std::ffi::CString;
use libc::{c_char, c_void};

pub struct QUrl {
    ptr: *mut c_void,
}

impl QUrl {
    pub fn new(url: &str) -> Self {
        let url = CString::new(url).unwrap();

        QUrl {
            ptr: unsafe { dos_qurl_create(url.as_ptr(), 0) }
        }
    }
}

impl Drop for QUrl {
    fn drop(&mut self) {
        unsafe { dos_qurl_delete(self.ptr); }
    }
}

pub fn get_mut(instance: &mut QUrl) -> *mut c_void {
    instance.ptr
}

extern "C" {
    fn dos_qurl_create(url: *const c_char, parsing_mode: i32) -> *mut c_void;
    fn dos_qurl_delete(url: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::QUrl;

    #[test]
    fn test_qurl_memory() {
        QUrl::new("http://some/url");
    }
}