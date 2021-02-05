pub mod applicative;
pub mod apply;
pub mod bifunctor;
pub mod foldable;
pub mod functor;

pub mod prelude {
    pub use super::applicative::*;
    pub use super::apply::*;
    pub use super::bifunctor::*;
    pub use super::foldable::*;
    pub use super::functor::*;
}
