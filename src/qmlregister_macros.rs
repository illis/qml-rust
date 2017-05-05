#[macro_export]
macro_rules! qml_register_qobject {
    ($name:ident as $qml:expr, $uri:expr, $major:expr, $minor:expr) => {
        impl QmlRegisterableObject for $name {
            extern "C" fn create_dobject(_: i32, wrapper: *mut c_void,
                                         dobject_ptr: *mut *mut c_void,
                                         qobject_ptr: *mut *mut c_void) {
                unsafe {
                    let mut dobject = QQmlObject::<$name>::new(wrapper);
                    *qobject_ptr = dobject.as_mut() as *mut c_void;
                    *dobject_ptr = Box::into_raw(Box::new(dobject)) as *mut c_void;
                }
            }

            extern "C" fn delete_dobject(_: i32, dobject_ptr: *mut c_void) {
                unsafe { Box::from_raw(dobject_ptr as *mut QObject<$name>); }
            }

            fn get_register_type() -> QmlRegisterType {
                QmlRegisterType::new($uri, $major, $minor, stringify!($qml))
            }
        }
    }
}
