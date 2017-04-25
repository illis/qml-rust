use std::ffi::CString;
use libc::{c_char, c_int, c_void};
use qobject::QObject;

pub struct QmlRegisterType {
    uri: &'static str,
    major: i32,
    minor: i32,
    name: &'static str,
}

pub trait QmlRegisterableObject {
    extern "C" fn create_qobject(id: i32, wrapper: *mut c_void,
                                 dobject_ptr: *mut *const c_void,
                                 qobject_ptr: *mut *const c_void);
    extern "C" fn delete_dobject(id: i32, dobject_ptr: *mut c_void);
    fn get_register_type() -> QmlRegisterType;
}

#[macro_export]
macro_rules! qml_register_qobject {
    ($rust_type:ident as $qml:expr, $uri:expr => $major:expr => $minor:expr) => {
        impl QmlRegisterableObject for $rust_type {
            extern "C" fn create_qobject(_: i32, _: *mut c_void,
                                         dobject_ptr: *mut *const c_void,
                                         qobject_ptr: *mut *const c_void) {
                unsafe {
                    let dobject = QObject::new(stringify!($qml), Vec::new(), Vec::new(), Vec::new(), $rust_type::new());
                    dobject.map(|mut dobject| {
                        *qobject_ptr = dobject.as_mut() as *mut c_void;
                        *dobject_ptr = Box::into_raw(Box::new(dobject)) as *mut c_void;
                    });
                }
            }

            extern "C" fn delete_dobject(_: i32, dobject_ptr: *mut c_void) {
                unsafe { Box::from_raw(dobject_ptr as *mut QObject<$rust_type>); }
            }

            fn get_register_type() -> QmlRegisterType {
                QmlRegisterType {
                    uri: $uri,
                    major: $major,
                    minor: $minor,
                    name: stringify!($qml),
                }
            }
        }
    }
}

/*
pub fn qml_register_qobject<T: QObjectContent>() {
    let register_type = T::get_register_type();
    let uri = CString::new(register_type.uri).unwrap();
    let qml = CString::new(register_type.qml).unwrap();
    let c_register_type = CQmlRegisterType {
        major: register_type.major,
        minor: register_type.minor,
        uri: uri.as_ptr(),
        qml: qml.as_ptr(),
        //static_meta_object:
    };
    unsafe {
        de_qqml_qmlregisterobject(T::get_register_type())
    }
}
*/


/*


type Registerer = unsafe extern "C" fn(*const QmlRegisterType) -> i32;
fn register_with<T: QMLRegisterable + 'static>(t: T, r: Registerer) {
    let (major, minor, uri, qml) = t.qualify_to_register();
    let qmeta = QMetaDefinition::new(t.qmeta());
    let meta = QMeta::new_for_qobject(qmeta);
    let mut map = unsafe { &mut *(REGISTERED_TYPES.0.get()) };

    let qrt = QmlRegisterType {
        major: major,
        minor: minor,
        uri: stoptr(uri),
        qml: stoptr(qml),
        static_meta_object: get_dos_qmeta(&meta),
        create_dobject: create_dobject,
        delete_dobject: delete_dobject,
    };
    forget(meta);
    let id = unsafe { r(&qrt as *const QmlRegisterType) };
    map.insert(id, Box::new(t));
    forget(qrt);
}

pub fn register_qml_type<T: QMLRegisterable + 'static>(t: T) {
    register_with(t, dos_qdeclarative_qmlregistertype)
}

pub fn register_qml_singleton_type<T: QMLRegisterable + 'static>(t: T) {
    register_with(t, dos_qdeclarative_qmlregistersingletontype)
}
*/

#[repr(C)]
pub struct CQmlRegisterType {
    major: c_int,
    minor: c_int,
    uri: *const c_char,
    qml: *const c_char,
    static_meta_object: *mut c_void,
    create_dobject: CreateDObject,
    delete_dobject: DeleteDObject,
}

pub type CreateDObject = extern "C" fn(c_int, *mut c_void, *mut *mut c_void, *mut *mut c_void);
pub type DeleteDObject = extern "C" fn(c_int, *mut c_void);

extern "C" {
    fn de_qqml_qmlregisterobject(qml_register_type: *const CQmlRegisterType) -> c_int;
    // fn dos_qdeclarative_qmlregistersingletontype(qmlRegisterType: *const QmlRegisterType) -> c_int;
}

/*
#[cfg(test)]
mod tests {
    use qvariant::QVariant;
    use qvariantview::QVariantView;
    use qobject::{QObject, QObjectContent, QObjectMeta};
    use super::{QmlRegisterType, QmlRegisterableObject};
    use libc::c_void;

    struct Test {
        value: i32
    }

    impl<'a> QObjectMeta for QObjectContent<'a, Test> {
        fn new_meta() -> Option<QMetaObject<'a>> {
            QMetaObject::new("QTest", Vec::new(), Vec::new(), Vec::new())
        }
        fn qslot_call(&mut self, _: &str, _: Vec<QVariantView>) -> Option<QVariant> {
            None
        }
    }

    impl Test {
        fn new() -> Self {
            Test {
                value: 0
            }
        }
    }

    qml_register_qobject!(Test as QTest, "test.submodule" => 1 => 0);

    #[test]
    fn test_qml_register_qobject() {
        assert_eq!(Test::get_register_type().uri, "test.submodule");
        assert_eq!(Test::get_register_type().major, 1);
        assert_eq!(Test::get_register_type().minor, 0);
        assert_eq!(Test::get_register_type().name, "QTest");
    }
}
*/