#![feature(generic_associated_types)]

pub trait Functor {
    type Inner;
    type Outter<B>: Functor;

    fn map<F, B>(self, f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B;
}
