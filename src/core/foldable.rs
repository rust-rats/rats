use crate::kernel::prelude::Monoid;

pub trait Foldable {
    type Inner;
    type Outter<B>: Foldable;

    fn fold_left<B>(self, start: B, f: impl FnMut(B, Self::Inner) -> B) -> B;
    fn fold_right<B>(self, start: B, f: impl FnMut(B, Self::Inner) -> B) -> B;
}

pub trait MonoidFoldable<T: Monoid>: Foldable<Inner = T>
where
    Self: Sized,
{
    fn folding(self) -> Self::Inner {
        self.fold_left(Self::Inner::empty(), |acc, x| x.combine(acc))
    }
}

impl<E: Foldable<Inner = T>, T: Monoid> MonoidFoldable<T> for E {}

impl<T> Foldable for Vec<T> {
    type Inner = T;

    type Outter<B> = Vec<B>;

    fn fold_left<B>(self, start: B, f: impl FnMut(B, Self::Inner) -> B) -> B {
        self.into_iter().fold(start, f)
    }

    fn fold_right<B>(self, start: B, f: impl FnMut(B, Self::Inner) -> B) -> B {
        //TODO should be lazy?
        self.into_iter().rev().fold(start, f)
    }
}

impl<T> Foldable for Option<T> {
    type Inner = T;

    type Outter<B> = Option<B>;

    fn fold_left<B>(self, start: B, f: impl FnMut(B, Self::Inner) -> B) -> B {
        self.into_iter().fold(start, f)
    }

    fn fold_right<B>(self, start: B, f: impl FnMut(B, Self::Inner) -> B) -> B {
        self.into_iter().rev().fold(start, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn folding_consistent_with_sum(x: Vec<i32>) {
        let cloned = x.clone();

        assert_eq!(
            x.folding(),
            cloned.iter().fold(0i32, |acc, x| acc.wrapping_add(*x))
        )
    }
}
