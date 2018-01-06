use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use qmetatype::QMetaType;

pub struct PropertyDefinition {
    name: String,
    property_metatype: QMetaType,
    read_slot: String,
    write_slot: String,
    notify_signal: String,
}

impl PropertyDefinition {
    pub fn new_const(name: &str, property_metatype: QMetaType, read_slot: &str) -> Self {
        PropertyDefinition {
            name: name.to_string(),
            property_metatype,
            read_slot: read_slot.to_string(),
            write_slot: "".to_string(),
            notify_signal: "".to_string(),
        }
    }

    pub fn new_read_only(name: &str, property_metatype: QMetaType, read_slot: &str, notify_signal: &str) -> Self {
        PropertyDefinition {
            name: name.to_string(),
            property_metatype,
            read_slot: read_slot.to_string(),
            write_slot: "".to_string(),
            notify_signal: notify_signal.to_string(),
        }
    }

    pub fn new_read_write(name: &str, property_metatype: QMetaType, read_slot: &str, write_slot: &str, notify_signal: &str) -> Self {
        PropertyDefinition {
            name: name.to_string(),
            property_metatype,
            read_slot: read_slot.to_string(),
            write_slot: write_slot.to_string(),
            notify_signal: notify_signal.to_string(),
        }
    }
}

pub(crate) struct PropertyDefinitionWrapper {
    name: CString,
    property_metatype: c_int,
    read_slot: CString,
    write_slot: CString,
    notify_slot: CString,
}

#[repr(C)]
pub(crate) struct CPropertyDefinition {
    name: *const c_char,
    property_metatype: c_int,
    read_slot: *const c_char,
    write_slot: *const c_char,
    notify_signal: *const c_char,
}

impl From<PropertyDefinition> for PropertyDefinitionWrapper {
    fn from(definition: PropertyDefinition) -> Self {
        PropertyDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            property_metatype: definition.property_metatype as c_int,
            read_slot: CString::new(definition.read_slot).unwrap(),
            write_slot: CString::new(definition.write_slot).unwrap(),
            notify_slot: CString::new(definition.notify_signal).unwrap(),
        }
    }
}

impl<'a> From<&'a PropertyDefinitionWrapper> for CPropertyDefinition {
    fn from(definition: &PropertyDefinitionWrapper) -> Self {
        CPropertyDefinition {
            name: definition.name.as_ptr(),
            property_metatype: definition.property_metatype as c_int,
            read_slot: definition.read_slot.as_ptr(),
            write_slot: definition.write_slot.as_ptr(),
            notify_signal: definition.notify_slot.as_ptr(),
        }
    }
}