use super::prelude::ApplyInstance;

#[derive(Copy, Clone)]
pub struct Applicative;

impl Applicative {
    #[inline]
    pub fn pure<Kind: ApplicativeTy, A>(_: Kind, value: A) -> Kind::Cons<A> {
        Kind::Cons::<A>::pure(value)
    }
}

pub trait ApplicativeTy {
    type Cons<T>: ApplicativeInstance<Kind = Self> + ApplyInstance<T, Kind = Self>;
}

pub trait ApplicativeInstance {
    type Kind: ApplicativeTy;

    fn pure<A>(value: A) -> <Self::Kind as ApplicativeTy>::Cons<A>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl ApplicativeTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<T> ApplicativeInstance for Option<T> {
        type Kind = OptionKind;

        fn pure<A>(value: A) -> Option<A> {
            Some(value)
        }
    }

    impl<E> ApplicativeTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }
    impl<T, E> ApplicativeInstance for Result<T, E> {
        type Kind = ResultKindOk<E>;

        fn pure<A>(value: A) -> Result<A, E> {
            Ok(value)
        }
    }

    impl ApplicativeTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<T> ApplicativeInstance for Vec<T> {
        type Kind = VecKind;

        fn pure<A>(value: A) -> Vec<A> {
            vec![value]
        }
    }
}

// #[cfg(test)]
// mod tests {
// use super::{Applicative, ApplicativeError};

// #[test]
// fn option() {
// let value = 3;
// assert_eq!(Option::pure(value), Some(value));
// }

// #[test]
// fn result() {
// let value = 3;
// assert_eq!(Result::<i32, ()>::pure(value), Ok(value));
// }

// #[test]
// fn vec() {
// let value = 3;
// assert_eq!(Vec::pure(value), vec![value]);
// }

// #[test]
// fn handle_error_with_for_result() {
// let value = Err(());
// let handler = |_err| Ok(3);
// assert_eq!(value.handle_error_with(handler), Ok(3));
// }

// #[test]
// fn raise_error_for_result() {
// let err = Result::<u64, String>::raise_error("ERROR!".to_string());
// assert_eq!(err, Err("ERROR!".to_string()));
// }

// #[test]
// fn handle_error_with_for_option() {
// let value = None;
// let handler = |_| Some(3);
// assert_eq!(value.handle_error_with(handler), Some(3));
// }

// #[test]
// fn raise_error_for_option() {
// let err = Option::<u64>::raise_error(());
// assert_eq!(err, None);
// }
// }
