use super::functor::Functor;

pub trait Pointed : Functor {
    fn wrap<B>(b: B) -> Self::Outter<B>;
}

impl<A> Pointed for Option<A> {
    fn wrap<B>(b: B) -> Self::Outter<B> {
        Some(b)
    }
}

pub trait Apply : Pointed {
    fn apply<B, F>(a: Self::Inner, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B
    {
        Self::wrap(f(a))
    }
}