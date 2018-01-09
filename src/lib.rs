#[macro_use]
extern crate error_chain;
extern crate futures;

mod conversions;
pub mod errors;
mod internal;
// mod qlistmodel;
// #[macro_use]
// mod qlistmodel_macros;
// mod qmetaobject;
mod qmetatype;
// mod qmlregister;
// #[macro_use]
// mod qmlregister_macros;
// mod qobject;
// #[macro_use]
// mod qobject_macros;
mod qquickview;
mod qurl;
mod qvariant;
mod qvariantmap;

pub use conversions::{TryFrom, TryInto};
/*
pub use qlistmodel::{
    QListModel,
    QListModelContentConstructor,
    QListModelInterface,
    QListModelItem,
    QQmlListModel,
};
pub use qmetaobject::{
    ParameterDefinition,
    PropertyDefinition,
    QMetaObject,
    SignalDefinition,
    SlotDefinition,
};
*/
pub use qmetatype::{QMetaTypable, QMetaType};
/*
pub use qmlregister::{qml_register_type, QmlRegisterType, QmlRegisterableObject};
pub use qobject::{
    QObject,
    QObjectContent,
    QObjectContentConstructor,
    QObjectRefMut,
    QQmlObject,
    QSignalEmitter,
};
*/
pub use qquickview::QQuickView;
pub use qurl::QUrl;
pub use qvariant::QVariant;
pub use qvariantmap::QVariantMap;
