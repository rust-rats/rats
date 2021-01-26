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

#[cfg(test)]
mod tests {
    use super::Apply;

    fn to_none<T>(_ : Option<T>) -> Option<T> {
        None
    }

    #[test]
    fn option_some_some() {
        let option = Some(3);
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(option.apply(function), Some(9.0));
    }

    #[test]
    fn option_some_none() {
        let option = Some(3);
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        let function = to_none(function);
        assert_eq!(option.apply(function), None);
    }

    #[test]
    fn option_none_some() {
        let option = None;
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(option.apply(function), None);
    }

    #[test]
    fn option_none_none() {
        let option = None;
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        let function = to_none(function);
        assert_eq!(option.apply(function), None);
    }
}
