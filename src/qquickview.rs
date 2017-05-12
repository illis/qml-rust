use std::env;
use std::ffi::CString;
use libc::{c_char, c_int, c_void};
use qurl;
use qurl::QUrl;

pub struct QQuickView {
    app: *mut c_void,
    view: *mut c_void,
}

impl<'a> QQuickView {
    pub fn new() -> Self {
        unsafe {
            let argv_strings = env::args()
                .map(|arg| CString::new(arg).unwrap())
                .collect::<Vec<_>>();
            let argv = argv_strings.iter()
                .map(|arg| arg.as_ptr())
                .collect::<Vec<_>>();


            let app = de_qguiapplication_create(argv.len() as c_int, argv.as_ptr());
            let view = de_qquickview_create();

            dos_qquickview_set_resize_mode(view, 1);

            QQuickView {
                app: app,
                view: view,
            }
        }
    }

    pub fn load_url(&mut self, mut url: QUrl) {
        let ptr = qurl::get_mut(&mut url);
        unsafe { de_qquickview_set_source_url(self.view, ptr) }
    }

    pub fn exec(&mut self) {
        unsafe {
            dos_qquickview_show(self.view);
            dos_qguiapplication_exec();
        }
    }
    pub fn quit(&mut self) {
        unsafe { dos_qguiapplication_quit(); }
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

extern "C" {
    fn de_qguiapplication_create(argc: c_int, argv: *const *const c_char) -> *mut c_void;
    fn dos_qguiapplication_exec();
    fn dos_qguiapplication_quit();
    fn de_qguiapplication_delete(vptr: *mut c_void);

    fn de_qquickview_create() -> *mut c_void;
    fn de_qquickview_set_source_url(vptr: *mut c_void, url: *const c_void);
    fn dos_qquickview_show(vptr: *mut c_void);
    fn dos_qquickview_delete(vptr: *mut c_void);
    fn dos_qquickview_set_resize_mode(vptr: *mut c_void, resize_mode: c_int);
}
