use std::os::raw::{c_char, c_double, c_float, c_int, c_void};

#[repr(C)]
pub(crate) struct QVariantMapEntry {
    pub(crate) key: *const c_char,
    pub(crate) value: *const c_void,
}

#[repr(C)]
pub(crate) struct QVariantMap {
    pub(crate) count: c_int,
    pub(crate) values: *mut QVariantMapEntry,
}

extern "C" {
    // const char *
    pub(crate) fn dos_chararray_delete(vptr: *const c_char);

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

    // QVariant
    pub(crate) fn dos_qvariant_assign(vptr: *mut c_void, other: *const c_void);
    pub(crate) fn dos_qvariant_delete(vptr: *mut c_void);
    pub(crate) fn dos_qvariant_create_qvariant(value: *const c_void) -> *mut c_void;

    pub(crate) fn dos_qvariant_create_int(value: c_int) -> *mut c_void;
    pub(crate) fn dos_qvariant_create_float(value: c_float) -> *mut c_void;
    pub(crate) fn dos_qvariant_create_double(value: c_double) -> *mut c_void;
    pub(crate) fn dos_qvariant_create_bool(value: bool) -> *mut c_void;
    pub(crate) fn dos_qvariant_create_string(value: *const c_char) -> *mut c_void;
    // pub(crate) fn dos_qvariant_create_qobject(value: *mut c_void) -> *mut c_void;
    // pub(crate) fn dos_qvariant_create_array(size: c_int, array: *const c_void)
    // -> *mut c_void;

    pub(crate) fn dos_qvariant_toInt(value: *const c_void) -> c_int;
    pub(crate) fn dos_qvariant_toFloat(value: *const c_void) -> c_float;
    pub(crate) fn dos_qvariant_toDouble(value: *const c_void) -> c_double;
    pub(crate) fn dos_qvariant_toBool(value: *const c_void) -> bool;
    pub(crate) fn dos_qvariant_toString(value: *const c_void) -> *mut c_char;
    // pub(crate) fn dos_qvariant_toQObject(value: *const c_void) -> *mut c_void;

    // QVariantMap
    pub(crate) fn de_qvariantmap_delete(vptr: *const QVariantMap);
    pub(crate) fn de_qvariant_create_qvariantmap(value: *const QVariantMap) -> *mut c_void;
    pub(crate) fn de_qvariant_to_qvariantmap(value: *const c_void) -> *mut QVariantMap;
}
