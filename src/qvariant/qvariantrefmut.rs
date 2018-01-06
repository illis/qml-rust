use std::os::raw::{c_char, c_double, c_float, c_int, c_void};

use internal::CStringWrapper;
use qvariant::QVariant;

pub struct QVariantRefMut<'a> {
    ptr: &'a mut c_void,
}

impl<'a> QVariantRefMut<'a> {
    pub fn set(&mut self, value: &'a QVariant) {
        unsafe { dos_qvariant_assign(self.ptr, value.as_cref()) }
    }

    pub(crate) fn from_ptr(ptr: *mut c_void) -> Self {
        let ptr = unsafe { ptr.as_mut().unwrap() };
        QVariantRefMut { ptr }
    }

    pub(crate) fn as_cref(&self) -> &c_void {
        self.ptr
    }
}

// i32
impl<'a> From<QVariantRefMut<'a>> for i32 {
    fn from(value: QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toInt(value.ptr) as i32 }
    }
}

impl<'a, 'b: 'a> From<&'b QVariantRefMut<'a>> for i32 {
    fn from(value: &QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toInt(value.ptr) as i32 }
    }
}

// f32
impl<'a> From<QVariantRefMut<'a>> for f32 {
    fn from(value: QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toFloat(value.ptr) as f32 }
    }
}

impl<'a, 'b: 'a> From<&'b QVariantRefMut<'a>> for f32 {
    fn from(value: &QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toFloat(value.ptr) as f32 }
    }
}

// f64
impl<'a> From<QVariantRefMut<'a>> for f64 {
    fn from(value: QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toDouble(value.ptr) as f64 }
    }
}

impl<'a, 'b: 'a> From<&'b QVariantRefMut<'a>> for f64 {
    fn from(value: &QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toDouble(value.ptr) as f64 }
    }
}

// bool
impl<'a> From<QVariantRefMut<'a>> for bool {
    fn from(value: QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toBool(value.ptr) }
    }
}

impl<'a, 'b: 'a> From<&'b QVariantRefMut<'a>> for bool {
    fn from(value: &QVariantRefMut) -> Self {
        unsafe { dos_qvariant_toBool(value.ptr) }
    }
}

// str
impl<'a> From<QVariantRefMut<'a>> for String {
    fn from(value: QVariantRefMut) -> Self {
        let string = CStringWrapper::new(unsafe { dos_qvariant_toString(value.ptr) });
        String::from(&string)
    }
}

impl<'a, 'b: 'a> From<&'b QVariantRefMut<'a>> for String {
    fn from(value: &QVariantRefMut) -> Self {
        let string = CStringWrapper::new(unsafe { dos_qvariant_toString(value.ptr) });
        String::from(&string)
    }
}

extern "C" {
    fn dos_qvariant_toInt(value: *const c_void) -> c_int;
    fn dos_qvariant_toFloat(value: *const c_void) -> c_float;
    fn dos_qvariant_toDouble(value: *const c_void) -> c_double;
    fn dos_qvariant_toBool(value: *const c_void) -> bool;
    fn dos_qvariant_toString(value: *const c_void) -> *mut c_char;

    fn dos_qvariant_assign(vptr: *mut c_void, other: *const c_void);
}
