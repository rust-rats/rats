pub mod monoid;
pub mod semigroup;

pub mod helpers {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id<T>(pub T);
    impl<T> Id<T> {
        #[inline]
        pub fn into_value(self) -> T {
            self.0
        }
    }

    impl<T> From<T> for Id<T> {
        fn from(value: T) -> Self {
            Id(value)
        }
    }

    #[cfg(test)]
    impl<T: quickcheck::Arbitrary + Clone> quickcheck::Arbitrary for Id<T> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Id(T::arbitrary(g))
        }
    }
}

pub trait Kind {
    type Ty<T>;
}

pub mod default_kinds {
    use super::Kind;

    pub struct VecKind;
    impl Kind for VecKind {
        type Ty<T> = Vec<T>;
    }

    pub struct OptionKind;
    impl Kind for OptionKind {
        type Ty<T> = Option<T>;
    }
}

pub mod prelude {
    pub use super::default_kinds::*;
    pub use super::helpers::*;
    pub use super::monoid::*;
    pub use super::semigroup::*;
}
