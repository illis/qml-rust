pub trait QMetaTypable {
    fn metatype() -> QMetaType;
}

impl QMetaTypable for bool {
    fn metatype() -> QMetaType {
        QMetaType::Bool
    }
}

impl QMetaTypable for i32 {
    fn metatype() -> QMetaType {
        QMetaType::Int
    }
}

impl QMetaTypable for f64 {
    fn metatype() -> QMetaType {
        QMetaType::Double
    }
}

impl QMetaTypable for String {
    fn metatype() -> QMetaType {
        QMetaType::QString
    }
}

impl QMetaTypable for f32 {
    fn metatype() -> QMetaType {
        QMetaType::Float
    }
}

pub enum QMetaType {
    Void = 43,
    Bool = 1,
    Int = 2,
    Double = 6,
    Long = 32,
    QString = 10,
    Float = 38,
    QVariantList = 9,
    QVariantMap = 8,
    QObject = 39,
}
