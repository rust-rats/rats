use super::prelude::ApplyInstance;

pub mod applicative {
    use super::*;

    #[inline]
    pub fn pure<Kind: ApplicativeTy, A>(_: Kind, value: A) -> Kind::Cons<A> {
        Kind::Cons::<A>::pure(value)
    }
}

pub trait ApplicativeTy {
    type Cons<T>: ApplicativeInstance<T, Kind = Self> + ApplyInstance<T, Kind = Self>;
}

pub trait ApplicativeInstance<T> {
    #[rustfmt::skip]
    type Kind: ApplicativeTy<Cons<T> = Self>;

    fn pure<A>(value: A) -> <Self::Kind as ApplicativeTy>::Cons<A>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl ApplicativeTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<T> ApplicativeInstance<T> for Option<T> {
        type Kind = OptionKind;

        fn pure<A>(value: A) -> Option<A> {
            Some(value)
        }
    }

    impl<E> ApplicativeTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }
    impl<T, E> ApplicativeInstance<T> for Result<T, E> {
        type Kind = ResultKindOk<E>;

        fn pure<A>(value: A) -> Result<A, E> {
            Ok(value)
        }
    }

    impl ApplicativeTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<T> ApplicativeInstance<T> for Vec<T> {
        type Kind = VecKind;

        fn pure<A>(value: A) -> Vec<A> {
            vec![value]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ApplicativeInstance;

    #[test]
    fn option() {
        let value = 3;
        assert_eq!(Option::<()>::pure(value), Some(value));
    }

    #[test]
    fn result() {
        let value = 3;
        assert_eq!(Result::<i32, ()>::pure(value), Ok(value));
    }

    #[test]
    fn vec() {
        let value = 3;
        assert_eq!(Vec::<()>::pure(value), vec![value]);
    }
}
