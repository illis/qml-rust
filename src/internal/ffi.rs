use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    // QGuiApplication
    pub(crate) fn de_qguiapplication_create(argc: c_int, argv: *const *const c_char)
        -> *mut c_void;
    pub(crate) fn dos_qguiapplication_exec();
    pub(crate) fn dos_qguiapplication_quit();
    pub(crate) fn de_qguiapplication_delete(vptr: *mut c_void);

    // QQuickView
    pub(crate) fn de_qquickview_create() -> *mut c_void;
    pub(crate) fn de_qquickview_set_source_url(vptr: *mut c_void, url: *const c_void);
    pub(crate) fn dos_qquickview_show(vptr: *mut c_void);
    pub(crate) fn dos_qquickview_delete(vptr: *mut c_void);
    pub(crate) fn dos_qquickview_set_resize_mode(vptr: *mut c_void, resize_mode: c_int);

    // QUrl
    pub(crate) fn dos_qurl_create(url: *const c_char, parsing_mode: c_int) -> *mut c_void;
    pub(crate) fn dos_qurl_delete(url: *mut c_void);
}
