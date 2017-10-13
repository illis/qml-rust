pub(crate) fn convert_into<T, U: From<T>>(input: Vec<T>) -> Vec<U> {
    input.into_iter()
        .map(U::from)
        .collect()
}

pub(crate) fn convert_as<'a, T, U: From<&'a T>>(input: &'a [T]) -> Vec<U> {
    input.iter()
        .map(U::from)
        .collect()
}