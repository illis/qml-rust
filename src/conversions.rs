use errors::Error;

// Pending stabilization
// Cf https://github.com/rust-lang/rust/issues/33417
pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;
    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

// From implies TryFrom
impl<S, T> TryFrom<S> for T where T: From<S> {
    type Error = Error;

    fn try_from(value: S) -> Result<Self, Self::Error> {
        Ok(T::from(value))
    }
}
