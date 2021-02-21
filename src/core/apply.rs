use super::prelude::FunctorInstance;

#[inline]
pub fn ap<Kind, F, A, B, K1>(
    _: Kind,
    fa: K1,
    ff: <<K1 as ApplyInstance<A>>::Kind as ApplyTy>::Cons<F>,
) -> Kind::Cons<B>
where
    Kind: ApplyTy,
    F: Fn(&A) -> B,
    K1: ApplyInstance<A, Kind = Kind> + FunctorInstance<A, Kind = Kind>,
{
    fa.apply(ff)
}

pub trait ApplyTy {
    type Cons<T>: ApplyInstance<T, Kind = Self> + FunctorInstance<T, Kind = Self>;
}

pub trait ApplyInstance<T> {
    #[rustfmt::skip]
    type Kind: ApplyTy<Cons<T> = Self>;

    fn apply<B, F>(self, f: <Self::Kind as ApplyTy>::Cons<F>) -> <Self::Kind as ApplyTy>::Cons<B>
    where
        F: Fn(&T) -> B;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl ApplyTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<A> ApplyInstance<A> for Option<A> {
        type Kind = OptionKind;

        fn apply<B, F>(self, f: Option<F>) -> Option<B>
        where
            F: FnOnce(&A) -> B,
        {
            match (self, f) {
                (Some(v), Some(f)) => Some(f(&v)),
                _ => None,
            }
        }
    }

    impl ApplyTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<A> ApplyInstance<A> for Vec<A> {
        type Kind = VecKind;

        fn apply<B, F>(self, mut f: Vec<F>) -> Vec<B>
        where
            F: FnMut(&A) -> B,
        {
            self.iter()
                .flat_map(|value| {
                    let mut applied_values = Vec::with_capacity(f.len());
                    for function in f.iter_mut() {
                        applied_values.push(function(value));
                    }
                    applied_values
                })
                .collect()
        }
    }

    impl<E> ApplyTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<A, E> ApplyInstance<A> for Result<A, E> {
        type Kind = ResultKindOk<E>;

        fn apply<B, F>(self, f: Result<F, E>) -> Result<B, E>
        where
            F: FnOnce(&A) -> B,
        {
            match (self, f) {
                (Ok(v), Ok(f)) => Ok(f(&v)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_none<T>(_: Option<T>) -> Option<T> {
        None
    }

    #[test]
    fn option_some_some() {
        let option = Some(3);
        let function = Some(|x: &u64| -> f32 { (x * x) as f32 });
        assert_eq!(option.apply(function), Some(9.0));
    }

    #[test]
    fn option_some_none() {
        let option = Some(3);
        let function = Some(|x: &u64| -> f32 { (x * x) as f32 });
        let function = to_none(function);
        assert_eq!(option.apply(function), None);
    }

    #[test]
    fn option_none_some() {
        let option = None;
        let function = Some(|x: &u64| -> f32 { (x * x) as f32 });
        assert_eq!(option.apply(function), None);
    }

    #[test]
    fn option_none_none() {
        let option = None;
        let function = Some(|x: &u64| -> f32 { (x * x) as f32 });
        let function = to_none(function);
        assert_eq!(option.apply(function), None);
    }

    fn to_err<T, E>(_: Result<T, E>, e: E) -> Result<T, E> {
        Err(e)
    }

    #[test]
    fn result_ok_ok() {
        let result: Result<u64, ()> = Ok(3);
        let function = Ok(|x: &u64| -> f32 { (x * x) as f32 });
        assert_eq!(result.apply(function), Ok(9.0));
    }

    #[test]
    fn result_ok_err() {
        let result: Result<u64, ()> = Ok(3);
        let function = Ok(|x: &u64| -> f32 { (x * x) as f32 });
        let function = to_err(function, ());
        assert_eq!(result.apply(function), Err(()));
    }

    #[test]
    fn result_err_ok() {
        let result = Err(());
        let function = Ok(|x: &u64| -> f32 { (x * x) as f32 });
        assert_eq!(result.apply(function), Err(()));
    }

    #[test]
    fn result_err_err() {
        let result = Err(());
        let function = Ok(|x: &u64| -> f32 { (x * x) as f32 });
        let function = to_err(function, ());
        assert_eq!(result.apply(function), Err(()));
    }

    #[test]
    fn vec() {
        let values = vec![1, 2, 3, 4];
        let functions: Vec<Box<dyn Fn(&u64) -> f32>> = vec![
            Box::new(|_x: &u64| -> f32 { 0.0 as f32 }),
            Box::new(|x: &u64| -> f32 { (x + 1) as f32 }),
            Box::new(|x: &u64| -> f32 { (x * x) as f32 }),
        ];
        #[rustfmt::skip]
        let expected = vec![
            //0.0, x+1, x*x
            0.0, 2.0, 1.0,
            0.0, 3.0, 4.0,
            0.0, 4.0, 9.0,
            0.0, 5.0, 16.0,
        ];
        let actual = values.apply(functions);
        assert_eq!(actual, expected);
    }
}
