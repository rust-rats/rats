use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

impl<A: Semigroup> Monoid for Option<A> {
    fn empty() -> Self {
        None
    }
}

impl<A: Semigroup + Monoid, E> Monoid for Result<A, E> {
    fn empty() -> Self {
        Ok(A::empty())
    }
}

impl<A> Monoid for Vec<A> {
    fn empty() -> Self {
        Vec::new()
    }
}

impl<A: Eq + Hash> Monoid for HashSet<A> {
    fn empty() -> Self {
        HashSet::new()
    }
}

impl<K: Eq + Hash, V> Monoid for HashMap<K, V> {
    fn empty() -> Self {
        HashMap::new()
    }
}

macro_rules! impl_monoid_defaultable {
    ($($t:ty),*) => {$(
        impl Monoid for $t {
            fn empty() -> Self {
                use std::default::Default;
                Default::default()
            }
        }
    )*};
}

impl_monoid_defaultable!(
    String,
    i8,
    i16,
    i32,
    i64,
    usize,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
    ()
);

#[cfg(test)]
mod laws {
    use std::collections::{HashMap, HashSet};

    macro_rules! left_identity {
        ($($t:ty),*) => {$(
            paste::paste! {
                left_identity!([<left_identity_ $t>]: $t);
            }
        )*};
        ($name:ident: f64) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f64) {
                use crate::kernel::prelude::*;
                use float_cmp::{approx_eq, F64Margin};
                let n1_copy = n1.clone();

                assert!(approx_eq!(f64, <f64 as Monoid>::empty().combine(n1), n1_copy, F64Margin::default()))
            }
        };
        ($name:ident: f32) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f32) {
                use crate::kernel::prelude::*;
                use float_cmp::{approx_eq, F32Margin};
                let n1_copy = n1.clone();

                assert!(approx_eq!(f32, <f32 as Monoid>::empty().combine(n1), n1_copy, F32Margin::default()))
            }
        };
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: $t) {
                use crate::kernel::prelude::*;
                let n1_copy = n1.clone();

                assert_eq!(<$t as Monoid>::empty().combine(n1), n1_copy)
            }
        };
    }

    macro_rules! right_identity {
        ($($t:ty),*) => {$(
            paste::paste! {
                right_identity!([<right_identity_ $t>]: $t);
            }
        )*};
        ($name:ident: f64) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f64) {
                use crate::kernel::prelude::*;
                use float_cmp::{approx_eq, F64Margin};
                let n1_copy = n1.clone();

                assert!(approx_eq!(f64, <f64 as Monoid>::empty().combine(n1), n1_copy, F64Margin::default()))
            }
        };
        ($name:ident: f32) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f32) {
                use crate::kernel::prelude::*;
                use float_cmp::{approx_eq, F32Margin};
                let n1_copy = n1.clone();

                assert!(approx_eq!(f32, <f32 as Monoid>::empty().combine(n1), n1_copy, F32Margin::default()))
            }
        };
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: $t) {
                use crate::kernel::prelude::*;
                let n1_copy = n1.clone();

                assert_eq!(n1.combine(<$t as Monoid>::empty()), n1_copy)
            }
        };
    }

    type VecUsize = Vec<usize>;
    type SetUsize = HashSet<usize>;
    type HashMapUsizeUsize = HashMap<usize, usize>;
    type OptionUsize = Option<usize>;
    type ResultUsizeUsize = Result<usize, usize>;
    type Unit = ();

    mod left_identity {
        use super::*;
        left_identity!(
            usize,
            u64,
            String,
            Unit,
            VecUsize,
            SetUsize,
            HashMapUsizeUsize,
            OptionUsize,
            ResultUsizeUsize,
            f64,
            f32
        );
    }

    mod right_identity {
        use super::*;
        right_identity!(
            usize,
            u64,
            String,
            Unit,
            VecUsize,
            SetUsize,
            HashMapUsizeUsize,
            OptionUsize,
            ResultUsizeUsize,
            f64,
            f32
        );
    }
}
