/*
use qobject::QObjectWrapper;

#[macro_export]
macro_rules! q_object {
    (
        pub $obj:ident as $wrapper:ident {
            signals:
                $(fn $signalname:ident($($signalvar:ident : $signalqtype:ident),*);)*
            slots:
                $(fn $slotname:ident ($($slotvar:ident : $slotqtype:ident),*);)*
            properties:
                $($propname:ident : $proptype:ident; read: $read_slot:ident, write: $write_slot:ident, notify: $notify_sig:ident;)*
        }
    ) =>{
        pub struct $wrapper {
            wrapper: QObjectWrapper,
            content: $obj,
        }

        impl $wrapper {
            fn new() -> $wrapper {
                $wrapper {
                    wrapper: {

                    },
                    content: $obj::new(),
                }
            }
        }
    }
}
*/