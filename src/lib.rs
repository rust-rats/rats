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
        let times_two_ref = |x: &i32| x * 2;

        fn plus_one(x: i32) -> i32 {
            x + 1
        }

        let mut lifted_times_two = lift(times_two);
        let mut lifted_plus_one = lift(plus_one);

        let value = Some(2i32);
        assert_eq!(lifted_times_two(value), Some(4));
        {
            // needs new scope to make sure the lifted function
            // does not outlive the parameter
            let mut lifted_times_two_ref = lift(times_two_ref);
            assert_eq!(lifted_times_two_ref(value.as_ref()), Some(4));
        }
        assert_eq!(lifted_plus_one(value), Some(3));
    }
}
