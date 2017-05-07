use libc::c_void;
use qobject::{QObject, QObjectContent, QObjectContentConstructor};
use qmetaobject;

pub struct QObjectRefMut<'a> {
    ptr: &'a mut c_void,
}

impl<'a> QObjectRefMut<'a> {
    pub fn as_mut(&mut self) -> &mut c_void {
        self.ptr
    }

    pub fn as_content<T: QObjectContent>(&mut self) -> Option<&'a mut T> {
        let mut meta = T::get_metaobject();
        unsafe {
            let ptr = de_qobject_check_and_get_dobject(self.ptr, qmetaobject::get_mut(&mut meta));
            (ptr as *mut T).as_mut()
        }
    }
}

impl<'a, T: QObjectContent + QObjectContentConstructor> From<&'a mut QObject<T>> for QObjectRefMut<'a> {
    fn from(value: &'a mut QObject<T>) -> Self {
        QObjectRefMut {
            ptr: super::qobject::get_mut(value),
        }
    }
}

pub fn get_mut<'a, 'b: 'a>(instance: &'b mut QObjectRefMut<'a>) -> &'a mut c_void {
    instance.ptr
}

extern "C" {
    fn de_qobject_check_and_get_dobject(vptr: *mut c_void, meta: *const c_void) -> *mut c_void;
}