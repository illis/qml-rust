use std::env;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

use internal::QUrlInternal;
use internal::ffi::{
    de_qguiapplication_create,
    de_qguiapplication_delete,
    de_qquickview_create,
    de_qquickview_set_source_url,
    dos_qguiapplication_exec,
    dos_qguiapplication_quit,
    dos_qquickview_delete,
    dos_qquickview_set_resize_mode,
    dos_qquickview_show,
};
use qurl::QUrl;

pub struct QQuickView {
    app: *mut c_void,
    view: *mut c_void,
}

impl QQuickView {
    pub fn new() -> Self {
        let argv_strings = env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<_>>();
        let argv = argv_strings
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<_>>();

        unsafe {
            let app = de_qguiapplication_create(argv.len() as c_int, argv.as_ptr());
            let view = de_qquickview_create();

            dos_qquickview_set_resize_mode(view, 1);

            QQuickView { app, view }
        }
    }

    pub fn load_url(&mut self, url: QUrl) {
        let url = QUrlInternal::from(url);
        let ptr = url.as_ptr();
        unsafe { de_qquickview_set_source_url(self.view, ptr) }
    }

    pub fn exec(&mut self) {
        unsafe {
            dos_qquickview_show(self.view);
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
            dos_qquickview_delete(self.view);
            de_qguiapplication_delete(self.app);
        }
    }
}

impl Default for QQuickView {
    fn default() -> Self {
        QQuickView::new()
    }
}
