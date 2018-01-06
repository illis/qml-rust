use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

use qmetaobject::{PropertyDefinition, SignalDefinition, SlotDefinition};
use qmetaobject::conversionutils::{convert_as, convert_into};
use qmetaobject::propertydefinition::CPropertyDefinition;
use qmetaobject::signaldefinition::CSignalDefinition;
use qmetaobject::slotdefinition::CSlotDefinition;

pub struct QMetaObject {
    ptr: *mut c_void,
}

impl QMetaObject {
    pub fn new_qobject(
        name: &str,
        signal_definitions: Vec<SignalDefinition>,
        slot_definitions: Vec<SlotDefinition>,
        property_definitions: Vec<PropertyDefinition>,
    ) -> Self {
        QMetaObject::new(
            name,
            QMetaObject::qobject_metaobject(),
            signal_definitions,
            slot_definitions,
            property_definitions,
        )
    }

    pub fn new_qlistmodel(
        name: &str,
        signal_definitions: Vec<SignalDefinition>,
        slot_definitions: Vec<SlotDefinition>,
        property_definitions: Vec<PropertyDefinition>,
    ) -> Self {
        QMetaObject::new(
            name,
            QMetaObject::qlistmodel_metaobject(),
            signal_definitions,
            slot_definitions,
            property_definitions,
        )
    }

    pub fn qobject_metaobject() -> Self {
        QMetaObject {
            ptr: unsafe { dos_qobject_qmetaobject() },
        }
    }

    pub fn qlistmodel_metaobject() -> Self {
        QMetaObject {
            ptr: unsafe { de_qlistmodel_qmetaobject() },
        }
    }

    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.ptr
    }

    pub(crate) fn as_ptr_mut(&mut self) -> *mut c_void {
        self.ptr
    }

    fn new(
        name: &str,
        qmeta: QMetaObject,
        signal_definitions: Vec<SignalDefinition>,
        slot_definitions: Vec<SlotDefinition>,
        property_definitions: Vec<PropertyDefinition>,
    ) -> Self {
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

        let name = CString::new(name).unwrap();
        let ptr = unsafe {
            dos_qmetaobject_create(
                qmeta.ptr,
                name.as_ptr(),
                &c_signal_definitions,
                &c_slot_definitions,
                &c_property_definitions,
            )
        };
        QMetaObject { ptr }
    }
}

impl Drop for QMetaObject {
    fn drop(&mut self) {
        unsafe {
            dos_qmetaobject_delete(self.ptr);
        }
    }
}

#[repr(C)]
struct CSignalDefinitions {
    count: c_int,
    definitions: *const CSignalDefinition,
}

#[repr(C)]
struct CSlotDefinitions {
    count: c_int,
    definitions: *const CSlotDefinition,
}

#[repr(C)]
struct CPropertyDefinitions {
    count: i32,
    definitions: *const CPropertyDefinition,
}

extern "C" {
    fn dos_qobject_qmetaobject() -> *mut c_void;
    fn de_qlistmodel_qmetaobject() -> *mut c_void;
    fn dos_qmetaobject_create(
        super_class_meta_object: *mut c_void,
        class_name: *const c_char,
        signal_definitions: *const CSignalDefinitions,
        slot_definitions: *const CSlotDefinitions,
        property_definitions: *const CPropertyDefinitions,
    ) -> *mut c_void;
    fn dos_qmetaobject_delete(vptr: *mut c_void);
}

#[cfg(test)]
mod tests {
    use super::QMetaObject;
    use qmetaobject::{ParameterDefinition, PropertyDefinition, SignalDefinition, SlotDefinition};
    use qmetatype::QMetaType;

    #[test]
    fn test_qobject_qmetaobject_memory() {
        QMetaObject::new_qobject("Meta", Vec::new(), Vec::new(), Vec::new());
    }

    #[test]
    fn test_qlistmodel_qmetaobject_memory() {
        QMetaObject::new_qlistmodel("Meta", Vec::new(), Vec::new(), Vec::new());
    }

    #[test]
    fn test_qobject_qmetaobject_memory_with_data() {
        let signal_definitions = vec![
            SignalDefinition::new(
                "testignal1",
                vec![
                    ParameterDefinition::new("first", QMetaType::Bool),
                    ParameterDefinition::new("second", QMetaType::Int),
                    ParameterDefinition::new("third", QMetaType::QString),
                ],
            ),
            SignalDefinition::new("testSignal2", vec![]),
        ];
        let slot_definitions = vec![
            SlotDefinition::new(
                "testSlot1",
                QMetaType::Void,
                vec![
                    ParameterDefinition::new("first", QMetaType::Bool),
                    ParameterDefinition::new("second", QMetaType::Int),
                    ParameterDefinition::new("third", QMetaType::QString),
                ],
            ),
            SlotDefinition::new("testSlot2", QMetaType::Int, vec![]),
        ];
        let property_definitions = vec![
            PropertyDefinition::new_read_write(
                "testProperty1",
                QMetaType::QString,
                "readTestProperty1",
                "writeTestProperty1",
                "testProperty1Changed",
            ),
            PropertyDefinition::new_read_write(
                "testProperty2",
                QMetaType::Int,
                "readTestProperty2",
                "writeTestProperty2",
                "testProperty2Changed",
            ),
        ];
        QMetaObject::new_qobject(
            "Meta",
            signal_definitions,
            slot_definitions,
            property_definitions,
        );
    }

    #[test]
    fn test_qlistmodel_qmetaobject_memory_with_data() {
        let signal_definitions = vec![
            SignalDefinition::new(
                "testignal1",
                vec![
                    ParameterDefinition::new("first", QMetaType::Bool),
                    ParameterDefinition::new("second", QMetaType::Int),
                    ParameterDefinition::new("third", QMetaType::QString),
                ],
            ),
            SignalDefinition::new("testSignal2", vec![]),
        ];
        let slot_definitions = vec![
            SlotDefinition::new(
                "testSlot1",
                QMetaType::Void,
                vec![
                    ParameterDefinition::new("first", QMetaType::Bool),
                    ParameterDefinition::new("second", QMetaType::Int),
                    ParameterDefinition::new("third", QMetaType::QString),
                ],
            ),
            SlotDefinition::new("testSlot2", QMetaType::Int, vec![]),
        ];
        let property_definitions = vec![
            PropertyDefinition::new_read_write(
                "testProperty1",
                QMetaType::QString,
                "readTestProperty1",
                "writeTestProperty1",
                "testProperty1Changed",
            ),
            PropertyDefinition::new_read_write(
                "testProperty2",
                QMetaType::Int,
                "readTestProperty2",
                "writeTestProperty2",
                "testProperty2Changed",
            ),
        ];
        QMetaObject::new_qlistmodel(
            "Meta",
            signal_definitions,
            slot_definitions,
            property_definitions,
        );
    }
}
