use super::functor::Functor;
use crate::kernel::prelude::Id;

pub trait Apply : Functor {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B;
}

impl<A> Apply for Option<A> {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B
    {
        self.fmap(f?)
    }
}

impl<A, E> Apply for Result<A, E> {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
        where
            F: FnMut(Self::Inner) -> B
    {
        self.fmap(f?)
    }
}

impl<A> Apply for Id<A> {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
        where
            F: FnMut(Self::Inner) -> B
    {
        self.fmap(f.into_value())
    }
}
