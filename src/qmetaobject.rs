use std::ffi::CString;
use libc::{c_char, c_int, c_void};
use qmetatype::QMetaType;

pub struct ParameterDefinition {
    name: String,
    metatype: QMetaType,
}

impl ParameterDefinition {
    pub fn new(name: &str, metatype: QMetaType) -> Self {
        ParameterDefinition {
            name: name.to_string(),
            metatype: metatype,
        }
    }
}

pub struct SignalDefinition {
    name: String,
    parameter_definitions: Vec<ParameterDefinition>,
}

impl SignalDefinition {
    pub fn new(name: &str, parameter_definitions: Vec<ParameterDefinition>) -> Self {
        SignalDefinition {
            name: name.to_string(),
            parameter_definitions: parameter_definitions,
        }
    }
}

pub struct SlotDefinition {
    name: String,
    return_metatype: QMetaType,
    parameter_definitions: Vec<ParameterDefinition>,
}

impl SlotDefinition {
    pub fn new(name: &str, return_metatype: QMetaType, parameter_definitions: Vec<ParameterDefinition>) -> Self {
        SlotDefinition {
            name: name.to_string(),
            return_metatype: return_metatype,
            parameter_definitions: parameter_definitions,
        }
    }
}

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
            property_metatype: property_metatype,
            read_slot: read_slot.to_string(),
            write_slot: "".to_string(),
            notify_signal: "".to_string(),
        }
    }

    pub fn new_read_only(name: &str, property_metatype: QMetaType, read_slot: &str, notify_signal: &str) -> Self {
        PropertyDefinition {
            name: name.to_string(),
            property_metatype: property_metatype,
            read_slot: read_slot.to_string(),
            write_slot: "".to_string(),
            notify_signal: notify_signal.to_string(),
        }
    }

    pub fn new_read_write(name: &str, property_metatype: QMetaType, read_slot: &str, write_slot: &str, notify_signal: &str) -> Self {
        PropertyDefinition {
            name: name.to_string(),
            property_metatype: property_metatype,
            read_slot: read_slot.to_string(),
            write_slot: write_slot.to_string(),
            notify_signal: notify_signal.to_string(),
        }
    }
}

pub struct QMetaObject {
    ptr: *mut c_void,
}

impl QMetaObject {
    pub fn new_qobject(name: &str,
                       signal_definitions: Vec<SignalDefinition>,
                       slot_definitions: Vec<SlotDefinition>,
                       property_definitions: Vec<PropertyDefinition>) -> Self {
        let signal_definition_wrappers = convert_into(signal_definitions);
        let signal_definition_intermediate = convert_as(&signal_definition_wrappers);
        let signal_definitions = convert_as(&signal_definition_intermediate);
        let c_signal_definitions = CSignalDefinitions {
            count: signal_definition_wrappers.len() as c_int,
            definitions: signal_definitions.as_ptr(),
        };

        let slot_definition_wrappers = convert_into(slot_definitions);
        let slot_definition_intermediate = convert_as(&slot_definition_wrappers);
        let slot_definition = convert_as(&slot_definition_intermediate);
        let c_slot_definitions = CSlotDefinitions {
            count: slot_definition_wrappers.len() as c_int,
            definitions: slot_definition.as_ptr(),
        };

        let property_definition_wrappers = convert_into(property_definitions);
        let property_definitions = convert_as(&property_definition_wrappers);
        let c_property_definitions = CPropertyDefinitions {
            count: property_definition_wrappers.len() as c_int,
            definitions: property_definitions.as_ptr(),
        };

        let qmeta = QMetaObject::qobject_metaobject();
        let name = CString::new(name).unwrap();
        let ptr = unsafe {
            dos_qmetaobject_create(qmeta.ptr, name.as_ptr(), &c_signal_definitions,
                                   &c_slot_definitions, &c_property_definitions)
        };
        QMetaObject {
            ptr: ptr
        }
    }

    pub fn qobject_metaobject() -> Self {
        QMetaObject {
            ptr: unsafe { dos_qobject_qmetaobject() },
        }
    }
}

impl Drop for QMetaObject {
    fn drop(&mut self) {
        unsafe {
            dos_qmetaobject_delete(self.ptr);
        }
    }
}

pub fn get_mut(instance: &mut QMetaObject) -> *mut c_void {
    instance.ptr
}

fn convert_into<T, U: From<T>>(input: Vec<T>) -> Vec<U> {
    input.into_iter()
        .map(|item| U::from(item))
        .collect()
}

fn convert_as<'a, T, U: From<&'a T>>(input: &'a Vec<T>) -> Vec<U> {
    input.iter()
        .map(|item| U::from(item))
        .collect()
}

struct ParameterDefinitionWrapper {
    name: CString,
    metatype: c_int,
}

#[derive(Debug)]
#[repr(C)]
struct CParameterDefinition {
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

struct SignalDefinitionWrapper {
    name: CString,
    parameter_definitions: Vec<ParameterDefinitionWrapper>,
}

struct IntermediateCSignalDefinition<'a> {
    name: &'a CString,
    parameter_definitions: Vec<CParameterDefinition>,
}

#[derive(Debug)]
#[repr(C)]
struct CSignalDefinition {
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

#[derive(Debug)]
#[repr(C)]
struct CSignalDefinitions {
    count: c_int,
    definitions: *const CSignalDefinition,
}

struct SlotDefinitionWrapper {
    name: CString,
    return_metatype: c_int,
    parameter_definitions: Vec<ParameterDefinitionWrapper>,
}

struct IntermediateCSlotDefinition<'a> {
    name: &'a CString,
    return_metatype: c_int,
    parameter_definitions: Vec<CParameterDefinition>,
}

#[derive(Debug)]
#[repr(C)]
struct CSlotDefinition {
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

#[derive(Debug)]
#[repr(C)]
struct CSlotDefinitions {
    count: c_int,
    definitions: *const CSlotDefinition,
}

struct PropertyDefinitionWrapper {
    name: CString,
    property_metatype: c_int,
    read_slot: CString,
    write_slot: CString,
    notify_slot: CString,
}

#[derive(Debug)]
#[repr(C)]
struct CPropertyDefinition {
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

#[derive(Debug)]
#[repr(C)]
struct CPropertyDefinitions {
    count: i32,
    definitions: *const CPropertyDefinition,
}

extern "C" {
    fn dos_qobject_qmetaobject() -> *mut c_void;
    fn dos_qmetaobject_create(super_class_meta_object: *mut c_void,
                              class_name: *const c_char,
                              signal_definitions: *const CSignalDefinitions,
                              slot_definitions: *const CSlotDefinitions,
                              property_definitions: *const CPropertyDefinitions)
                              -> *mut c_void;
    fn dos_qmetaobject_delete(vptr: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::{QMetaObject, ParameterDefinition, SignalDefinition, SlotDefinition, PropertyDefinition};
    use qmetatype::QMetaType;

    #[test]
    fn test_qmetaobject_memory() {
        QMetaObject::new_qobject("Meta", Vec::new(), Vec::new(), Vec::new());
    }

    #[test]
    fn test_qmetaobject_memory_with_data() {
        let signal_definitions = vec![
            SignalDefinition {
                name: "testSignal1".to_string(),
                parameter_definitions: vec![
                    ParameterDefinition::new("first", QMetaType::Bool),
                    ParameterDefinition::new("second", QMetaType::Int),
                    ParameterDefinition::new("third", QMetaType::QString),
                ],
            },
            SignalDefinition {
                name: "testSignal2".to_string(),
                parameter_definitions: vec![],
            }
        ];
        let slot_definitions = vec![
            SlotDefinition {
                name: "testSlot1".to_string(),
                return_metatype: QMetaType::Void,
                parameter_definitions: vec![
                    ParameterDefinition::new("first", QMetaType::Bool),
                    ParameterDefinition::new("second", QMetaType::Int),
                    ParameterDefinition::new("third", QMetaType::QString),
                ],
            },
            SlotDefinition {
                name: "testSlot2".to_string(),
                return_metatype: QMetaType::Int,
                parameter_definitions: vec![],
            }
        ];
        let property_definitions = vec![
            PropertyDefinition {
                name: "testProperty1".to_string(),
                property_metatype: QMetaType::QString,
                read_slot: "readTestProperty1".to_string(),
                write_slot: "writeTestProperty1".to_string(),
                notify_signal: "testProperty1Changed".to_string()
            },
            PropertyDefinition {
                name: "testProperty2".to_string(),
                property_metatype: QMetaType::Int,
                read_slot: "readTestProperty2".to_string(),
                write_slot: "writeTestProperty2".to_string(),
                notify_signal: "testProperty2Changed".to_string()
            }
        ];
        QMetaObject::new_qobject("Meta", signal_definitions, slot_definitions, property_definitions);
    }
}