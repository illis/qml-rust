use std::ffi::CString;
use libc::{c_char, c_int, c_void};
use qobject::QObjectContent;

pub struct QmlRegisterType {
    uri: &'static str,
    major: i32,
    minor: i32,
    name: &'static str,
}

impl QmlRegisterType {
    pub fn new(uri: &'static str, major: i32, minor: i32, name: &'static str) -> Self {
        QmlRegisterType {
            uri,
            major,
            minor,
            name,
        }
    }
}

pub trait QmlRegisterableObject {
    extern "C" fn create_dobject(id: i32, wrapper: *mut c_void,
                                 dobject_ptr: *mut *mut c_void,
                                 qobject_ptr: *mut *mut c_void);
    extern "C" fn delete_dobject(id: i32, dobject_ptr: *mut c_void);
    fn register_type() -> QmlRegisterType;
}

pub fn qml_register_type<T>()
    where T: QObjectContent + QmlRegisterableObject {
    let register_type = T::register_type();
    let uri = CString::new(register_type.uri).unwrap();
    let qml = CString::new(register_type.name).unwrap();
    let mut qmeta = T::metaobject();

    let c_register_type = CQmlRegisterType {
        major: register_type.major,
        minor: register_type.minor,
        uri: uri.as_ptr(),
        qml: qml.as_ptr(),
        static_meta_object: qmeta.as_ptr_mut(),
        create_dobject: T::create_dobject,
        delete_dobject: T::delete_dobject,
    };
    unsafe { de_qqml_qmlregisterobject(&c_register_type); }
}

#[repr(C)]
struct CQmlRegisterType {
    major: c_int,
    minor: c_int,
    uri: *const c_char,
    qml: *const c_char,
    static_meta_object: *mut c_void,
    create_dobject: CreateDObject,
    delete_dobject: DeleteDObject,
}

type CreateDObject = extern "C" fn(c_int, *mut c_void, *mut *mut c_void, *mut *mut c_void);
type DeleteDObject = extern "C" fn(c_int, *mut c_void);

extern "C" {
    fn de_qqml_qmlregisterobject(qml_register_type: *const CQmlRegisterType) -> c_int;
    // fn dos_qdeclarative_qmlregistersingletontype(qmlRegisterType: *const QmlRegisterType) -> c_int;
}
