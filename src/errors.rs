use std::ffi;
use std::str;

error_chain! {
    foreign_links {
        StringUtf8Error(str::Utf8Error);
        FfiNulError(ffi::NulError);
    }
    errors {
        NullPointerError {
            description("Null pointer")
            display("Null pointer")
        }
    }
}
