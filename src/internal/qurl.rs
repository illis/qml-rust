use std::os::raw::c_void;

use internal::ffi::{dos_qurl_create, dos_qurl_delete};
use qurl::QUrl;

pub(crate) struct QUrlInternal {
    ptr: *mut c_void,
}

impl QUrlInternal {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.ptr
    }
}

impl Drop for QUrlInternal {
    fn drop(&mut self) {
        unsafe {
            dos_qurl_delete(self.ptr);
        }
    }
}

impl From<QUrl> for QUrlInternal {
    fn from(url: QUrl) -> Self {
        let str = url.into_str();
        QUrlInternal {
            ptr: unsafe { dos_qurl_create(str.as_ptr(), 0) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{QUrl, QUrlInternal};

    #[test]
    fn test_qurl_memory() {
        let url = QUrl::new("http://some/url").unwrap();
        QUrlInternal::from(url);
    }
}
