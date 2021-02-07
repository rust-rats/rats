pub trait SemigroupK {
    type Inner;
    type Outter<B>: SemigroupK;

    fn combine_k(self, other: Self::Outter<Self::Inner>) -> Self::Outter<Self::Inner>;
}

impl<T> SemigroupK for Option<T> {
    type Inner = T;
    type Outter<B> = Option<T>;

    fn combine_k(self, other: Self::Outter<Self::Inner>) -> Self::Outter<Self::Inner> {
        self.or(other)
    }
}

impl<T, E> SemigroupK for Result<T, E> {
    type Inner = T;
    type Outter<B> = Result<B, E>;

    fn combine_k(self, other: Self::Outter<Self::Inner>) -> Self::Outter<Self::Inner> {
        self.or(other)
    }
}

impl<T> SemigroupK for Vec<T> {
    type Inner = T;
    type Outter<B> = Vec<B>;

    fn combine_k(mut self, mut other: Self::Outter<Self::Inner>) -> Self::Outter<Self::Inner> {
        self.append(&mut other);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::SemigroupK;

    #[test]
    fn semigroup_k_option_is_first_success() {
        assert_eq!(Some(1).combine_k(None), Some(1));
        assert_eq!(Some(1).combine_k(Some(2)), Some(1));
        assert_eq!(Option::<i32>::None.combine_k(Some(2)), Some(2));
        assert_eq!(Option::<i32>::None.combine_k(Option::<i32>::None), None);
    }

    #[test]
    fn semigroup_k_result_is_first_success() {
        assert_eq!(Ok(1).combine_k(Err(())), Ok(1));
        assert_eq!(Ok(1).combine_k(Result::<i32, ()>::Ok(2)), Ok(1));
        assert_eq!(Err(()).combine_k(Ok(2)), Ok(2));
        assert_eq!(Result::<i32, ()>::Err(()).combine_k(Err(())), Err(()));
    }

    #[test]
    fn semigroup_k_vec_is_append() {
        assert_eq!(vec![1, 2].combine_k(vec![3, 4]), vec![1, 2, 3, 4]);
    }
}
