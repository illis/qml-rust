use std::ffi::CString;
use std::os::raw::{c_double, c_float, c_int, c_void};

use conversions::TryFrom;
use errors::{Error, Result};
use internal::CStringWrapper;
use internal::ffi::{
    dos_qvariant_assign,
    dos_qvariant_create_bool,
    dos_qvariant_create_double,
    dos_qvariant_create_float,
    dos_qvariant_create_int,
    dos_qvariant_create_qvariant,
    dos_qvariant_create_string,
    dos_qvariant_delete,
    dos_qvariant_toBool,
    dos_qvariant_toDouble,
    dos_qvariant_toFloat,
    dos_qvariant_toInt,
    dos_qvariant_toString,
};
// use qobject::QObjectRefMut;

pub struct QVariant {
    ptr: *mut c_void,
}

impl QVariant {
    pub fn set(&mut self, value: &QVariant) {
        unsafe { dos_qvariant_assign(self.ptr, value.ptr) }
    }

    pub(crate) fn from_raw(ptr: *mut c_void) -> QVariant {
        QVariant { ptr }
    }

    pub(crate) fn as_raw(&self) -> *const c_void {
        self.ptr
    }

    /*
    pub(crate) fn as_raw_mut(&mut self) -> *mut c_void {
        self.ptr
    }
    */
}

impl Clone for QVariant {
    fn clone(&self) -> Self {
        unsafe {
            QVariant {
                ptr: dos_qvariant_create_qvariant(self.ptr),
            }
        }
    }
}

impl Drop for QVariant {
    fn drop(&mut self) {
        unsafe {
            dos_qvariant_delete(self.ptr);
        }
    }
}

// i32
impl From<QVariant> for i32 {
    fn from(value: QVariant) -> Self {
        unsafe { dos_qvariant_toInt(value.ptr) as i32 }
    }
}

impl<'a> From<&'a QVariant> for i32 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toInt(value.ptr) as i32 }
    }
}

impl From<i32> for QVariant {
    fn from(value: i32) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_int(value as c_int) },
        }
    }
}

// f32
impl From<QVariant> for f32 {
    fn from(value: QVariant) -> Self {
        unsafe { dos_qvariant_toFloat(value.ptr) as f32 }
    }
}

impl<'a> From<&'a QVariant> for f32 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toFloat(value.ptr) as f32 }
    }
}

impl From<f32> for QVariant {
    fn from(value: f32) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_float(value as c_float) },
        }
    }
}

// f64
impl From<QVariant> for f64 {
    fn from(value: QVariant) -> Self {
        unsafe { dos_qvariant_toDouble(value.ptr) as f64 }
    }
}

impl<'a> From<&'a QVariant> for f64 {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toDouble(value.ptr) as f64 }
    }
}

impl From<f64> for QVariant {
    fn from(value: f64) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_double(value as c_double) },
        }
    }
}

// bool
impl From<QVariant> for bool {
    fn from(value: QVariant) -> Self {
        unsafe { dos_qvariant_toBool(value.ptr) }
    }
}

impl<'a> From<&'a QVariant> for bool {
    fn from(value: &QVariant) -> Self {
        unsafe { dos_qvariant_toBool(value.ptr) }
    }
}

impl From<bool> for QVariant {
    fn from(value: bool) -> Self {
        QVariant {
            ptr: unsafe { dos_qvariant_create_bool(value) },
        }
    }
}

// str
impl TryFrom<QVariant> for String {
    type Error = Error;

    fn try_from(value: QVariant) -> Result<Self> {
        let string = CStringWrapper::from_raw(unsafe { dos_qvariant_toString(value.ptr) });
        let string = string.into_str()?;
        Ok(string)
    }
}

impl<'a> TryFrom<&'a QVariant> for String {
    type Error = Error;

    fn try_from(value: &QVariant) -> Result<Self> {
        let string = CStringWrapper::from_raw(unsafe { dos_qvariant_toString(value.ptr) });
        let string = string.into_str()?;
        Ok(string)
    }
}

impl<'a> TryFrom<&'a str> for QVariant {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let string = CString::new(value)?;
        Ok(QVariant {
            ptr: unsafe { dos_qvariant_create_string(string.as_ptr()) },
        })
    }
}

/*
// QObjectRefMut
impl<'a, 'b: 'a> From<QObjectRefMut<'a>> for QVariant<'b> {
    fn from(mut value: QObjectRefMut<'a>) -> Self {
        let ptr = value.as_cref_mut();
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
*/

#[cfg(test)]
mod tests {
    use super::QVariant;
    use conversions::TryFrom;

    #[test]
    fn test_qvariant_i32_memory() {
        let value = QVariant::from(123i32);
        QVariant::try_from(123i32).unwrap();
        i32::from(value);
    }

    #[test]
    fn test_qvariant_f32_memory() {
        let value = QVariant::from(123.456f32);
        QVariant::try_from(123.456f32).unwrap();
        f32::from(value);
    }

    #[test]
    fn test_qvariant_f64_memory() {
        let value = QVariant::from(123.456f64);
        QVariant::try_from(123.456f64).unwrap();
        f64::from(value);
    }

    #[test]
    fn test_qvariant_bool_memory() {
        let value = QVariant::from(true);
        QVariant::try_from(true).unwrap();
        bool::from(value);
    }

    #[test]
    fn test_qvariant_string_memory() {
        let value = QVariant::try_from("test").unwrap();
        String::try_from(value).unwrap();
    }
}
