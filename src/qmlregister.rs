use libc::{c_char, c_int, c_void};

/*
extern "C" fn delete_dobject(id: i32, ptr: *const libc::c_void) {}

pub type RegisterQualifier = (i32, i32, &'static str, &'static str);
#[doc(hidden)]
pub trait QMLRegisterable: QObjectMacro {
    fn qualify_to_register(&self) -> RegisterQualifier;
    fn get_new(&self) -> *mut libc::c_void;
    fn get_qobj_from_ptr(&self, ptr: *mut libc::c_void) -> *mut QObject;
}

extern "C" fn create_dobject(id: i32,
                             wrapper: DosQObject,
                             binded_ptr: *mut *const libc::c_void,
                             dosQObject: *mut DosQObject) {
    let map = unsafe { &*(REGISTERED_TYPES.0.get()) };
    // Getting shallow object from the map
    let shallow = map.get(&id).unwrap();
    // Getting pointer to a created object
    let binded = shallow.get_new();

    // Returning pointers to a wrapper and to an DosQObject, then swapping DosQObject with a fresh one
    // Comments are copied 'as is' from the DOtherSide docs to ensure correctness
    unsafe {
        let mut qobj = &mut *shallow.get_qobj_from_ptr(binded);
        // # Retrieve the DosQObject created dos_qobject_create() inside the nimQObject
        *dosQObject = get_qobj_ptr(qobj);
        // # Store the pointer to the nimQObject
        *binded_ptr = get_binded_ptr(qobj);
        // # Swap the vptr inside the nimQObject with the wrapper
        set_qobj_ptr(qobj, wrapper);
    }
    forget(binded);
}

struct UnsafeWrapper(UnsafeCell<HashMap<i32, Box<QMLRegisterable>>>);
unsafe impl Sync for UnsafeWrapper {}
unsafe impl Send for UnsafeWrapper {}

lazy_static!{
    static ref REGISTERED_TYPES: UnsafeWrapper = UnsafeWrapper(UnsafeCell::new(HashMap::new()));
}

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
pub struct QmlRegisterType {
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

/*
lazy_static!{
    static ref REGISTERED_TYPES: UnsafeWrapper = UnsafeWrapper(UnsafeCell::new(HashMap::new()));
}
*/

extern "C" fn create_dobject(id: i32,
                             wrapper: *mut c_void,
                             binded_qobject: *mut *const c_void,
                             dos_qobject: *mut *const c_void) {
    /*let map = unsafe { &*(REGISTERED_TYPES.0.get()) };
    // Getting shallow object from the map
    let shallow = map.get(&id).unwrap();
    // Getting pointer to a created object
    let binded = shallow.get_new();

    // Returning pointers to a wrapper and to an DosQObject, then swapping DosQObject with a fresh one
    // Comments are copied 'as is' from the DOtherSide docs to ensure correctness
    unsafe {
        let mut qobj = &mut *shallow.get_qobj_from_ptr(binded);
        // # Retrieve the DosQObject created dos_qobject_create() inside the nimQObject
        *dosQObject = get_qobj_ptr(qobj);
        // # Store the pointer to the nimQObject
        *binded_ptr = get_binded_ptr(qobj);
        // # Swap the vptr inside the nimQObject with the wrapper
        set_qobj_ptr(qobj, wrapper);
    }
    forget(binded);*/

}

extern "C" fn delete_dobject(id: i32, ptr: *const c_void) {}

extern "C" {
    fn de_qqml_qmlregisterobject(qml_register_type: *const QmlRegisterType) -> c_int;
    // fn dos_qdeclarative_qmlregistersingletontype(qmlRegisterType: *const QmlRegisterType) -> c_int;
}