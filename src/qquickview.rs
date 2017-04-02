
use qvariant::*;
use types::*;
use qurl::*;
use utils::*;
use libc;

extern "C" {
    fn dos_qguiapplication_create();
    fn dos_qguiapplication_exec();
    fn dos_qguiapplication_quit();
    fn dos_qguiapplication_delete();

    fn dos_qquickview_create() -> DosQQuickView;
    fn dos_qquickview_set_source_url(vptr: DosQQuickView, url: DosQUrl);
    fn dos_qquickview_show(vptr: DosQQuickView);
    fn dos_qquickview_rootContext(vptr: DosQQuickView) -> DosQQmlContext;
    fn dos_qquickview_delete(vptr: DosQQuickView);
    fn dos_qquickview_set_resize_mode(vptr: DosQQuickView, resizeMode: libc::c_int);

    fn dos_qqmlcontext_setcontextproperty(vptr: DosQQmlContext,
                                          name: DosCStr,
                                          value: DosQVariant);

}

/// Provides an entry point for building QML applications from Rust
pub struct QQuickView {
    ptr: DosQQuickView,
    stored: Vec<QVariant>,
}

impl QQuickView {
    /// Creates a QML context for QQuickView based application
    pub fn new() -> Self {
        unsafe {
            dos_qguiapplication_create();
            let view = dos_qquickview_create();
            dos_qquickview_set_resize_mode(view, 1);
            QQuickView {
                ptr: view,
                stored: Vec::new(),
            }
        }
    }

    /// Loads a file as a qml file
    pub fn load_file(&mut self, path: &str) {
        let path_raw = ::std::env::current_dir().unwrap().join(path);
        let path = if cfg!(windows) {
            format!("file:///{}", path_raw.display())
        } else {
            format!("file://{}", path_raw.display())
        };
        unsafe { dos_qquickview_set_source_url(self.ptr, construct_qurl(&path)) }
    }

    /// Loads qml from a specified url (`file://`, `qrc://`, `http://`)
    pub fn load_url(&mut self, uri: &str) {
        unsafe { dos_qquickview_set_source_url(self.ptr, construct_qurl(uri)) }
    }

    /// Launches the application
    pub fn exec(&mut self) {
        unsafe {
            dos_qquickview_show(self.ptr);
            dos_qguiapplication_exec();
        }
    }
    /// Closes the application
    pub fn quit(&mut self) {
        unsafe {
            dos_qguiapplication_quit();
        }
    }

    /// Sets a property for this QML context
    ///
    /// This variant stores qvariant, so it is removed, only when this QmlApplicationEngine is removed.
    pub fn set_and_store_property<T: Into<QVariant>>(&mut self, name: &str, value: T) {
        let val = value.into();
        unsafe {
            let context = dos_qquickview_rootContext(self.ptr);
            dos_qqmlcontext_setcontextproperty(context, stoptr(name), get_private_variant(&val));
        }
        self.stored.push(val);
    }

    /// Sets a property for this QML context
    pub fn set_property(&mut self, name: &str, value: &QVariant) {
        unsafe {
            let context = dos_qquickview_rootContext(self.ptr);
            dos_qqmlcontext_setcontextproperty(context, stoptr(name), get_private_variant(value));
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
