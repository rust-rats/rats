pub mod applicative;
pub mod apply;
pub mod bifunctor;
pub mod flatmap;
pub mod foldable;
pub mod functor;
pub mod monad;
pub mod semigroup_k;

pub mod prelude {
    pub use super::applicative::*;
    pub use super::apply::*;
    pub use super::bifunctor::*;
    pub use super::flatmap::*;
    pub use super::foldable::*;
    pub use super::functor::*;
    pub use super::monad::*;
    pub use super::semigroup_k::*;
}
