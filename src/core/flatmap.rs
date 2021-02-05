use super::apply::Apply;
use crate::kernel::prelude::Id;

pub trait FlatMap: Apply {
    fn flat_map<B, F>(self, f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> Self::Outter<B>;
}

impl<A> FlatMap for Option<A> {
    fn flat_map<B, F>(self, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> Self::Outter<B>,
    {
        f(self?)
    }
}

impl<A, E> FlatMap for Result<A, E> {
    fn flat_map<B, F>(self, mut f: F) -> Self::Outter<B>
        where
            F: FnMut(Self::Inner) -> Self::Outter<B>,
    {
        f(self?)
    }
}

impl<A: Clone> FlatMap for Vec<A> {
    fn flat_map<B, F>(self, f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> Self::Outter<B>,
    {
        self.into_iter()
            .flat_map(f)
            .collect()
    }
}

impl<A> FlatMap for Id<A> {
    fn flat_map<B, F>(self, mut f: F) -> Self::Outter<B>
    where
        F: FnMut(Self::Inner) -> Self::Outter<B>,
    {
        f(self.into_value())
    }
}

#[cfg(test)]
mod tests {
    use super::FlatMap;

    #[test]
    fn option_some_some() {
        let option = Some(3);
        let function = |x: u64| -> Option<f32> { Some((x * x) as f32) };
        assert_eq!(option.flat_map(function), Some(9.0));
    }

    #[test]
    fn option_some_none() {
        let option = Some(3);
        let function = |_x: u64| -> Option<f32> { None };
        assert_eq!(option.flat_map(function), None);
    }

    #[test]
    fn option_none_some() {
        let option = None;
        let function = |x: u64| -> Option<f32> { Some((x * x) as f32) };
        assert_eq!(option.flat_map(function), None);
    }

    #[test]
    fn option_none_none() {
        let option = None;
        let function = |_x: u64| -> Option<f32> { None };
        assert_eq!(option.flat_map(function), None);
    }

    #[test]
    fn result_ok_ok() {
        let result: Result<u64, ()> = Ok(3);
        let function = |x: u64| -> Result<f32, ()> { Ok((x * x) as f32) };
        assert_eq!(result.flat_map(function), Ok(9.0));
    }

    #[test]
    fn result_ok_err() {
        let result: Result<u64, ()> = Ok(3);
        let function = |_x: u64| -> Result<f32, ()> { Err(()) };
        assert_eq!(result.flat_map(function), Err(()));
    }

    #[test]
    fn result_err_ok() {
        let result = Err(());
        let function = |x: u64| -> Result<f32, ()> { Ok((x * x) as f32) };
        assert_eq!(result.flat_map(function), Err(()));
    }

    #[test]
    fn result_err_err() {
        let result = Err(());
        let function = |_x: u64| -> Result<f32, ()> { Err(()) };
        assert_eq!(result.flat_map(function), Err(()));
    }

    #[test]
    fn id() {
        use crate::kernel::prelude::Id;

        let id = Id(3);
        let function = |x: u64| -> Id<f32> { Id((x * x) as f32) };
        assert_eq!(id.flat_map(function), Id(9.0));
    }

    #[test]
    fn vec() {
        let values = vec![1, 2, 3, 4];
        let function = |x: u64| -> Vec<f32> {
            vec![0.0, (x + 1) as f32, (x * x) as f32]
        };
        #[rustfmt::skip]
        let expected = vec![
          //0.0, x+1, x*x
            0.0, 2.0, 1.0,
            0.0, 3.0, 4.0,
            0.0, 4.0, 9.0,
            0.0, 5.0, 16.0,
        ];
        assert_eq!(values.flat_map(function), expected);
    }
}
