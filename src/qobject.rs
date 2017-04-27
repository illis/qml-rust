use std::cell::RefCell;
use std::ffi::CString;
use std::rc::{Rc, Weak};
use std::slice::from_raw_parts_mut;
use libc::{c_char, c_int, c_void};
use qmetaobject::QMetaObject;
use qobjectcontent::{QObjectContent, QObjectContentConstructor};
use qsignalemitter::QSignalEmitter;
use qvariant::QVariant;
use qvariantview::QVariantView;

pub struct QObject<T: QObjectContent> {
    _meta: QMetaObject,
    ptr: QObjectSharedPtr,
    content: Box<T>,
}

impl<T: QObjectContent + QObjectContentConstructor> QObject<T> {
    pub fn new() -> Self {
        let mut meta = T::get_metatype();
        let ptr = unsafe {
            de_qobject_create(::qmetaobject::get_mut(&mut meta), QObject::<T>::qslot_callback)
        };

        let ptr = Rc::new(RefCell::new(QObjectPtr::new(ptr)));
        let content = Box::new(T::new(Box::new(SignalEmitter::new(Rc::downgrade(&ptr)))));
        let content_ptr = Box::into_raw(content);
        let returned = QObject {
            _meta: meta,
            ptr: ptr.clone(),
            content: unsafe { Box::from_raw(content_ptr) },
        };
        unsafe {
            de_qobject_set_dobject(ptr.borrow_mut().as_mut(), content_ptr as *mut c_void);
        }
        returned
    }

    pub fn get_content(&self) -> &T {
        &*self.content
    }

    pub fn get_content_mut(&mut self) -> &mut T {
        &mut *self.content
    }

    pub fn as_mut<'a>(&'a mut self) -> &'a mut c_void {
        let ptr = self.ptr.borrow_mut().as_mut();
        unsafe {
            ptr.as_mut().unwrap()
        }
    }

    extern "C" fn qslot_callback(object: *mut c_void, slot_name: *mut c_void,
                                 argc: c_int, argv: *mut *mut c_void) {
        let object_ptr = object as *mut T;
        let object = unsafe { object_ptr.as_mut() }.unwrap();
        let slice = unsafe { from_raw_parts_mut(argv, argc as usize) };
        let vec: Vec<QVariantView> = slice.iter()
            .skip(1)
            .map(|&variant| QVariantView::from_ptr(variant))
            .collect();
        let slot_name: String = QVariantView::from_ptr(slot_name).into();

        if let Some(returned) = object.invoke_slot(&slot_name, vec) {
            let mut output = QVariantView::from_ptr(slice[0]);
            output.set(&returned);
        }
    }
}

pub struct SignalEmitter {
    ptr: QObjectWeakPtr,
}

impl SignalEmitter {
    fn new(ptr: QObjectWeakPtr) -> SignalEmitter {
        SignalEmitter {
            ptr: ptr,
        }
    }
}

impl QSignalEmitter for SignalEmitter {
    fn emit_signal(&mut self, name: &str, mut args: Vec<QVariant>) {
        let string = CString::new(name).unwrap();
        let mut args: Vec<*mut c_void> = args.iter_mut()
            .map(|item| ::qvariant::get_mut(item))
            .collect();

        self.ptr.upgrade().and_then::<(), _>(|ptr| {
            let ptr = ptr.borrow_mut().as_mut();

            unsafe {
                dos_qobject_signal_emit(ptr, string.as_ptr(), args.len() as c_int,
                                        args.as_mut_ptr());
            }
            None
        });
    }
}

type QObjectSharedPtr = Rc<RefCell<QObjectPtr>>;
type QObjectWeakPtr = Weak<RefCell<QObjectPtr>>;

struct QObjectPtr {
    ptr: *mut c_void,
}

impl QObjectPtr {
    fn new(ptr: *mut c_void) -> Self {
        QObjectPtr {
            ptr: ptr,
        }
    }

    fn as_mut(&mut self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for QObjectPtr {
    fn drop(&mut self) {
        unsafe {
            dos_qobject_delete(self.ptr)
        }
    }
}

type DObjectCallback = extern "C" fn(*mut c_void, *mut c_void, c_int, *mut *mut c_void);

extern "C" {
    fn de_qobject_create(meta_object: *const c_void, callback: DObjectCallback) -> *mut c_void;
    fn de_qobject_set_dobject(vptr: *mut c_void, content: *mut c_void);
    fn dos_qobject_delete(vptr: *mut c_void);

    fn dos_qobject_signal_emit(vptr: *mut c_void, name: *const c_char,
                               argc: c_int, argv: *mut *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::{QObject, QSignalEmitter};
    use qmetaobject::QMetaObject;
    use qobjectcontent::{QObjectContent, QObjectContentConstructor};
    use qvariant::QVariant;
    use qvariantview::QVariantView;

    struct Content {}

    impl QObjectContent for Content {
        fn get_metatype() -> QMetaObject {
            QMetaObject::new_qobject("Meta", Vec::new(), Vec::new(), Vec::new())
        }

        fn invoke_slot(&mut self, _: &str, _: Vec<QVariantView>) -> Option<QVariant> {
            None
        }
    }

    impl QObjectContentConstructor for Content {
        fn new(_: Box<QSignalEmitter>) -> Self {
            Content {}
        }
    }

    #[test]
    fn test_qobject_memory() {
        QObject::<Content>::new();
    }
}