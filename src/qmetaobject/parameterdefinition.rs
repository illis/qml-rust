use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use qmetatype::QMetaType;

pub struct ParameterDefinition {
    name: String,
    metatype: QMetaType,
}

impl ParameterDefinition {
    pub fn new(name: &str, metatype: QMetaType) -> Self {
        ParameterDefinition {
            name: name.to_string(),
            metatype,
        }
    }
}

pub(crate) struct ParameterDefinitionWrapper {
    name: CString,
    metatype: c_int,
}

#[repr(C)]
pub(crate) struct CParameterDefinition {
    name: *const c_char,
    metatype: c_int,
}

impl From<ParameterDefinition> for ParameterDefinitionWrapper {
    fn from(definition: ParameterDefinition) -> Self {
        ParameterDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            metatype: definition.metatype as c_int,
        }
    }
}

impl<'a> From<&'a ParameterDefinitionWrapper> for CParameterDefinition {
    fn from(definition: &ParameterDefinitionWrapper) -> Self {
        CParameterDefinition {
            name: definition.name.as_ptr(),
            metatype: definition.metatype as c_int,
        }
    }
}
