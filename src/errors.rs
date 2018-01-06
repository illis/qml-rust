use std::ffi;

error_chain! {
    foreign_links {
        FfiString(ffi::NulError);
    }
}
