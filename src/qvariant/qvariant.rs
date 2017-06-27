use std::ffi::CString;
use libc::{c_char, c_double, c_float, c_int, c_void};
use qobject::QObjectRefMut;
use internal::CStringWrapper;

pub struct QVariant<'a> {
    ptr: &'a mut c_void,
}

impl<'a> QVariant<'a> {
    pub fn set(&mut self, value: &QVariant) {
        unsafe { dos_qvariant_assign(self.ptr, value.ptr) }
    }

    pub(crate) fn new(ptr: &'a mut c_void) -> QVariant<'a> {
        QVariant {
            ptr: ptr,
        }
    }

    pub(crate) fn get_ptr(&self) -> &c_void {
        self.ptr
    }

    pub(crate) fn get_mut(&mut self) -> &mut c_void {
        self.ptr
    }
}

impl<'a> Clone for QVariant<'a> {
    fn clone(&self) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_qvariant(self.ptr).as_mut().unwrap(),
            }
        }
    }
}

impl<'a> Drop for QVariant<'a> {
    fn drop(&mut self) {
        unsafe { dos_qvariant_delete(self.ptr); }
    }
}

// i32
impl<'a, 'b: 'a> From<&'b QVariant<'a>> for i32 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toInt(value.ptr) as i32 }
    }
}

impl<'a> From<i32> for QVariant<'a> {
    fn from(value: i32) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_int(value as c_int).as_mut().unwrap() },
        }
    }
}

// f32
impl<'a, 'b: 'a> From<&'b QVariant<'a>> for f32 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toFloat(value.ptr) as f32 }
    }
}

impl<'a> From<f32> for QVariant<'a> {
    fn from(value: f32) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_float(value as c_float).as_mut().unwrap() },
        }
    }
}

// f64
impl<'a, 'b: 'a> From<&'b QVariant<'a>> for f64 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toDouble(value.ptr) as f64 }
    }
}

impl<'a> From<f64> for QVariant<'a> {
    fn from(value: f64) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_double(value as c_double).as_mut().unwrap() },
        }
    }
}

// bool
impl<'a, 'b: 'a> From<&'b QVariant<'a>> for bool {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toBool(value.ptr) }
    }
}

impl<'a> From<bool> for QVariant<'a> {
    fn from(value: bool) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_bool(value).as_mut().unwrap() },
        }
    }
}

// str
impl<'a, 'b: 'a> From<&'b QVariant<'a>> for String {
    fn from(value: &QVariant) -> Self {
        let string = CStringWrapper::new(unsafe { dos_qvariant_toString(value.ptr) });
        String::from(&string)
    }
}

impl<'a, 'b> From<&'a str> for QVariant<'b> {
    fn from(value: &'a str) -> Self {
        let string = CString::new(value).unwrap();
        QVariant {
            ptr: unsafe { dos_qvariant_create_string(string.as_ptr()).as_mut().unwrap() },
        }
    }
}

impl<'a> From<String> for QVariant<'a> {
    fn from(value: String) -> Self {
        let string = CString::new(value).unwrap();
        QVariant {
            ptr: unsafe { dos_qvariant_create_string(string.as_ptr()).as_mut().unwrap() },
        }
    }
}

impl<'a, 'b> From<&'a String> for QVariant<'b> {
    fn from(value: &String) -> Self {
        let string = CString::new(value.as_str()).unwrap();
        QVariant {
            ptr: unsafe { dos_qvariant_create_string(string.as_ptr()).as_mut().unwrap() },
        }
    }
}

// QObjectRefMut
impl<'a, 'b: 'a> From<QObjectRefMut<'a>> for QVariant<'b> {
    fn from(mut value: QObjectRefMut<'a>) -> Self {
        let ptr = value.as_mut();
        QVariant {
            ptr: unsafe { dos_qvariant_create_qobject(ptr).as_mut().unwrap() },
        }
    }
}

impl<'a, 'b: 'a> From<&'b QVariant<'a>> for QObjectRefMut<'a> {
    fn from(value: &QVariant) -> Self {
        let ptr = unsafe { dos_qvariant_toQObject(value.ptr) };
        unsafe { QObjectRefMut::new(ptr.as_mut().unwrap()) }
    }
}

extern "C" {
    fn dos_qvariant_assign(vptr: *mut c_void, other: *const c_void);
    fn dos_qvariant_delete(vptr: *mut c_void);
    fn dos_qvariant_create_qvariant(value: *const c_void) -> *mut c_void;

    fn dos_qvariant_create_int(value: c_int) -> *mut c_void;
    fn dos_qvariant_create_float(value: c_float) -> *mut c_void;
    fn dos_qvariant_create_double(value: c_double) -> *mut c_void;
    fn dos_qvariant_create_bool(value: bool) -> *mut c_void;
    fn dos_qvariant_create_string(value: *const c_char) -> *mut c_void;
    fn dos_qvariant_create_qobject(value: *mut c_void) -> *mut c_void;
    // fn dos_qvariant_create_array(size: c_int, array: *const c_void) -> *mut c_void;

    fn dos_qvariant_toInt(value: *const c_void) -> c_int;
    fn dos_qvariant_toFloat(value: *const c_void) -> c_float;
    fn dos_qvariant_toDouble(value: *const c_void) -> c_double;
    fn dos_qvariant_toBool(value: *const c_void) -> bool;
    fn dos_qvariant_toString(value: *const c_void) -> *mut c_char;
    fn dos_qvariant_toQObject(value: *const c_void) -> *mut c_void;
}

#[cfg(test)]
mod tests {
    use super::QVariant;

    #[test]
    fn test_qvariant_i32_memory() {
        QVariant::from(123 as i32);
    }

    #[test]
    fn test_qvariant_f32_memory() {
        QVariant::from(123.456 as f32);
    }

    #[test]
    fn test_qvariant_f64_memory() {
        QVariant::from(123.456 as f64);
    }

    #[test]
    fn test_qvariant_bool_memory() {
        QVariant::from(true);
    }

    #[test]
    fn test_qvariant_string_memory() {
        QVariant::from("test");
    }
}