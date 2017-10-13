use std::ffi::CString;
use libc::{c_char, c_int};
use qmetaobject::conversionutils::{convert_as, convert_into};
use qmetaobject::ParameterDefinition;
use qmetaobject::parameterdefinition::{CParameterDefinition, ParameterDefinitionWrapper};

pub struct SignalDefinition {
    name: String,
    parameter_definitions: Vec<ParameterDefinition>,
}

impl SignalDefinition {
    pub fn new(name: &str, parameter_definitions: Vec<ParameterDefinition>) -> Self {
        SignalDefinition {
            name: name.to_string(),
            parameter_definitions,
        }
    }
}

pub(crate) struct SignalDefinitionWrapper {
    name: CString,
    parameter_definitions: Vec<ParameterDefinitionWrapper>,
}

pub(crate) struct IntermediateCSignalDefinition<'a> {
    name: &'a CString,
    parameter_definitions: Vec<CParameterDefinition>,
}

#[repr(C)]
pub(crate) struct CSignalDefinition {
    name: *const c_char,
    parameters_count: c_int,
    parameter_definitions: *const CParameterDefinition,
}

impl From<SignalDefinition> for SignalDefinitionWrapper {
    fn from(definition: SignalDefinition) -> Self {
        SignalDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            parameter_definitions: convert_into(definition.parameter_definitions),
        }
    }
}

impl<'a> From<&'a SignalDefinitionWrapper> for IntermediateCSignalDefinition<'a> {
    fn from(definition: &'a SignalDefinitionWrapper) -> Self {
        IntermediateCSignalDefinition {
            name: &definition.name,
            parameter_definitions: convert_as(&definition.parameter_definitions),
        }
    }
}

impl<'a> From<&'a IntermediateCSignalDefinition<'a>> for CSignalDefinition {
    fn from(definition: &IntermediateCSignalDefinition) -> Self {
        CSignalDefinition {
            name: definition.name.as_ptr(),
            parameters_count: definition.parameter_definitions.len() as c_int,
            parameter_definitions: definition.parameter_definitions.as_ptr(),
        }
    }
}