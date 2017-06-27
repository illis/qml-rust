pub(crate) fn convert_into<T, U: From<T>>(input: Vec<T>) -> Vec<U> {
    input.into_iter()
        .map(|item| U::from(item))
        .collect()
}

pub(crate) fn convert_as<'a, T, U: From<&'a T>>(input: &'a Vec<T>) -> Vec<U> {
    input.iter()
        .map(|item| U::from(item))
        .collect()
}