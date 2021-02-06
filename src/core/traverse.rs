use super::prelude::{Applicative, Foldable, Functor};

pub trait Traverse: Functor<Outter = Self> + Foldable<Outter = Self> {
    fn traverse<G: Applicative, B, F>(
        self,
        f: F,
    ) -> <G as Functor>::Outter<<Self as Functor>::Outter<B>>
    where
        F: FnMut(<Self as Functor>::Inner) -> <G as Functor>::Outter<B>;
}

// impl<T> Traverse for Vec<T> {
// fn traverse<G: Applicative, B, F>(
// self,
// f: F,
// ) -> <G as Functor>::Outter<<Self as Functor>::Outter<B>>
// where
// F: FnMut(<Self as Functor>::Inner) -> <G as Functor>::Outter<B>,
// {
// if self.is_empty() {
// G::pure(Vec::<B>::new());
// } else {
// self.fold_right(G::pure(Vec::<B>::new()), |acc, b| {
// //acc.fmap(|mut vec| G::pure(vec.push(b)))
// todo!()
// })
// }
// }
// }
