use super::flatmap::FlatMapInstance;

pub trait MonadTy {
    type Cons<T>: MonadInstance<Kind = Self> + FlatMapInstance<T, Kind = Self>;
}

pub trait MonadInstance {
    type Kind: MonadTy;
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

    impl<A> MonadInstance for Vec<A> {
        type Kind = VecKind;
    }
}
