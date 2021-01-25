pub mod bifunctor;
pub mod functor;
pub mod apply;

pub mod prelude {
    pub use super::bifunctor::*;
    pub use super::functor::*;
    pub use super::apply::*;
}
