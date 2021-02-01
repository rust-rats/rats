use super::functor::Functor;
use crate::kernel::prelude::Id;

pub trait Apply : Functor {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B;
}

impl<A> Apply for Option<A> {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> B
    {
        self.fmap(f?)
    }
}

impl<A, E> Apply for Result<A, E> {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
        where
            F: FnMut(Self::Inner) -> B
    {
        self.fmap(f?)
    }
}

impl<A : Clone> Apply for Vec<A> {
    fn apply<B, F>(self, mut f: Self::Outter<F>) -> Self::Outter<B>
        where
            F: FnMut(Self::Inner) -> B
    {
        self.into_iter().flat_map(|value| -> Self::Outter<B> {
            let mut applied_values = Vec::with_capacity(f.len());
            for function in f.iter_mut() {
                applied_values.push(function(value.clone()));
            };
            applied_values
        }).collect()
    }
}

impl<A> Apply for Id<A> {
    fn apply<B, F>(self, f: Self::Outter<F>) -> Self::Outter<B>
        where
            F: FnMut(Self::Inner) -> B
    {
        self.fmap(f.into_value())
    }
}

#[cfg(test)]
mod tests {
    use super::Apply;

    fn to_none<T>(_ : Option<T>) -> Option<T> {
        None
    }

    #[test]
    fn option_some_some() {
        let option = Some(3);
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(option.apply(function), Some(9.0));
    }

    #[test]
    fn option_some_none() {
        let option = Some(3);
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        let function = to_none(function);
        assert_eq!(option.apply(function), None);
    }

    #[test]
    fn option_none_some() {
        let option = None;
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(option.apply(function), None);
    }

    #[test]
    fn option_none_none() {
        let option = None;
        let function = Some(|x : u64| -> f32 {
            (x*x) as f32
        });
        let function = to_none(function);
        assert_eq!(option.apply(function), None);
    }

    fn to_err<T, E>(_ : Result<T, E>, e: E) -> Result<T, E> {
        Err(e)
    }

    #[test]
    fn result_ok_ok() {
        let result : Result<u64, ()> = Ok(3);
        let function = Ok(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(result.apply(function), Ok(9.0));
    }

    #[test]
    fn result_ok_err() {
        let result : Result<u64, ()> = Ok(3);
        let function = Ok(|x : u64| -> f32 {
            (x*x) as f32
        });
        let function = to_err(function, ());
        assert_eq!(result.apply(function), Err(()));
    }

    #[test]
    fn result_err_ok() {
        let result = Err(());
        let function = Ok(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(result.apply(function), Err(()));
    }

    #[test]
    fn result_err_err() {
        let result = Err(());
        let function = Ok(|x : u64| -> f32 {
            (x*x) as f32
        });
        let function = to_err(function, ());
        assert_eq!(result.apply(function), Err(()));
    }

    #[test]
    fn id() {
        use crate::kernel::prelude::Id;

        let id = Id(3);
        let function = Id(|x : u64| -> f32 {
            (x*x) as f32
        });
        assert_eq!(id.apply(function), Id(9.0));
    }

    #[test]
    fn vec() {
        let values = vec![1, 2, 3, 4];
        let functions = vec![
            |x : u64| -> f32 { (x*x) as f32 }
        ];
        let expected = vec![
            1.0, 4.0, 9.0, 16.0,
        ];
        assert_eq!(values.apply(functions), expected);
    }
}
