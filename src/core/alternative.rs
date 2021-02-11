use super::prelude::{Applicative, SemigroupK};

pub trait Alternative: SemigroupK + Applicative {}

impl<T: SemigroupK + Applicative> Alternative for T {}
