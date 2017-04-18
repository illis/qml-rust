use std::ffi::CString;
use libc::{c_char, c_void};

pub struct QUrl<'a> {
    ptr: &'a mut c_void,
}

impl<'a> QUrl<'a> {
    pub fn new(url: &str) -> Option<Self> {
        let url = CString::new(url).unwrap();
        let qurl = unsafe {dos_qurl_create(url.as_ptr(), 0).as_mut()};

        qurl.map(|ptr| {
            QUrl {
                ptr: ptr
            }
        })
    }

    pub fn as_ptr(&self) -> &c_void {
        self.ptr
    }
}

impl<'a> Drop for QUrl<'a> {
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

#[cfg(test)]
mod tests {
    use super::QUrl;

    #[test]
    fn test_qurl_memory() {
        QUrl::new("http://some/url");
    }
}