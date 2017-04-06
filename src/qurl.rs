use libc;
use std::ffi::CString;

pub enum WQUrl {}

pub type DosQUrl = *mut WQUrl;

extern "C" {
    fn dos_qurl_create(url: *const libc::c_char, parsingMode: i32) -> DosQUrl;
}


pub fn construct_qurl(url: &str) -> DosQUrl {
    let url = CString::new(url).unwrap();
    unsafe { dos_qurl_create(url.as_ptr(), 0) }
}
