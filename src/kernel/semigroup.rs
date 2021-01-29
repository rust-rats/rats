use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Semigroup {
    fn combine(self, b: Self) -> Self;
}

impl<K: Eq + Hash, V> Semigroup for HashMap<K, V> {
    fn combine(mut self, b: Self) -> Self {
        self.extend(b);
        self
    }
}

impl<A: Eq + Hash> Semigroup for HashSet<A> {
    fn combine(mut self, b: Self) -> Self {
        self.extend(b);
        self
    }
}

impl<A> Semigroup for Vec<A> {
    fn combine(mut self, mut b: Self) -> Self {
        self.append(&mut b);
        self
    }
}

impl<A: Semigroup> Semigroup for Option<A> {
    fn combine(self, b: Self) -> Self {
        match (self, b) {
            (None, None) => None,
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => Some(a.combine(b)),
        }
    }
}

impl<A: Semigroup, E> Semigroup for Result<A, E> {
    fn combine(self, b: Self) -> Self {
        match (self, b) {
            (a @ Err(_), _) => a,
            (_, b @ Err(_)) => b,
            (Ok(a), Ok(b)) => Ok(a.combine(b)),
        }
    }
}

impl Semigroup for String {
    fn combine(mut self, b: Self) -> Self {
        self.push_str(&b);
        self
    }
}

impl Semigroup for () {
    fn combine(self, _: Self) -> Self {}
}

macro_rules! impl_semigroup_wrapping {
    ($($t:ty),*) => {$(
        impl Semigroup for $t {
            fn combine(self, b: Self) -> Self {
                self.wrapping_add(b)
            }
        }
    )*};
}

impl Semigroup for f32 {
    fn combine(self, b: f32) -> Self {
        self + b
    }
}
impl Semigroup for f64 {
    fn combine(self, b: f64) -> Self {
        self + b
    }
}

impl_semigroup_wrapping!(i8, i16, i32, i64, u8, u16, u32, u64, usize);

#[cfg(test)]
mod laws {

    macro_rules! semigroup_associativity {
        ($($t:ty),*) => {$(
            paste::paste! {
                semigroup_associativity!([<associativity_ $t>]: $t);
            }
        )*};
        ($name:ident: f64) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f64, n2: f64, n3: f64) -> bool {
                use crate::kernel::semigroup::*;
                use float_cmp::{approx_eq, F64Margin};
                let n1_copy = n1.clone();
                let n2_copy = n2.clone();
                let n3_copy = n3.clone();

                let left = n1.combine(n2.combine(n3));
                let right = n1_copy.combine(n2_copy).combine(n3_copy);

                if left.is_nan() && right.is_nan() {
                    true
                } else {
                    approx_eq!(f64, left, right, F64Margin::default())
                }
            }
        };
        ($name:ident: f32) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: f32, n2: f32, n3: f32) -> bool {
                use crate::kernel::semigroup::*;
                use float_cmp::{approx_eq, F32Margin};
                let n1_copy = n1.clone();
                let n2_copy = n2.clone();
                let n3_copy = n3.clone();

                let left = n1.combine(n2.combine(n3));
                let right = n1_copy.combine(n2_copy).combine(n3_copy);

                if left.is_nan() && right.is_nan() {
                    true
                } else {
                    approx_eq!(f32, left, right, F32Margin::default())
                }
            }
        };
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: $t, n2: $t, n3: $t) -> bool {
                use crate::kernel::semigroup::*;
                let n1_copy = n1.clone();
                let n2_copy = n2.clone();
                let n3_copy = n3.clone();

                n1.combine(n2.combine(n3)) == n1_copy.combine(n2_copy).combine(n3_copy)
            }
        };
    }

    mod associativity {
        use std::collections::{HashMap, HashSet};

        type VecUsize = Vec<usize>;
        type SetUsize = HashSet<usize>;
        type HashMapUsizeUsize = HashMap<usize, usize>;
        type OptionUsize = Option<usize>;
        type ResultUsizeUsize = Result<usize, usize>;
        type Unit = ();
        semigroup_associativity!(
            usize,
            u64,
            String,
            // f32, TODO handle this
            // f64, TODO handle this
            Unit,
            VecUsize,
            SetUsize,
            HashMapUsizeUsize,
            OptionUsize,
            ResultUsizeUsize
        );
    }
}
