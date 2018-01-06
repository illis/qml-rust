use std::cell::RefCell;
use std::os::raw::c_void;
use qobject::{QObject, QObjectContent};

pub struct QObjectRefMut<'a> {
    ptr: &'a mut c_void,
}

impl<'a> QObjectRefMut<'a> {
    pub(crate) fn new(ptr: &'a mut c_void) -> Self {
        QObjectRefMut {
            ptr,
        }
    }

    pub fn as_cref_mut(&mut self) -> &mut c_void {
        self.ptr
    }

    pub fn as_content<T>(&mut self) -> Option<&'a RefCell<T>>
        where T: QObjectContent {
        let meta = T::metaobject();
        unsafe {
            let ptr = de_qobject_check_and_get_dobject(self.ptr, meta.as_ptr());
            (ptr as *const RefCell<T>).as_ref()
        }
    }
}

impl<'a, T> From<&'a mut QObject<T>> for QObjectRefMut<'a>
    where T: QObjectContent {
    fn from(value: &'a mut QObject<T>) -> Self {
        QObjectRefMut {
            ptr: value.as_cref_mut(),
        }
    }
}

extern "C" {
    fn de_qobject_check_and_get_dobject(vptr: *mut c_void, meta: *const c_void) -> *mut c_void;
}