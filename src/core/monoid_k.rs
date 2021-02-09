use super::prelude::SemigroupK;

pub trait MonoidK: SemigroupK {
    fn empty() -> <Self as SemigroupK>::Outter<<Self as SemigroupK>::Inner>;
}

impl<T> MonoidK for Option<T> {
    fn empty() -> <Self as SemigroupK>::Outter<<Self as SemigroupK>::Inner> {
        None
    }
}

impl<T> MonoidK for Vec<T> {
    fn empty() -> <Self as SemigroupK>::Outter<<Self as SemigroupK>::Inner> {
        vec![]
    }
}
