use super::{
    applicative::Applicative,
    flatmap::{FlatMapInstance, FlatMapTy},
    prelude::{ApplicativeTy, ApplyInstance, ApplyTy, FunctorTy},
};
use super::{
    flatmap::FlatMap,
    prelude::{ApplicativeInstance, FunctorInstance},
};

pub trait MonadTy {
    type Cons<T>: ApplicativeInstance<Kind = Self>
        + FlatMapInstance<T, Kind = Self>
        + ApplyInstance<T, Kind = Self>
        + FunctorInstance<T, Kind = Self>;
}

pub trait MonadInstance {
    type Kind: MonadTy + ApplicativeTy + FlatMapTy + ApplyTy + FunctorTy;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl MonadTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<A> MonadInstance for Option<A> {
        type Kind = OptionKind;
    }

    impl<E> MonadTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<A, E> MonadInstance for Result<A, E> {
        type Kind = ResultKindOk<E>;
    }

    impl MonadTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<A: Clone> MonadInstance for Vec<A> {
        type Kind = VecKind;
    }
}
