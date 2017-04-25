use std::ffi::CString;
use libc::{c_char, c_int, c_float, c_double, c_void};
use qvariant::QVariant;
use stringutils::CStringWrapper;

pub struct QVariantView {
    ptr: *mut c_void,
}

impl<'a> QVariantView {
    pub fn from_ptr(ptr: *mut c_void) -> Self {
        QVariantView {
            ptr: unsafe {ptr.as_mut().unwrap()}
        }
    }
    pub fn set(&mut self, value: &'a QVariant) {
        unsafe {
            dos_qvariant_assign(self.ptr, ::qvariant::get_ptr(value))
        }
    }
}

// i32
impl<'a> From<QVariantView> for i32 {
    fn from(value: QVariantView) -> Self {
        unsafe {dos_qvariant_toInt(value.ptr) as i32}
    }
}

impl<'a> From<&'a QVariantView> for i32 {
    fn from(value: &QVariantView) -> Self {
        unsafe {dos_qvariant_toInt(value.ptr) as i32}
    }
}

// f32
impl<'a> From<QVariantView> for f32 {
    fn from(value: QVariantView) -> Self {
        unsafe {dos_qvariant_toFloat(value.ptr) as f32}
    }
}

impl<'a> From<&'a QVariantView> for f32 {
    fn from(value: &QVariantView) -> Self {
        unsafe {dos_qvariant_toFloat(value.ptr) as f32}
    }
}

// f64
impl<'a> From<QVariantView> for f64 {
    fn from(value: QVariantView) -> Self {
        unsafe {dos_qvariant_toDouble(value.ptr) as f64}
    }
}

impl<'a> From<&'a QVariantView> for f64 {
    fn from(value: &QVariantView) -> Self {
        unsafe {dos_qvariant_toDouble(value.ptr) as f64}
    }
}

// bool
impl<'a> From<QVariantView> for bool {
    fn from(value: QVariantView) -> Self {
        unsafe {dos_qvariant_toBool(value.ptr)}
    }
}

impl<'a> From<&'a QVariantView> for bool {
    fn from(value: &QVariantView) -> Self {
        unsafe {dos_qvariant_toBool(value.ptr)}
    }
}

// str
impl<'a> From<QVariantView> for String {
    fn from(value: QVariantView) -> Self {
        unsafe {
            let string = CStringWrapper::new(unsafe {dos_qvariant_toString(value.ptr)});
            String::from(&string)
        }
    }
}

impl<'a> From<&'a QVariantView> for String {
    fn from(value: &QVariantView) -> Self {
        unsafe {
            let string = CStringWrapper::new(unsafe {dos_qvariant_toString(value.ptr)});
            String::from(&string)
        }
    }
}

extern "C" {
    fn dos_qvariant_toInt(value: *const c_void) -> c_int;
    fn dos_qvariant_toBool(value: *const c_void) -> bool;
    fn dos_qvariant_toString(value: *const c_void) -> *mut c_char;
    fn dos_qvariant_toFloat(value: *const c_void) -> c_float;
    fn dos_qvariant_toDouble(value: *const c_void) -> c_double;

    fn dos_qvariant_assign(vptr: *mut c_void, other: *const c_void);
}
