use crate::kernel::prelude::Id;

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

impl<A> Functor for Id<A> {
    type Inner = A;

    type Outter<B> = Id<B>;

    fn fmap<F, B>(self, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B,
    {
        Id(f(self.into_value()))
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
    use crate::kernel::prelude::Id;

    type OptionUsize = Option<usize>;
    type IdUsize = Id<usize>;
    type ResultUsizeUsize = Result<usize, usize>;
    type Idf64 = Id<f64>;
    type VecUsize = Vec<usize>;
    type IdUnit = Id<()>;

    macro_rules! preserve_identity {
        ($($t:ty),*) => {$(
            paste::paste! {
                preserve_identity!([<preserve_identity_ $t>]: $t);
            }
        )*};
        ($name:ident: f64) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f64) {
                use crate::core::prelude::*;
                use float_cmp::{approx_eq, F64Margin};
                use std::convert::identity;
                let n1_copy = n1.clone();

                assert!(approx_eq!(f64, n1.fmap(identity), n1_copy, F64Margin::default()))
            }
        };
        ($name:ident: f32) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f32) {
                use crate::core::prelude::*;
                use float_cmp::{approx_eq, F32Margin};
                use std::convert::identity;
                let n1_copy = n1.clone();

                assert!(approx_eq!(f32, n1.fmap(identity), n1_copy, F32Margin::default()))
            }
        };
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case, unused_imports)]
            #[quickcheck]
            fn $name(n1: $t) {
                use crate::core::prelude::*;
                use std::convert::identity;
                let n1_copy = n1.clone();

                assert_eq!(n1.fmap(identity), n1_copy)
            }
        };
    }

    macro_rules! composition_identity_id {
        ($($t:ty),*) => {$(
            paste::paste! {
                composition_identity_id!([<composition_identity_ $t>]: $t);
            }
        )*};
        ($name:ident: f64) => {
            #[allow(non_snake_case, unused_imports)]
            #[quickcheck]
            fn $name(n1: Id<f64>) {
                use crate::core::prelude::*;
                use float_cmp::{approx_eq, F64Margin};
                use std::convert::identity;
                let f1 = |a: f64| a * 2f64;
                let f2 = |a: f64| a * 3f64;
                let n1_copy = n1.clone();

                assert!(approx_eq!(f64, n1.fmap(f1).fmap(f2).into_value(), n1_copy.fmap(|a| f2(f1(a))).into_value(), F64Margin::default()))
            }
        };
        ($name:ident: f32) => {
            #[allow(non_snake_case, unused_imports)]
            #[quickcheck]
            fn $name(n1: Id<f32>) {
                use crate::core::prelude::*;
                use float_cmp::{approx_eq, F32Margin};
                use std::convert::identity;
                let f1 = |a: f32| a * 2f32;
                let f2 = |a: f32| a * 3f32;
                let n1_copy = n1.clone();

                assert!(approx_eq!(f32, n1.fmap(f1).fmap(f2).into_value(), n1_copy.fmap(|a| f2(f1(a))).into_value(), F32Margin::default()))
            }
        };
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case, unused_imports)]
            #[quickcheck]
            fn $name(n1: Id<$t>) {
                use crate::core::prelude::*;
                use std::convert::identity;
                let f1 = |a: $t| a * 2;
                let f2 = |a: $t| a * 3;
                let n1_copy = n1.clone();

                assert_eq!(n1.fmap(f1).fmap(f2), n1_copy.fmap(|a| f2(f1(a))))
            }
        };
    }

    mod preserve_identity {
        use super::*;

        preserve_identity!(
            OptionUsize,
            IdUsize,
            ResultUsizeUsize,
            Idf64,
            VecUsize,
            IdUnit
        );
    }

    mod composition_identity {
        use super::*;

        composition_identity_id!(f32, f64, usize);

        #[quickcheck]
        fn composition_identity_vec_usize(vec: Vec<usize>) {
            use crate::core::prelude::*;
            let f1 = |a: usize| a * 2;
            let f2 = |a: usize| a * 3;
            let vec_copy = vec.clone();

            assert_eq!(vec.fmap(f1).fmap(f2), vec_copy.fmap(|a| f2(f1(a))))
        }

        #[quickcheck]
        fn composition_identity_option_usize(opt: Option<usize>) {
            use crate::core::prelude::*;
            let f1 = |a: usize| a * 2;
            let f2 = |a: usize| a * 3;
            let opt_copy = opt;

            assert_eq!(opt.fmap(f1).fmap(f2), opt_copy.fmap(|a| f2(f1(a))))
        }

        #[quickcheck]
        fn composition_identity_result_usize_usize(res: Result<usize, usize>) {
            use crate::core::prelude::*;
            let f1 = |a: usize| a * 2;
            let f2 = |a: usize| a * 3;
            let res_copy = res;

            assert_eq!(res.fmap(f1).fmap(f2), res_copy.fmap(|a| f2(f1(a))))
        }
    }
}
