use std::ffi::CString;

use errors::Result;

pub struct QUrl {
    url: CString,
}

impl QUrl {
    pub fn new(url: &str) -> Result<Self> {
        let url = CString::new(url)?;

        Ok(QUrl { url })
    }

    pub(crate) fn into_str(self) -> CString {
        self.url
    }
}
