use super::{
    applicative::ApplicativeInstance,
    flatmap::FlatMapInstance,
    prelude::{ApplyInstance, FunctorInstance},
};

pub trait MonadTy {
    type Cons<T>: ApplicativeInstance<T, Kind = Self>
        + FunctorInstance<T, Kind = Self>
        + ApplyInstance<T, Kind = Self>
        + MonadInstance<T, Kind = Self>
        + FlatMapInstance<T, Kind = Self>;
}

pub trait MonadInstance<T> {
    #[rustfmt::skip]
    type Kind: MonadTy<Cons<T> = Self>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl MonadTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<A> MonadInstance<A> for Option<A> {
        type Kind = OptionKind;
    }

    impl<E> MonadTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<A, E> MonadInstance<A> for Result<A, E> {
        type Kind = ResultKindOk<E>;
    }

    impl MonadTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<A> MonadInstance<A> for Vec<A> {
        type Kind = VecKind;
    }
}
