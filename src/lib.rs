#![allow(incomplete_features)]
#![feature(generic_associated_types)]

pub trait Functor {
    type Inner;
    type Outter<B>: Functor;

    fn map<F, B>(self, f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Outter<B> = Option<B>;

    fn map<F, B>(self, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B,
    {
        match self {
            Some(v) => Some(f(v)),
            None => None,
        }
    }
}

pub fn lift<A: Functor, B, F>(fun: F) -> impl FnMut(A) -> <A as Functor>::Outter<B>
where
    F: FnMut(<A as Functor>::Inner) -> B + Copy,
{
    move |a: A| a.map(fun)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_you_even_lift() {
        let times_two = |x: i32| x * 2;

        let mut lifted = lift::<Option<i32>, _, _>(times_two);

        let value = Some(1i32);
        assert_eq!(lifted(value), Some(2))
    }
}
