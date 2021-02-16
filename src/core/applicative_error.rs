use super::{
    applicative::ApplicativeInstance,
    prelude::{ApplyInstance, FunctorInstance},
};

pub trait ApplicativeErrorTy {
    type Cons<T>: ApplicativeErrorInstance<T, Kind = Self>
        + ApplicativeInstance<Kind = Self>
        + ApplyInstance<T, Kind = Self>
        + FunctorInstance<T, Kind = Self>;
}

pub trait ApplicativeErrorInstance<T> {
    type Kind: ApplicativeErrorTy;
    type ErrorT;

    fn handle_error_with(
        self,
        f: impl Fn(&Self::ErrorT) -> <Self::Kind as ApplicativeErrorTy>::Cons<T>,
    ) -> <Self::Kind as ApplicativeErrorTy>::Cons<T>;

    fn raise_error(error: Self::ErrorT) -> <Self::Kind as ApplicativeErrorTy>::Cons<T>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk};

    use super::*;

    impl<E> ApplicativeErrorTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<A, E> ApplicativeErrorInstance<A> for Result<A, E> {
        type Kind = ResultKindOk<E>;
        type ErrorT = E;

        fn handle_error_with(self, f: impl FnOnce(&Self::ErrorT) -> Result<A, E>) -> Result<A, E> {
            match self {
                Err(e) => f(&e),
                _ => self,
            }
        }

        fn raise_error(error: Self::ErrorT) -> Result<A, E> {
            Err(error)
        }
    }

    impl ApplicativeErrorTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<A> ApplicativeErrorInstance<A> for Option<A> {
        type Kind = OptionKind;
        type ErrorT = ();

        fn handle_error_with(self, f: impl FnOnce(&Self::ErrorT) -> Option<A>) -> Option<A> {
            match self {
                None => f(&()),
                _ => self,
            }
        }

        fn raise_error(_error: Self::ErrorT) -> Option<A> {
            None
        }
    }
}
