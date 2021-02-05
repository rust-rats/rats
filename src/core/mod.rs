pub mod apply;
pub mod bifunctor;
pub mod foldable;
pub mod flatmap;
pub mod functor;

pub mod prelude {
    pub use super::apply::*;
    pub use super::bifunctor::*;
    pub use super::foldable::*;
    pub use super::flatmap::*;
    pub use super::functor::*;
}
