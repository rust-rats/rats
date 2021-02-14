pub mod alternative;
pub mod applicative;
pub mod apply;
pub mod bifunctor;
pub mod flatmap;
pub mod foldable;
pub mod functor;
pub mod monad;
pub mod monoid_k;
pub mod semigroup_k;

pub mod std_kinds {
    use std::marker::PhantomData;

    #[derive(Copy, Clone, Default)]
    pub struct VecKind;
    #[derive(Copy, Clone, Default)]
    pub struct OptionKind;
    #[derive(Copy, Clone, Default)]
    pub struct ResultKind<E>(PhantomData<E>);
}

pub mod prelude {
    pub use super::alternative::*;
    pub use super::applicative::*;
    pub use super::apply::*;
    pub use super::bifunctor::*;
    pub use super::flatmap::*;
    pub use super::foldable::*;
    pub use super::functor::*;
    pub use super::monad::*;
    pub use super::monoid_k::*;
    pub use super::semigroup_k::*;
    pub use super::std_kinds::*;
}
