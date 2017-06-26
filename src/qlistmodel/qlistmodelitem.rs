use qvariant::QVariant;
use std::collections::HashMap;

pub trait QListModelItem {
    fn role_names() -> Vec<&'static str>;
    fn to_variant_map<'a>(&self) -> HashMap<&'static str, QVariant<'a>>;
    fn from_variant_map<'a>(input: HashMap<&'static str, QVariant<'a>>) -> Self;
}