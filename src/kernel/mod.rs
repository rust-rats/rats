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
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            Id(T::arbitrary(g))
        }
    }
}

pub mod prelude {
    pub use super::helpers::*;
    pub use super::monoid::*;
    pub use super::semigroup::*;
}
