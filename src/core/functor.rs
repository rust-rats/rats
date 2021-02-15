pub trait Functor {
    type Inner;
    type Outter<B>: Functor;

    fn fmap<F, B>(self, f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Outter<B> = Option<B>;

    fn fmap<F, B>(self, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B,
    {
        match self {
            Some(v) => Some(f(v)),
            None => None,
        }
    }
}

impl<A, E> Functor for Result<A, E> {
    type Inner = A;
    type Outter<B> = Result<B, E>;

    fn fmap<F, B>(self, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B,
    {
        match self {
            Ok(v) => Ok(f(v)),
            Err(e) => Err(e),
        }
    }
}

impl<A> Functor for Vec<A> {
    type Inner = A;
    type Outter<B> = Vec<B>;

    fn fmap<F, B>(self, f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

// TODO handle these bounds
// impl<A: Eq + Hash> Functor for HashSet<A> {
//     type Inner = A;
//     type Outter<B> = HashSet<B>;

//     fn fmap<F, B>(self, f: F) -> Self::Outter<B>
//     where
//         F: FnMut(Self::Inner) -> B,
//     {
//         self.into_iter().map(f).collect()
//     }
// }
//

pub fn lift<A: Functor, B>(
    fun: impl FnMut(<A as Functor>::Inner) -> B + Copy,
) -> impl FnMut(A) -> <A as Functor>::Outter<B> {
    move |a: A| a.fmap(fun)
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

#[cfg(test)]
mod laws {
    type OptionUsize = Option<usize>;
    type ResultUsizeUsize = Result<usize, usize>;
    type VecUsize = Vec<usize>;

    macro_rules! preserve_identity {
        ($($t:ty),*) => {$(
            paste::paste! {
                preserve_identity!([<preserve_identity_ $t>]: $t);
            }
        )*};
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case, unused_imports)]
            #[quickcheck]
            fn $name(n1: $t) -> bool {
                use crate::core::prelude::*;
                use std::convert::identity;
                let n1_copy = n1.clone();

                let left = n1.fmap(identity);
                let right = n1_copy;

                left == right
            }
        };
    }

    mod preserve_identity {
        use super::*;

        preserve_identity!(OptionUsize, ResultUsizeUsize, VecUsize);
    }

    mod composition_identity {
        use super::*;

        #[quickcheck]
        fn composition_identity_vec_usize(vec: Vec<usize>) {
            use crate::core::prelude::*;
            let f1 = |a: usize| (a / 5) * 2;
            let f2 = |a: usize| (a / 5) * 3;
            let vec_copy = vec.clone();

            assert_eq!(vec.fmap(f1).fmap(f2), vec_copy.fmap(|a| f2(f1(a))))
        }

        #[quickcheck]
        fn composition_identity_option_usize(opt: Option<usize>) {
            use crate::core::prelude::*;
            let f1 = |a: usize| (a / 5) * 2;
            let f2 = |a: usize| (a / 5) * 3;
            let opt_copy = opt;

            assert_eq!(opt.fmap(f1).fmap(f2), opt_copy.fmap(|a| f2(f1(a))))
        }

        #[quickcheck]
        fn composition_identity_result_usize_usize(res: Result<usize, usize>) {
            use crate::core::prelude::*;
            let f1 = |a: usize| (a / 5) * 2;
            let f2 = |a: usize| (a / 5) * 3;
            let res_copy = res;

            assert_eq!(res.fmap(f1).fmap(f2), res_copy.fmap(|a| f2(f1(a))))
        }
    }
}
