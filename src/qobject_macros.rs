#[macro_export]
macro_rules! q_object {
    (
        pub $name:ident => $signals:ident {
            $($definitions:tt)*
        }
    ) => {
        q_object_generate_signal_trait!($signals, $($definitions)*);
        q_object_generate_signal_impl!($name, $signals, $($definitions)*);
        q_object_generate_content!($name, new_qobject, $($definitions)*);
    }
}

#[macro_export]
macro_rules! q_object_generate_signal_trait {
    ($signals:ident, $($definitions:tt)*) => {
        trait $signals {
            q_object_generate_signal_definitions!($($definitions)*);
        }
    }
}

#[macro_export]
macro_rules! q_object_generate_signal_definitions {
    (signal fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*) => {
        #[allow(non_snake_case)]
        fn $name(&self, $($param: $paramtype),*);

        q_object_generate_signal_definitions!($($rest)*);
    };
    (slot fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*) => {
        q_object_generate_signal_definitions!($($rest)*);
    };
    (slot fn $name:ident($($param:ident: $paramtype:ident),*) -> $returntype:ident; $($rest:tt)*) => {
        q_object_generate_signal_definitions!($($rest)*);
    };
    (property $name:ident: $propertytype:ident, read: $accessor:ident; $($rest:tt)*) => {
        q_object_generate_signal_definitions!($($rest)*);
    };
    (property $name:ident: $propertytype:ident, read: $accessor:ident, notify: $notifier:ident; $($rest:tt)*) => {
        q_object_generate_signal_definitions!($($rest)*);
    };
    (property $name:ident: $propertytype:ident, read: $accessor:ident, write: $setter:ident, notify: $notifier:ident; $($rest:tt)*) => {
        q_object_generate_signal_definitions!($($rest)*);
    };
    () => {};
}

#[macro_export]
macro_rules! q_object_generate_signal_impl {
    ($name:ident, $signals:ident, $($definitions:tt)*) => {
        impl $signals for $name {
            q_object_generate_signal_implementations!($($definitions)*);
        }
    }
}

#[macro_export]
macro_rules! q_object_generate_signal_implementations {
    (signal fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*) => {
        #[allow(unused_mut)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        fn $name(&self, $($param: $paramtype),*) {
            let mut vec: Vec<QVariant> = Vec::new();
            $(
                vec.push(QVariant::from($param));
            )*
            self.signal_emitter.emit_signal(stringify!($name), vec);
        }

        q_object_generate_signal_implementations!($($rest)*);
    };
    (slot fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*) => {
        q_object_generate_signal_implementations!($($rest)*);
    };
    (slot fn $name:ident($($param:ident: $paramtype:ident),*) -> $returntype:ident; $($rest:tt)*) => {
        q_object_generate_signal_implementations!($($rest)*);
    };
    (property $name:ident: $propertytype:ident, read: $accessor:ident; $($rest:tt)*) => {
        q_object_generate_signal_implementations!($($rest)*);
    };
    (property $name:ident: $propertytype:ident, read: $accessor:ident, notify: $notifier:ident; $($rest:tt)*) => {
        q_object_generate_signal_implementations!($($rest)*);
    };
    (property $name:ident: $propertytype:ident, read: $accessor:ident, write: $setter:ident, notify: $notifier:ident; $($rest:tt)*) => {
        q_object_generate_signal_implementations!($($rest)*);
    };
    () => {};
}

#[macro_export]
macro_rules! q_object_generate_content {
    ($name:ident, $meta:ident, $($definitions:tt)*) => {
        impl QObjectContent for $name {
            #[allow(unused_mut)]
            fn get_metaobject() -> QMetaObject {
                let mut signals = Vec::<SignalDefinition>::new();
                q_object_generate_signal_metas!(signals => {$($definitions)*});

                let mut slots = Vec::<SlotDefinition>::new();
                q_object_generate_slot_metas!(slots => {$($definitions)*});

                let mut properties = Vec::<PropertyDefinition>::new();
                q_object_generate_properties_metas!(properties => {$($definitions)*});

                QMetaObject::$meta(stringify!($name), signals, slots, properties)
            }

            #[allow(dead_code)]
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn invoke_slot(&mut self, name: &str, args: Vec<QVariantRefMut>) -> Option<QVariant> {
                fn next_or_panic(value: Option<QVariantRefMut>) -> QVariantRefMut {
                    if let Some(value) = value {
                        value
                    } else {
                        panic!("Not enough parameters to call a slot")
                    }
                }

                q_object_generate_slot_implementations!(self, args, name => {$($definitions)*});

                panic!("Unrecognized slot call: {}", name)
            }
        }
    }
}

#[macro_export]
macro_rules! q_object_generate_signal_metas {
    ($signals:ident => {signal fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        {
            let mut parameters_definitions = Vec::<ParameterDefinition>::new();
            $(parameters_definitions.push(ParameterDefinition::new(stringify!($param), $paramtype::metatype()));)*
            $signals.push(SignalDefinition::new(stringify!($name), parameters_definitions));
        }

        q_object_generate_signal_metas!($signals => {$($rest)*});
    };
    ($signals:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        q_object_generate_signal_metas!($signals => {$($rest)*});
    };
    ($signals:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*) -> $returntype:ident; $($rest:tt)*}) => {
        q_object_generate_signal_metas!($signals => {$($rest)*});
    };
    ($signals:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident; $($rest:tt)*}) => {
        q_object_generate_signal_metas!($signals => {$($rest)*});
    };
    ($signals:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        q_object_generate_signal_metas!($signals => {$($rest)*});
    };
    ($signals:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, write: $setter:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        q_object_generate_signal_metas!($signals => {$($rest)*});
    };
    ($signals:ident => {}) => {};
}

#[macro_export]
macro_rules! q_object_generate_slot_metas {
    ($slots:ident => {signal fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        q_object_generate_slot_metas!($slots => {$($rest)*});
    };
    ($slots:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        {
            let mut parameters_definitions = Vec::<ParameterDefinition>::new();
            $(parameters_definitions.push(ParameterDefinition::new(stringify!($param), $paramtype::metatype()));)*
            $slots.push(SlotDefinition::new(stringify!($name), QMetaType::Void, parameters_definitions));
        }

        q_object_generate_slot_metas!($slots => {$($rest)*});
    };
    ($slots:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*) -> $returntype:ident; $($rest:tt)*}) => {
        {
            let mut parameters_definitions = Vec::<ParameterDefinition>::new();
            $(parameters_definitions.push(ParameterDefinition::new(stringify!($param), $paramtype::metatype()));)*
            $slots.push(SlotDefinition::new(stringify!($name), $returntype::metatype(), parameters_definitions));
        }

        q_object_generate_slot_metas!($slots => {$($rest)*});
    };
    ($slots:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident; $($rest:tt)*}) => {
        q_object_generate_slot_metas!($slots => {$($rest)*});
    };
    ($slots:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        q_object_generate_slot_metas!($slots => {$($rest)*});
    };
    ($slots:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, write: $setter:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        q_object_generate_slot_metas!($slots => {$($rest)*});
    };
    ($slots:ident => {}) => {};
}

#[macro_export]
macro_rules! q_object_generate_properties_metas {
    ($properties:ident => {signal fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        q_object_generate_properties_metas!($properties => {$($rest)*});
    };
    ($properties:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        q_object_generate_properties_metas!($properties => {$($rest)*});
    };
    ($properties:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*) -> $returntype:ident; $($rest:tt)*}) => {
        q_object_generate_properties_metas!($properties => {$($rest)*});
    };
    ($properties:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident; $($rest:tt)*}) => {
        $properties.push(PropertyDefinition::new_const(stringify!($name), $propertytype::metatype(), stringify!($accessor)));

        q_object_generate_properties_metas!($properties => {$($rest)*});
    };
    ($properties:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        $properties.push(PropertyDefinition::new_read_only(stringify!($name), $propertytype::metatype(), stringify!($accessor), stringify!($notifier)));

        q_object_generate_properties_metas!($properties => {$($rest)*});
    };
    ($properties:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, write: $setter:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        $properties.push(PropertyDefinition::new_read_write(stringify!($name), $propertytype::metatype(), stringify!($accessor), stringify!($setter), stringify!($notifier)));

        q_object_generate_properties_metas!($properties => {$($rest)*});
    };
    ($properties:ident => {}) => {};
}

#[macro_export]
macro_rules! q_object_generate_slot_implementations {
    ($_self:ident, $args:ident, $caller:ident => {signal fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        q_object_generate_slot_implementations!($_self, $args, $caller => {$($rest)*});
    };
    ($_self:ident, $args:ident, $caller:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*); $($rest:tt)*}) => {
        if $caller == stringify!($name) {
            let mut iter = $args.into_iter();
            $(
                let next = next_or_panic(iter.next());
                let $param = next.into();
            )*
            $_self.$name($($param),*);
            return None
        }

        q_object_generate_slot_implementations!($_self, $args, $caller => {$($rest)*});
    };
    ($_self:ident, $args:ident, $caller:ident => {slot fn $name:ident($($param:ident: $paramtype:ident),*) -> $returntype:ident; $($rest:tt)*}) => {
        if $caller == stringify!($name) {
            let mut iter = $args.into_iter();
            $(
                let next = next_or_panic(iter.next());
                let $param = next.into();
            )*
            return Some($_self.$name($($param),*).into())
        }

        q_object_generate_slot_implementations!($_self, $args, $caller => {$($rest)*});
    };
    ($_self:ident, $args:ident, $caller:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident; $($rest:tt)*}) => {
        q_object_generate_slot_implementations!($_self, $args, $caller => {$($rest)*});
    };
    ($_self:ident, $args:ident, $caller:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        q_object_generate_slot_implementations!($_self, $args, $caller => {$($rest)*});
    };
    ($_self:ident, $args:ident, $caller:ident => {property $name:ident: $propertytype:ident, read: $accessor:ident, write: $setter:ident, notify: $notifier:ident; $($rest:tt)*}) => {
        q_object_generate_slot_implementations!($_self, $args, $caller => {$($rest)*});
    };
    ($_self:ident, $args:ident, $caller:ident => {}) => {};
}

#[cfg(test)]
mod tests {
    use qmetaobject::{ParameterDefinition, PropertyDefinition, QMetaObject, SignalDefinition, SlotDefinition};
    use qmetatype::{QMetaTypable, QMetaType};
    use qobject::{QObject, QObjectContent, QObjectContentConstructor, QSignalEmitter};
    use qvariant::{QVariant, QVariantRefMut};

    q_object! {
        pub TestObject => TestObjectSignals {
            signal fn value_changed(value: i32);
            slot fn set_value(value: i32);
            slot fn get_value() -> i32;
            property value: i32, read: get_value;
            property value2: i32, read: get_value, notify: value_changed;
            property value3: i32, read: get_value, write: set_value, notify: value_changed;
        }
    }

    struct TestObject {
        signal_emitter: Box<QSignalEmitter>,
    }

    impl TestObject {
        fn set_value(&mut self, _: i32) {}
        fn get_value(&self) -> i32 {
            123
        }
    }

    impl QObjectContentConstructor for TestObject {
        fn new(signal_emitter: Box<QSignalEmitter>) -> Self {
            TestObject {
                signal_emitter: signal_emitter,
            }
        }
    }

    #[test]
    fn test_qobject_macros() {
        QObject::<TestObject>::new();
    }

    q_object! {
        pub TestObject2 => TestObject2Signals {}
    }

    struct TestObject2 {}

    impl QObjectContentConstructor for TestObject2 {
        fn new(_: Box<QSignalEmitter>) -> Self {
            TestObject2 {}
        }
    }
}