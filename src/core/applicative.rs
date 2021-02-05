use super::apply::Apply;
use crate::kernel::helpers::Id;

pub trait Applicative: Apply {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner>;
}

impl<A> Applicative for Option<A> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        Some(value)
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        Ok(value)
    }
}

impl<A: Clone> Applicative for Vec<A> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        vec![value]
    }
}

impl<A> Applicative for Id<A> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        Id(value)
    }
}

pub trait ApplicativeError: Applicative {
    type ErrorT;

    fn handle_error_with<F>(self, f: F) -> Self::Outter<Self::Inner>
    where
        F: FnMut(Self::ErrorT) -> Self::Outter<Self::Inner>;

    fn raise_error(error: Self::ErrorT) -> Self::Outter<Self::Inner>;
}

impl<A, E> ApplicativeError for Result<A, E> {
    type ErrorT = E;

    fn handle_error_with<F>(self, mut f: F) -> Self::Outter<Self::Inner>
    where
        F: FnMut(Self::ErrorT) -> Self::Outter<Self::Inner>,
    {
        match self {
            Err(e) => f(e),
            _ => self,
        }
    }

    fn raise_error(error: Self::ErrorT) -> Self::Outter<Self::Inner> {
        Err(error)
    }
}

impl<A> ApplicativeError for Option<A> {
    type ErrorT = ();

    fn handle_error_with<F>(self, mut f: F) -> Self::Outter<Self::Inner>
    where
        F: FnMut(Self::ErrorT) -> Self::Outter<Self::Inner>,
    {
        match self {
            None => f(()),
            _ => self,
        }
    }

    fn raise_error(_error: Self::ErrorT) -> Self::Outter<Self::Inner> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Applicative, ApplicativeError};

    #[test]
    fn option() {
        let value = 3;
        assert_eq!(Option::pure(value), Some(value));
    }

    #[test]
    fn result() {
        let value = 3;
        assert_eq!(Result::<i32, ()>::pure(value), Ok(value));
    }

    #[test]
    fn vec() {
        let value = 3;
        assert_eq!(Vec::pure(value), vec![value]);
    }

    #[test]
    fn id() {
        use crate::kernel::prelude::Id;

        let value = 3;
        assert_eq!(Id::pure(value), Id(3));
    }

    #[test]
    fn handle_error_with_for_result() {
        let value = Err(());
        let handler = |_err| Ok(3);
        assert_eq!(value.handle_error_with(handler), Ok(3));
    }

    #[test]
    fn raise_error_for_result() {
        let err = Result::<u64, String>::raise_error("ERROR!".to_string());
        assert_eq!(err, Err("ERROR!".to_string()));
    }

    #[test]
    fn handle_error_with_for_option() {
        let value = None;
        let handler = |_| Some(3);
        assert_eq!(value.handle_error_with(handler), Some(3));
    }

    #[test]
    fn raise_error_for_option() {
        let err = Option::<u64>::raise_error(());
        assert_eq!(err, None);
    }
}
