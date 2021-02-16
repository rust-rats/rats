use super::prelude::{SemigroupK, SemigroupKInstance, SemigroupKTy};

#[derive(Copy, Clone, Default)]
pub struct MonoidK;

impl MonoidK {
    pub fn empty<Kind: MonoidKTy, T>(_: Kind) -> Kind::Cons<T> {
        Kind::Cons::<T>::empty()
    }
}

pub trait MonoidKTy {
    type Cons<T>: MonoidKInstance<T, Kind = Self> + SemigroupKInstance<T, Kind = Self>;
}

pub trait MonoidKInstance<T> {
    type Kind: MonoidKTy + SemigroupKTy;

    fn empty() -> <Self::Kind as MonoidKTy>::Cons<T>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, VecKind};

    use super::*;

    impl MonoidKTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<T> MonoidKInstance<T> for Option<T> {
        type Kind = OptionKind;

        fn empty() -> Option<T> {
            None
        }
    }

    impl MonoidKTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<T> MonoidKInstance<T> for Vec<T> {
        type Kind = VecKind;

        fn empty() -> Vec<T> {
            vec![]
        }
    }
}
