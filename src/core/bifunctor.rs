pub trait Bifunctor {
    type Inner1;
    type Inner2;
    type Outter<B1, C1>: Bifunctor;

    fn bimap<F1, F2, B, C>(self, f1: F1, f2: F2) -> Self::Outter<B, C>
    where
        F1: FnMut(Self::Inner1) -> B,
        F2: FnMut(Self::Inner2) -> C;
}

impl<T1, T2> Bifunctor for (T1, T2) {
    type Inner1 = T1;

    type Inner2 = T2;

    type Outter<B, C> = (B, C);

    fn bimap<F1, F2, B, C>(self, mut f1: F1, mut f2: F2) -> Self::Outter<B, C>
    where
        F1: FnMut(Self::Inner1) -> B,
        F2: FnMut(Self::Inner2) -> C,
    {
        (f1(self.0), f2(self.1))
    }
}

impl<T1, T2> Bifunctor for Result<T1, T2> {
    type Inner1 = T1;

    type Inner2 = T2;

    type Outter<B, C> = Result<B, C>;

    fn bimap<F1, F2, B, C>(self, f1: F1, f2: F2) -> Self::Outter<B, C>
    where
        F1: FnMut(Self::Inner1) -> B,
        F2: FnMut(Self::Inner2) -> C,
    {
        self.map(f1).map_err(f2)
    }
}
