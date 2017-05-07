use std::ffi::CString;
use libc::{c_char, c_int, c_float, c_double, c_void};
use stringutils::CStringWrapper;

pub struct QVariant {
    ptr: *mut c_void,
}

impl QVariant {
    pub fn set(&mut self, value: &QVariant) {
        unsafe { dos_qvariant_assign(self.ptr, value.ptr) }
    }
}

impl Clone for QVariant {
    fn clone(&self) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_qvariant(self.ptr).as_mut().unwrap(),
            }
        }
    }
}

impl Drop for QVariant {
    fn drop(&mut self) {
        unsafe { dos_qvariant_delete(self.ptr); }
    }
}

pub fn get_ptr(instance: &QVariant) -> *const c_void {
    instance.ptr
}

pub fn get_mut(instance: &mut QVariant) -> *mut c_void {
    instance.ptr
}

// i32
impl<'a> From<&'a QVariant> for i32 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toInt(value.ptr) as i32 }
    }
}

impl<'a> From<i32> for QVariant {
    fn from(value: i32) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_int(value as c_int).as_mut().unwrap() },
        }
    }
}

// f32
impl<'a> From<&'a QVariant> for f32 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toFloat(value.ptr) as f32 }
    }
}

impl<'a> From<f32> for QVariant {
    fn from(value: f32) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_float(value as c_float).as_mut().unwrap() },
        }
    }
}

// f64
impl<'a> From<&'a QVariant> for f64 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toDouble(value.ptr) as f64 }
    }
}

impl<'a> From<f64> for QVariant {
    fn from(value: f64) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_double(value as c_double).as_mut().unwrap() },
        }
    }
}

// bool
impl<'a> From<&'a QVariant> for bool {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toBool(value.ptr) }
    }
}

impl<'a> From<bool> for QVariant {
    fn from(value: bool) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_bool(value).as_mut().unwrap() },
        }
    }
}

// str
impl<'a> From<&'a QVariant> for String {
    fn from(value: &QVariant) -> Self {
        let string = CStringWrapper::new(unsafe { dos_qvariant_toString(value.ptr) });
        String::from(&string)
    }
}

impl<'a> From<&'a str> for QVariant {
    fn from(value: &'a str) -> Self {
        let string = CString::new(value).unwrap();
        QVariant {
            ptr: unsafe { dos_qvariant_create_string(string.as_ptr()).as_mut().unwrap() },
        }
    }
}

impl From<String> for QVariant {
    fn from(value: String) -> Self {
        let string = CString::new(value).unwrap();
        QVariant {
            ptr: unsafe { dos_qvariant_create_string(string.as_ptr()).as_mut().unwrap() },
        }
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
    // fn dos_qvariant_create_qobject(value: *mut c_void) -> *mut c_void;
    // fn dos_qvariant_create_array(size: c_int, array: *const c_void) -> *mut c_void;

    fn dos_qvariant_toInt(value: *const c_void) -> c_int;
    fn dos_qvariant_toFloat(value: *const c_void) -> c_float;
    fn dos_qvariant_toDouble(value: *const c_void) -> c_double;
    fn dos_qvariant_toBool(value: *const c_void) -> bool;
    fn dos_qvariant_toString(value: *const c_void) -> *mut c_char;
    // fn dos_qvariant_toQObject(value: *const c_void) -> *mut c_void;
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