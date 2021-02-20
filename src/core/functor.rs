pub mod functor {
    use super::*;

    #[inline]
    pub fn fmap<Kind: FunctorTy, A, B>(
        _: Kind,
        fa: impl FunctorInstance<A, Kind = Kind>,
        f: impl Fn(&A) -> B,
    ) -> Kind::Cons<B> {
        fa.fmap(f)
    }

    pub fn lift<Kind: FunctorTy, A: FunctorInstance<T, Kind = Kind>, T, B>(
        _: Kind,
        fun: impl Fn(&T) -> B,
    ) -> impl FnOnce(A) -> Kind::Cons<B> {
        move |a: A| a.fmap(fun)
    }
}

pub trait FunctorTy {
    type Cons<T>: FunctorInstance<T, Kind = Self>;
}

pub trait FunctorInstance<T> {
    type Kind: FunctorTy;

    fn fmap<B>(self, f: impl Fn(&T) -> B) -> <Self::Kind as FunctorTy>::Cons<B>;
}

pub mod std_instances {
    use super::*;
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    impl FunctorTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<A> FunctorInstance<A> for Option<A> {
        type Kind = OptionKind;

        fn fmap<B>(self, f: impl FnOnce(&A) -> B) -> Option<B> {
            match self {
                Some(v) => Some(f(&v)),
                None => None,
            }
        }
    }

    impl FunctorTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<A> FunctorInstance<A> for Vec<A> {
        type Kind = VecKind;

        fn fmap<B>(self, f: impl FnMut(&A) -> B) -> Vec<B> {
            self.iter().map(f).collect()
        }
    }

    impl<E> FunctorTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<A, E> FunctorInstance<A> for Result<A, E> {
        type Kind = ResultKindOk<E>;

        fn fmap<B>(self, f: impl FnOnce(&A) -> B) -> Result<B, E> {
            match self {
                Ok(v) => Ok(f(&v)),
                Err(e) => Err(e),
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::std_kinds::*;

    #[test]
    fn do_you_even_lift() {
        let times_two = |x: &i32| x * 2;

        let lifted_times_two = functor::lift(OptionKind, times_two);

        let value = Some(2i32);
        assert_eq!(lifted_times_two(value), Some(4));
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
                let n1_copy = n1.clone();

                // we have to use clone as identity
                // because the function receives a ref
                let left = n1.fmap(Clone::clone);
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

        #[quickcheck]
        fn composition_identity_vec_usize(vec: Vec<usize>) {
            use crate::core::prelude::*;
            let f1 = |a: &usize| (a / 5) * 2;
            let f2 = |a: &usize| (a / 5) * 3;
            let vec_copy = vec.clone();

            assert_eq!(vec.fmap(f1).fmap(f2), vec_copy.fmap(|a| f2(&f1(a))))
        }

        #[quickcheck]
        fn composition_identity_option_usize(opt: Option<usize>) {
            use crate::core::prelude::*;
            let f1 = |a: &usize| (a / 5) * 2;
            let f2 = |a: &usize| (a / 5) * 3;
            let opt_copy = opt;

            assert_eq!(opt.fmap(f1).fmap(f2), opt_copy.fmap(|a| f2(&f1(a))))
        }

        #[quickcheck]
        fn composition_identity_result_usize_usize(res: Result<usize, usize>) {
            use crate::core::prelude::*;
            let f1 = |a: &usize| (a / 5) * 2;
            let f2 = |a: &usize| (a / 5) * 3;
            let res_copy = res;

            assert_eq!(res.fmap(f1).fmap(f2), res_copy.fmap(|a| f2(&f1(a))))
        }
    }
}
