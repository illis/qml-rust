#[macro_export]
macro_rules! q_listmodel {
    (
        pub struct $name:ident(signal_emitter: $signals:ident) {
            $($definitions:tt)*
        }
    ) => {
        q_object_generate_signal_trait!($signals, $($definitions)*);
        q_object_generate_signal_impl!($name, $signals, $($definitions)*);
        q_object_generate_content!($name, new_qlistmodel, $($definitions)*);
    }
}

#[macro_export]
macro_rules! q_listmodelitem {
    (
        pub struct $name:ident {
            $($attribute:ident: $attribute_type:ident,)*
        }
    ) => {
        pub struct $name {
            $($attribute: $attribute_type,)*
        }

        impl QListModelItem for $name {
            fn role_names() -> Vec<&'static str> {
                let mut returned = Vec::new();
                $(
                    returned.push(stringify!($attribute));
                )*
                returned
            }

            fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>> {
                let mut returned = HashMap::new();
                $(
                    returned.insert(stringify!($attribute), QVariant::from(self.$attribute.clone()));
                )*
                returned
            }

            fn from_variant_map<'a>(input: QVariantMap<'a>) -> Self {
                Self {
                    $(
                        $attribute: (&input[stringify!($attribute)]).into(),
                    )*
                }
            }
        }
    }
}
