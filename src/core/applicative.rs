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

#[cfg(test)]
mod tests {
    use super::Applicative;

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
}
