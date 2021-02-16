use super::prelude::ApplicativeInstance;

#[derive(Copy, Clone, Default)]
pub struct FlatMap;

impl FlatMap {
    #[inline]
    pub fn flat_map<Kind: FlatMapTy, A, B>(
        _: Kind,
        fa: impl FlatMapInstance<A, Kind = Kind>,
        f: impl Fn(&A) -> Kind::Cons<B>,
    ) -> Kind::Cons<B> {
        fa.flat_map(f)
    }
}

pub trait FlatMapTy {
    type Cons<T>: ApplicativeInstance<Kind = Self> + FlatMapInstance<T, Kind = Self>;
}

pub trait FlatMapInstance<T> {
    type Kind: FlatMapTy;

    fn flat_map<B>(
        self,
        f: impl Fn(&T) -> <Self::Kind as FlatMapTy>::Cons<B>,
    ) -> <Self::Kind as FlatMapTy>::Cons<B>;
}

pub mod std_instances {
    use crate::core::prelude::{OptionKind, ResultKindOk, VecKind};

    use super::*;

    impl FlatMapTy for OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<A> FlatMapInstance<A> for Option<A> {
        type Kind = OptionKind;

        fn flat_map<B>(self, f: impl FnOnce(&A) -> Option<B>) -> Option<B> {
            f(&self?)
        }
    }

    impl<E> FlatMapTy for ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }

    impl<A, E> FlatMapInstance<A> for Result<A, E> {
        type Kind = ResultKindOk<E>;

        fn flat_map<B>(self, f: impl FnOnce(&A) -> Result<B, E>) -> Result<B, E> {
            f(&self?)
        }
    }

    impl FlatMapTy for VecKind {
        type Cons<T> = Vec<T>;
    }

    impl<A> FlatMapInstance<A> for Vec<A> {
        type Kind = VecKind;

        fn flat_map<B>(self, f: impl FnMut(&A) -> Vec<B>) -> Vec<B> {
            self.iter().flat_map(f).collect()
        }
    }
}

// #[cfg(test)]
// mod tests {
// use super::FlatMap;

// #[test]
// fn option_some_some() {
// let option = Some(3);
// let function = |x: u64| -> Option<f32> { Some((x * x) as f32) };
// assert_eq!(option.flat_map(function), Some(9.0));
// }

// #[test]
// fn option_some_none() {
// let option = Some(3);
// let function = |_x: u64| -> Option<f32> { None };
// assert_eq!(option.flat_map(function), None);
// }

// #[test]
// fn option_none_some() {
// let option = None;
// let function = |x: u64| -> Option<f32> { Some((x * x) as f32) };
// assert_eq!(option.flat_map(function), None);
// }

// #[test]
// fn option_none_none() {
// let option = None;
// let function = |_x: u64| -> Option<f32> { None };
// assert_eq!(option.flat_map(function), None);
// }

// #[test]
// fn result_ok_ok() {
// let result: Result<u64, ()> = Ok(3);
// let function = |x: u64| -> Result<f32, ()> { Ok((x * x) as f32) };
// assert_eq!(result.flat_map(function), Ok(9.0));
// }

// #[test]
// fn result_ok_err() {
// let result: Result<u64, ()> = Ok(3);
// let function = |_x: u64| -> Result<f32, ()> { Err(()) };
// assert_eq!(result.flat_map(function), Err(()));
// }

// #[test]
// fn result_err_ok() {
// let result = Err(());
// let function = |x: u64| -> Result<f32, ()> { Ok((x * x) as f32) };
// assert_eq!(result.flat_map(function), Err(()));
// }

// #[test]
// fn result_err_err() {
// let result = Err(());
// let function = |_x: u64| -> Result<f32, ()> { Err(()) };
// assert_eq!(result.flat_map(function), Err(()));
// }

// #[test]
// fn vec() {
// let values = vec![1, 2, 3, 4];
// let function = |x: u64| -> Vec<f32> { vec![0.0, (x + 1) as f32, (x * x) as f32] };
// #[rustfmt::skip]
// let expected = vec![
// //0.0, x+1, x*x
// 0.0, 2.0, 1.0,
// 0.0, 3.0, 4.0,
// 0.0, 4.0, 9.0,
// 0.0, 5.0, 16.0,
// ];
// assert_eq!(values.flat_map(function), expected);
// }
// }
