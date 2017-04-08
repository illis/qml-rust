use libc::{c_int, c_void};
use qurl::QUrl;

pub struct QQuickView {
    ptr: *mut c_void,
}

impl QQuickView {
    pub fn new() -> Self {
        unsafe {
            dos_qguiapplication_create();
            let view = dos_qquickview_create();
            dos_qquickview_set_resize_mode(view, 1);
            QQuickView {
                ptr: view,
            }
        }
    }

    pub fn load_url(&mut self, url: &str) {
        let qurl = QUrl::new(url);
        unsafe { dos_qquickview_set_source_url(self.ptr, qurl.as_ptr()) }
    }

    pub fn exec(&mut self) {
        unsafe {
            dos_qquickview_show(self.ptr);
            dos_qguiapplication_exec();
        }
    }
    pub fn quit(&mut self) {
        unsafe {
            dos_qguiapplication_quit();
        }
    }
}

impl Drop for QQuickView {
    fn drop(&mut self) {
        unsafe {
            dos_qguiapplication_quit();
            dos_qquickview_delete(self.ptr);
            dos_qguiapplication_delete();
        }
    }
}

extern "C" {
    fn dos_qguiapplication_create();
    fn dos_qguiapplication_exec();
    fn dos_qguiapplication_quit();
    fn dos_qguiapplication_delete();

    fn dos_qquickview_create() -> *mut c_void;
    fn dos_qquickview_set_source_url(vptr: *mut c_void, url: *mut c_void);
    fn dos_qquickview_show(vptr: *mut c_void);
    fn dos_qquickview_delete(vptr: *mut c_void);
    fn dos_qquickview_set_resize_mode(vptr: *mut c_void, resize_mode: c_int);
}

#[cfg(test)]
mod tests {
    use super::QQuickView;
    use std::ptr;

    // #[test]
    fn test_qurl_memory() {
        let view = QQuickView::new();
    }
}