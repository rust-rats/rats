use super::prelude::{ApplicativeInstance, MonoidKInstance};

pub trait AlternativeTy {
    type Cons<T>: AlternativeInstance<T, Kind = Self>
        + MonoidKInstance<T, Kind = Self>
        + ApplicativeInstance<T, Kind = Self>;
}

pub trait AlternativeInstance<T> {
    #[rustfmt::skip]
    type Kind: AlternativeTy<Cons<T> = Self>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, VecKind};

    use super::*;

    impl AlternativeTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<T> AlternativeInstance<T> for Option<T> {
        type Kind = OptionKind;
    }

    impl AlternativeTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<T> AlternativeInstance<T> for Vec<T> {
        type Kind = VecKind;
    }
}
