use super::applicative::Applicative;
use super::flatmap::FlatMap;

pub trait Monad: Applicative + FlatMap {}

impl<A> Monad for Option<A> {}

impl<A, E> Monad for Result<A, E> {}

impl<A: Clone> Monad for Vec<A> {}
