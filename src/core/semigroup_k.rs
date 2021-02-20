pub mod semigroupk {
    use super::*;

    pub fn combiane_k<Kind: SemigroupKTy, T>(
        _: Kind,
        this: Kind::Cons<T>,
        other: Kind::Cons<T>,
    ) -> Kind::Cons<T> {
        this.combine_k(other)
    }
}

pub trait SemigroupKTy {
    type Cons<T>: SemigroupKInstance<T, Kind = Self>;
}

pub trait SemigroupKInstance<T> {
    type Kind: SemigroupKTy;

    fn combine_k(
        self,
        other: <Self::Kind as SemigroupKTy>::Cons<T>,
    ) -> <Self::Kind as SemigroupKTy>::Cons<T>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl SemigroupKTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<T> SemigroupKInstance<T> for Option<T> {
        type Kind = OptionKind;

        fn combine_k(self, other: Option<T>) -> Option<T> {
            self.or(other)
        }
    }

    impl<E> SemigroupKTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<T, E> SemigroupKInstance<T> for Result<T, E> {
        type Kind = ResultKindOk<E>;

        fn combine_k(self, other: Result<T, E>) -> Result<T, E> {
            self.or(other)
        }
    }

    impl SemigroupKTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<T> SemigroupKInstance<T> for Vec<T> {
        type Kind = VecKind;

        fn combine_k(mut self, mut other: Vec<T>) -> Vec<T> {
            self.append(&mut other);
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
