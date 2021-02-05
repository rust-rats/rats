use super::applicative::Applicative;
use super::flatmap::FlatMap;
use crate::kernel::prelude::Id;

pub trait Monad: Applicative + FlatMap {}

impl<A> Monad for Option<A> {}

impl<A, E> Monad for Result<A, E> {}

impl<A: Clone> Monad for Vec<A> {}

impl<A> Monad for Id<A> {}
