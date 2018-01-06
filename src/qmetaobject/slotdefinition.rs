use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use qmetaobject::ParameterDefinition;
use qmetaobject::conversionutils::{convert_as, convert_into};
use qmetaobject::parameterdefinition::{CParameterDefinition, ParameterDefinitionWrapper};
use qmetatype::QMetaType;

pub struct SlotDefinition {
    name: String,
    return_metatype: QMetaType,
    parameter_definitions: Vec<ParameterDefinition>,
}

impl SlotDefinition {
    pub fn new(
        name: &str,
        return_metatype: QMetaType,
        parameter_definitions: Vec<ParameterDefinition>,
    ) -> Self {
        SlotDefinition {
            name: name.to_string(),
            return_metatype,
            parameter_definitions,
        }
    }
}

pub(crate) struct SlotDefinitionWrapper {
    name: CString,
    return_metatype: c_int,
    parameter_definitions: Vec<ParameterDefinitionWrapper>,
}

pub(crate) struct IntermediateCSlotDefinition<'a> {
    name: &'a CString,
    return_metatype: c_int,
    parameter_definitions: Vec<CParameterDefinition>,
}

#[repr(C)]
pub(crate) struct CSlotDefinition {
    name: *const c_char,
    return_metatype: c_int,
    parameters_count: c_int,
    parameter_definitions: *const CParameterDefinition,
}

impl From<SlotDefinition> for SlotDefinitionWrapper {
    fn from(definition: SlotDefinition) -> Self {
        SlotDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            return_metatype: definition.return_metatype as c_int,
            parameter_definitions: convert_into(definition.parameter_definitions),
        }
    }
}

impl<'a> From<&'a SlotDefinitionWrapper> for IntermediateCSlotDefinition<'a> {
    fn from(definition: &'a SlotDefinitionWrapper) -> Self {
        IntermediateCSlotDefinition {
            name: &definition.name,
            return_metatype: definition.return_metatype,
            parameter_definitions: convert_as(&definition.parameter_definitions),
        }
    }
}

impl<'a> From<&'a IntermediateCSlotDefinition<'a>> for CSlotDefinition {
    fn from(definition: &IntermediateCSlotDefinition<'a>) -> Self {
        CSlotDefinition {
            name: definition.name.as_ptr(),
            return_metatype: definition.return_metatype as c_int,
            parameters_count: definition.parameter_definitions.len() as c_int,
            parameter_definitions: definition.parameter_definitions.as_ptr(),
        }
    }
}
