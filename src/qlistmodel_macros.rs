#[macro_export]
macro_rules! q_listmodel {
    (
        pub $name:ident(signal_emitter: $signals:ident, role_names: $($role_name:ident),*) {
            $($definitions:tt)*
        }
    ) => {
        q_object_generate_signal_trait!($signals, $($definitions)*);
        q_object_generate_signal_impl!($name, $signals, $($definitions)*);
        q_object_generate_content!($name, new_qlistmodel, $($definitions)*);
        q_listmodel_generate_content!($name, $($role_name)*);
    }
}

#[macro_export]
macro_rules! q_listmodel_generate_content {
    ($name:ident, $($role_name:ident)*) => {
        impl QListModelContent for $name {
            fn role_names() -> Vec<&'static str> {
                vec![$(stringify!($role_name)),*]
            }
        }
    }
}