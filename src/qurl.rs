use std::ffi::CString;
use libc::{c_char, c_void};

pub struct QUrl {
    ptr: *mut c_void,
}

impl QUrl {
    pub fn new(url: &str) -> QUrl {
        QUrl {
            ptr: construct_qurl(url)
        }
    }
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for QUrl {
    fn drop(&mut self) {
        unsafe {
            dos_qurl_delete(self.ptr);
        }
    }
}

extern "C" {
    fn dos_qurl_create(url: *const c_char, parsing_mode: i32) -> *mut c_void;
    fn dos_qurl_delete(url: *mut c_void);
}

fn construct_qurl(url: &str) -> *mut c_void {
    let url = CString::new(url).unwrap();
    unsafe { dos_qurl_create(url.as_ptr(), 0) }
}

#[cfg(test)]
mod tests {
    use super::QUrl;
    use std::ptr;

    #[test]
    fn test_qurl_memory() {
        let url = QUrl::new("http://some/url");
        assert_ne!(url.as_ptr(), ptr::null_mut());
    }
}