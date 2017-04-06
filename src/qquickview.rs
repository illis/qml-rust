use libc;
use qurl::*;

enum WQQuickView {}

type DosQQuickView = *mut WQQuickView;

extern "C" {
    fn dos_qguiapplication_create();
    fn dos_qguiapplication_exec();
    fn dos_qguiapplication_quit();
    fn dos_qguiapplication_delete();

    fn dos_qquickview_create() -> DosQQuickView;
    fn dos_qquickview_set_source_url(vptr: DosQQuickView, url: DosQUrl);
    fn dos_qquickview_show(vptr: DosQQuickView);
    fn dos_qquickview_delete(vptr: DosQQuickView);
    fn dos_qquickview_set_resize_mode(vptr: DosQQuickView, resizeMode: libc::c_int);
}

pub struct QQuickView {
    ptr: DosQQuickView
}

impl QQuickView {
    pub fn new() -> Self {
        unsafe {
            dos_qguiapplication_create();
            let view = dos_qquickview_create();
            dos_qquickview_set_resize_mode(view, 1);
            QQuickView {
                ptr: view
            }
        }
    }

    pub fn load_url(&mut self, uri: &str) {
        unsafe { dos_qquickview_set_source_url(self.ptr, construct_qurl(uri)) }
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

impl Default for QQuickView {
    fn default() -> Self {
        Self::new()
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
