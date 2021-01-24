use super::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}
