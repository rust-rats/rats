#[derive(Copy, Clone)]
pub struct Bifunctor;

impl Bifunctor {
    #[inline]
    pub fn bimap<Kind: BifunctorTy, T1, T2, B, C>(
        _: Kind,
        fab: impl BifunctorInstance<T1, T2, Kind = Kind>,
        f1: impl Fn(&T1) -> B,
        f2: impl Fn(&T2) -> C,
    ) -> Kind::Cons<B, C> {
        fab.bimap(f1, f2)
    }
}

pub trait BifunctorTy {
    type Cons<T1, T2>: BifunctorInstance<T1, T2>;
}
pub trait BifunctorInstance<T1, T2> {
    type Kind: BifunctorTy;

    fn bimap<B, C>(
        self,
        f1: impl Fn(&T1) -> B,
        f2: impl Fn(&T2) -> C,
    ) -> <Self::Kind as BifunctorTy>::Cons<B, C>;
}

pub mod std_instances {
    use crate::core::prelude::{ResultKind, Tuple2Kind};

    use super::*;

    impl BifunctorTy for Tuple2Kind {
        type Cons<T1, T2> = (T1, T2);
    }

    impl<T1, T2> BifunctorInstance<T1, T2> for (T1, T2) {
        type Kind = Tuple2Kind;

        fn bimap<B, C>(self, f1: impl FnOnce(&T1) -> B, f2: impl FnOnce(&T2) -> C) -> (B, C) {
            (f1(&self.0), f2(&self.1))
        }
    }

    impl BifunctorTy for ResultKind {
        type Cons<T1, T2> = Result<T1, T2>;
    }

    impl<T1, T2> BifunctorInstance<T1, T2> for Result<T1, T2> {
        type Kind = ResultKind;

        fn bimap<B, C>(self, f1: impl FnOnce(&T1) -> B, f2: impl FnOnce(&T2) -> C) -> Result<B, C> {
            match self {
                Ok(ok) => Ok(f1(&ok)),
                Err(err) => Err(f2(&err)),
            }
        }
    }
}
