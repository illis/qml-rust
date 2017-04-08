use std::ffi::CString;
use libc::{c_char, c_int, c_void};
use qmetatype::QMetaType;

pub struct SignalDefinition {
    name: String,
    parameters_meta_types: Vec<QMetaType>,
}

pub struct SlotDefinition {
    name: String,
    return_meta_type: QMetaType,
    parameters_meta_types: Vec<QMetaType>,
}

pub struct PropertyDefinition {
    name: String,
    property_meta_type: QMetaType,
    read_slot: String,
    write_slot: String,
    notify_signal: String,
}

pub struct QMetaObject {
    ptr: *mut c_void,
}

impl QMetaObject {
    pub fn new_qobject(name: &str,
                       signal_definitions: Vec<SignalDefinition>,
                       slot_definitions: Vec<SlotDefinition>,
                       property_definitions: Vec<PropertyDefinition>) -> QMetaObject {
        unsafe {
            let meta_obj = QMetaObject {
                ptr: dos_qobject_qmetaobject(),
            };

            let signal_definition_wrappers = convert_into(signal_definitions);
            let signal_definitions = convert_as(&signal_definition_wrappers);
            let c_signal_definitions = CSignalDefinitions {
                count: signal_definition_wrappers.len() as c_int,
                definitions: signal_definitions.as_ptr(),
            };

            let slot_definition_wrappers = convert_into(slot_definitions);
            let slot_definition = convert_as(&slot_definition_wrappers);
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

            let name = CString::new(name).unwrap();
            let dos_meta = dos_qmetaobject_create(meta_obj.ptr, name.as_ptr(), &c_signal_definitions,
                                                  &c_slot_definitions, &c_property_definitions);

            QMetaObject {
                ptr: dos_meta
            }
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for QMetaObject {
    fn drop(&mut self) {
        unsafe {
            dos_qmetaobject_delete(self.ptr);
        }
    }
}

fn convert_into<T, U: From<T>>(input: Vec<T>) -> Vec<U> {
    input.into_iter()
        .map(|item| U::from(item))
        .collect()
}

fn convert_into_metatype(input: Vec<QMetaType>) -> Vec<c_int> {
    input.into_iter()
        .map(|item| item as c_int)
        .collect()
}

fn convert_as<'a, T, U: From<&'a T>>(input: &'a Vec<T>) -> Vec<U> {
    input.iter()
        .map(|item| U::from(item))
        .collect()
}

struct SignalDefinitionWrapper {
    name: CString,
    parameters_meta_types: Vec<c_int>,
}

impl From<SignalDefinition> for SignalDefinitionWrapper {
    fn from(definition: SignalDefinition) -> Self {
        SignalDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            parameters_meta_types: convert_into_metatype(definition.parameters_meta_types),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct CSignalDefinition {
    name: *const c_char,
    parameters_count: c_int,
    parameters_meta_types: *const c_int,
}

impl<'a> From<&'a SignalDefinitionWrapper> for CSignalDefinition {
    fn from(definition: &SignalDefinitionWrapper) -> Self {
        CSignalDefinition {
            name: definition.name.as_ptr(),
            parameters_count: definition.parameters_meta_types.len() as c_int,
            parameters_meta_types: definition.parameters_meta_types.as_ptr(),
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
    return_meta_type: c_int,
    parameters_meta_types: Vec<c_int>,
}

impl From<SlotDefinition> for SlotDefinitionWrapper {
    fn from(definition: SlotDefinition) -> Self {
        SlotDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            return_meta_type: definition.return_meta_type as c_int,
            parameters_meta_types: convert_into_metatype(definition.parameters_meta_types),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct CSlotDefinition {
    name: *const c_char,
    return_meta_type: c_int,
    parameters_count: c_int,
    parameters_meta_types: *const c_int,
}

impl<'a> From<&'a SlotDefinitionWrapper> for CSlotDefinition {
    fn from(definition: &SlotDefinitionWrapper) -> Self {
        CSlotDefinition {
            name: definition.name.as_ptr(),
            return_meta_type: definition.return_meta_type as c_int,
            parameters_count: definition.parameters_meta_types.len() as c_int,
            parameters_meta_types: definition.parameters_meta_types.as_ptr(),
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
    property_meta_type: c_int,
    read_slot: CString,
    write_slot: CString,
    notify_slot: CString,
}

impl From<PropertyDefinition> for PropertyDefinitionWrapper {
    fn from(definition: PropertyDefinition) -> Self {
        PropertyDefinitionWrapper {
            name: CString::new(definition.name).unwrap(),
            property_meta_type: definition.property_meta_type as c_int,
            read_slot: CString::new(definition.read_slot).unwrap(),
            write_slot: CString::new(definition.write_slot).unwrap(),
            notify_slot: CString::new(definition.notify_signal).unwrap(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct CPropertyDefinition {
    name: *const c_char,
    property_meta_type: c_int,
    read_slot: *const c_char,
    write_slot: *const c_char,
    notify_signal: *const c_char,
}

impl<'a> From<&'a PropertyDefinitionWrapper> for CPropertyDefinition {
    fn from(definition: &PropertyDefinitionWrapper) -> Self {
        CPropertyDefinition {
            name: definition.name.as_ptr(),
            property_meta_type: definition.property_meta_type as c_int,
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
    use super::{QMetaObject, SignalDefinition, SlotDefinition, PropertyDefinition};
    use qmetatype::QMetaType;
    use std::ptr;

    #[test]
    fn test_qmetaobject_memory() {
        let meta_object = QMetaObject::new_qobject("Meta", Vec::new(), Vec::new(), Vec::new());
        assert_ne!(meta_object.as_ptr(), ptr::null_mut());
    }

    #[test]
    fn test_qmetaobject_memory_with_data() {
        let signal_definitions = vec![
            SignalDefinition {
                name: "testSignal1".to_string(),
                parameters_meta_types: vec![QMetaType::Bool, QMetaType::Int, QMetaType::QString],
            },
            SignalDefinition {
                name: "testSignal2".to_string(),
                parameters_meta_types: vec![],
            }
        ];
        let slot_definitions = vec![
            SlotDefinition {
                name: "testSlot1".to_string(),
                return_meta_type: QMetaType::Void,
                parameters_meta_types: vec![QMetaType::Bool, QMetaType::Int, QMetaType::QString],
            },
            SlotDefinition {
                name: "testSlot2".to_string(),
                return_meta_type: QMetaType::Int,
                parameters_meta_types: vec![],
            }
        ];
        let property_definitions = vec![
            PropertyDefinition {
                name: "testProperty1".to_string(),
                property_meta_type: QMetaType::QString,
                read_slot: "readTestProperty1".to_string(),
                write_slot: "writeTestProperty1".to_string(),
                notify_signal: "testProperty1Changed".to_string()
            },
            PropertyDefinition {
                name: "testProperty2".to_string(),
                property_meta_type: QMetaType::Int,
                read_slot: "readTestProperty2".to_string(),
                write_slot: "writeTestProperty2".to_string(),
                notify_signal: "testProperty2Changed".to_string()
            }
        ];
        let meta_object = QMetaObject::new_qobject("Meta", signal_definitions, slot_definitions, property_definitions);
        assert_ne!(meta_object.as_ptr(), ptr::null_mut());
    }
}