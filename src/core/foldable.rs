use crate::kernel::prelude::Monoid;

#[derive(Copy, Clone, Default)]
pub struct Foldable;

impl Foldable {
    #[inline]
    pub fn fold_left<Kind: FoldableTy, A, B>(
        _: Kind,
        fa: impl FoldableInstance<A, Kind = Kind>,
        start: B,
        f: impl Fn(B, &A) -> B,
    ) -> B {
        fa.fold_left(start, f)
    }

    #[inline]
    pub fn fold_right<Kind: FoldableTy, A, B>(
        _: Kind,
        fa: impl FoldableInstance<A, Kind = Kind>,
        start: B,
        f: impl Fn(&A, B) -> B,
    ) -> B {
        fa.fold_right(start, f)
    }

    #[inline]
    pub fn fold<Kind: FoldableTy, A: Monoid + Copy>(
        // TODO remove the need for this Copy
        _: Kind,
        fa: impl FoldableInstance<A, Kind = Kind>,
    ) -> A {
        fa.fold_left(A::empty(), |acc, x| x.combine(acc))
    }
}

pub trait FoldableTy {
    type Cons<T>: FoldableInstance<T, Kind = Self>;
}

pub trait FoldableInstance<T> {
    type Kind: FoldableTy;

    fn fold_left<B>(self, start: B, f: impl Fn(B, &T) -> B) -> B;
    fn fold_right<B>(self, start: B, f: impl Fn(&T, B) -> B) -> B;
}

pub mod std_instances {
    use crate::core::prelude::VecKind;

    use super::*;

    impl FoldableTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<T> FoldableInstance<T> for Vec<T> {
        type Kind = VecKind;

        fn fold_left<B>(self, start: B, f: impl FnMut(B, &T) -> B) -> B {
            self.iter().fold(start, f)
        }

        fn fold_right<B>(self, start: B, mut f: impl FnMut(&T, B) -> B) -> B {
            //TODO should be lazy?
            self.iter().rev().fold(start, |a, b| f(b, a))
        }
    }

    impl<T> FoldableInstance<T> for Option<T> {
        type Kind = VecKind;

        fn fold_left<B>(self, start: B, f: impl FnMut(B, &T) -> B) -> B {
            self.iter().fold(start, f)
        }

        fn fold_right<B>(self, start: B, mut f: impl FnMut(&T, B) -> B) -> B {
            self.iter().rev().fold(start, |a, b| f(b, a))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::std_kinds::*;

    #[quickcheck]
    fn folding_consistent_with_sum(x: Vec<i32>) {
        let cloned = x.clone();

        assert_eq!(
            Foldable::fold(VecKind, x.clone()),
            cloned.iter().fold(0i32, |acc, x| acc.wrapping_add(*x))
        )
    }
}
