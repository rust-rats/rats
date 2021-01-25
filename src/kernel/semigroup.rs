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
    fn combine(self, _: Self) -> Self {
        ()
    }
}

macro_rules! impl_semigroup {
    ($($t:ty),*) => {$(
        impl Semigroup for $t {
            fn combine(self, b: Self) -> Self {
                self + b
            }
        }
    )*};
}

impl_semigroup!(i8, i16, i32, i64, u8, u16, u32, u64, usize, f32, f64);

#[cfg(test)]
mod laws {

    macro_rules! semigroup_associativity {
        ($($t:ty),*) => {$(
            paste::paste! {
                semigroup_associativity!([<associativity_ $t>]: $t);
            }
        )*};
        ($name:ident: $t:ty) => {
            #[allow(non_snake_case)]
            #[quickcheck]
            fn $name(n1: $t, n2: $t, n3: $t) {
                use crate::kernel::semigroup::*;
                let n1_copy = n1.clone();
                let n2_copy = n2.clone();
                let n3_copy = n3.clone();

                assert_eq!(n1.combine(n2.combine(n3)), n1_copy.combine(n2_copy).combine(n3_copy))
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
            Unit,
            VecUsize,
            SetUsize,
            HashMapUsizeUsize,
            OptionUsize,
            ResultUsizeUsize
        );
    }
}
