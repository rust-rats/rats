#![deny(missing_docs)]
//! Defines the Applicative and ApplicativeError traits
//! and implements them for various types.

use super::apply::Apply;
use crate::kernel::helpers::Id;

/// Takes a value and lifts it into a context through the pure function.
///
/// A Type T that implements this trait must implement Apply and satisfy the following:
/// 1. Applicative Identity: `identity(A)->A` must be lifted to `identity(T<A>)->T<A>`, i.e.,
/// for every value `t: T<A>`:
/// ```rust
/// # use rats::core::apply::Apply;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_identity<A>(t: T<A>)
/// # where
/// #     A : Copy + std::fmt::Debug + std::cmp::PartialEq,
/// # {
/// assert_eq!(t.apply(T::pure(std::convert::identity)), t);
/// # }
/// ```
/// 2. Applicative Homomorphism: Lifting a value, then applying a lifted function must be the same
/// as applying the function first, then lifting the result, i.e.,
/// for every value `a: A` and for every function `f: A -> B`:
/// ```rust
/// # use rats::core::apply::Apply;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_homomorphism<A, B, F>(a: A, mut f: F)
/// # where
/// #     A : Copy,
/// #     B : std::fmt::Debug + std::cmp::PartialEq,
/// #     F : Copy + FnMut(A)->B,
/// # {
/// assert_eq!(T::pure(a).apply(T::pure(f)), T::pure(f(a)));
/// # }
/// ```
/// 3. Applicative Interchange: Lifting a value `a` and applying a lifted function must be the same as
/// calling apply on the lifted function an applying a lifted closure that takes a function as an
/// argument and returns that functions applied to `a`, i.e., for every value `a: A` and for every
/// lifted function `f: T<FnMut(A)->B>`:
/// ```rust
/// # use rats::core::apply::Apply;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_interchange<A, B, F>(a: A, mut tf: T<F>)
/// # where
/// #     A : Copy,
/// #     B : std::fmt::Debug + std::cmp::PartialEq,
/// #     F : Copy + FnMut(A)->B,
/// # {
/// assert_eq!(T::pure(a).apply(tf), tf.apply(T::pure(|mut fun: F| fun(a))));
/// # }
/// ```
/// 4. Applicative Map: Calling map must be equivalent to calling pure then calling apply, i.e.,
/// for every lifted value `a: T<A>` and for every function `f: FnMut(A)->B`:
/// ```rust
/// # use rats::core::functor::Functor;
/// # use rats::core::apply::Apply;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_map<A, B, F>(t: T<A>, mut f: F)
/// # where
/// #     A : Copy,
/// #     B : std::fmt::Debug + std::cmp::PartialEq,
/// #     F : Copy + FnMut(A)->B,
/// # {
/// assert_eq!(t.fmap(f), t.apply(T::pure(f)));
/// # }
/// ```
/// 5. Applicative Composition: This is a consquence of Apply Composition and Applicative Map,
/// and hence not strictly necessary. Applying two lifted functions consecutevely to a value `t: T` must
/// be the same as composing those lifted functions, then applying the result to `t`, i.e.,
/// for every value `t: T` and for every pair of functions `fab: FnMut(A)->B`, `fbc: FnMut(B)->C`:
/// ```rust
/// # use rats::core::apply::Apply;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_composition<A, B, C, FAB, FBC, FAC>(t: T<A>, mut fab: T<FAB>, mut fbc: T<FBC>)
/// # where
/// #     A : Copy,
/// #     C : std::fmt::Debug + std::cmp::PartialEq,
/// #     FAB : Copy + FnMut(A)->B,
/// #     FBC : Copy + FnMut(B)->C,
/// #     FBC : Copy + FnMut(A)->C,
/// # {
/// let compose = |mut fun_bc: FBC| {
///     move |mut fun_ab: FAB| {
///         move |a| fun_bc(fun_ab(a))
///     }
/// };
/// assert_eq!(t.apply(fab).apply(fbc), t.apply(fab.apply(fbc.apply(T::pure(compose)))));
/// # }
/// ```
/// 6. Applicative Product Consistency:
/// ```rust
/// # use rats::core::functor::Functor;
/// # use rats::core::apply::Apply;
/// # type T<A> = Option<A>;
/// # fn applicative_product_consistency<A, B, F>(t: T<A>, f: T<F>)
/// # where
/// #     A : Copy,
/// #     B : std::fmt::Debug + std::cmp::PartialEq,
/// #     F : Copy + FnMut(A)->B,
/// # {
/// # //TODO: after T::product is implemented, substitute it for this closure
/// # let product = |x, y| Some((x?, y?));
/// let prod = product(f, t);
/// let ap = |(mut fun, val): (F, A)| fun(val);
/// assert_eq!(t.apply(f), prod.map(ap));
/// # }
/// ```
/// 7. Applicative Unit: Lifting `()` and mapping it to `a: A` is the same as lifting `a`:
/// ```rust
/// # use rats::core::functor::Functor;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_unit<A>(a: A)
/// # where
/// #     A : Copy + std::fmt::Debug + std::cmp::PartialEq,
/// # {
/// let unit = T::pure(());
/// assert_eq!(unit.fmap(|_| a), T::pure(a));
/// # }
/// ```
/// 7. Applicative Monoidal Identity: Products with a lifted `()` don't do anything.
/// ```rust
/// # use rats::core::functor::Functor;
/// # use rats::core::applicative::Applicative;
/// # type T<A> = Option<A>;
/// # fn applicative_monoidal_identities<A>(t: T<A>)
/// # where
/// #     A : Copy + std::fmt::Debug + std::cmp::PartialEq,
/// # {
/// let unit = T::pure(());
/// # //TODO: after T::product is implemented, substitute it for this closure
/// # let product = |_, t| t;
/// assert_eq!(product(unit, t), t);
/// # //TODO: after T::product is implemented, substitute it for this closure
/// # let product = |t, _| t;
/// assert_eq!(product(t, unit), t);
/// # }
/// ```
pub trait Applicative: Apply {
    /// Takes a value and lifts it into a context.
    /// ```rust
    /// # use rats::core::applicative::Applicative;
    /// # type T<A> = Option<A>;
    /// // Creates a T holding the value 42
    /// let t = T::pure(42);
    /// ```
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner>;
}

impl<A> Applicative for Option<A> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        Some(value)
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        Ok(value)
    }
}

impl<A: Clone> Applicative for Vec<A> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        vec![value]
    }
}

impl<A> Applicative for Id<A> {
    fn pure(value: Self::Inner) -> Self::Outter<Self::Inner> {
        Id(value)
    }
}

/// Allows error handling for types that implement Applicative.
///
/// A Type T that implements this trait must implement Applicative and satisfy the following:
/// 1. ApplicativeError Raise and Handle: `handle_error_with(f)` composed with `raise_error`
/// must be equivalent to applying `f`
/// ```rust
/// # use rats::core::applicative::Applicative;
/// # use rats::core::applicative::ApplicativeError;
/// # type T<A, E> = Result<A, E>;
/// # fn applicative_identity<A, E, F>(e: E, mut f: F)
/// # where
/// #     A : std::fmt::Debug + std::cmp::PartialEq,
/// #     E : Copy + std::fmt::Debug + std::cmp::PartialEq,
/// #     F : Copy + FnMut(E)->T<A, E>,
/// # {
/// assert_eq!(T::raise_error(e).handle_error_with(f), f(e));
/// # }
/// ```
/// TODO
//
//   def applicativeErrorHandle[A](e: E, f: E => A): IsEq[F[A]] =
//     F.handleError(F.raiseError[A](e))(f) <-> F.pure(f(e))
//
//   def handleErrorWithPure[A](a: A, f: E => F[A]): IsEq[F[A]] =
//     F.handleErrorWith(F.pure(a))(f) <-> F.pure(a)
//
//   def handleErrorPure[A](a: A, f: E => A): IsEq[F[A]] =
//     F.handleError(F.pure(a))(f) <-> F.pure(a)
//
//   def raiseErrorAttempt(e: E): IsEq[F[Either[E, Unit]]] =
//     F.attempt(F.raiseError[Unit](e)) <-> F.pure(Left(e))
//
//   def pureAttempt[A](a: A): IsEq[F[Either[E, A]]] =
//     F.attempt(F.pure(a)) <-> F.pure(Right(a))
//
//   def handleErrorWithConsistentWithRecoverWith[A](fa: F[A], f: E => F[A]): IsEq[F[A]] =
//     F.handleErrorWith(fa)(f) <-> F.recoverWith(fa) { case x => f(x) }
//
//   def handleErrorConsistentWithRecover[A](fa: F[A], f: E => A): IsEq[F[A]] =
//     F.handleError(fa)(f) <-> F.recover(fa) { case x => f(x) }
//
//   def recoverConsistentWithRecoverWith[A](fa: F[A], pf: PartialFunction[E, A]): IsEq[F[A]] =
//     F.recover(fa)(pf) <-> F.recoverWith(fa)(pf.andThen(F.pure(_)))
//
//   def attemptConsistentWithAttemptT[A](fa: F[A]): IsEq[EitherT[F, E, A]] =
//     EitherT(F.attempt(fa)) <-> F.attemptT(fa)
//
//   def attemptFromEitherConsistentWithPure[A](eab: Either[E, A]): IsEq[F[Either[E, A]]] =
//     F.attempt(F.fromEither(eab)) <-> F.pure(eab)
//
//   def onErrorPure[A](a: A, f: E => F[Unit]): IsEq[F[A]] =
//     F.onError(F.pure(a)) { case x => f(x) } <-> F.pure(a)
//
//   def onErrorRaise[A](fa: F[A], e: E, fb: F[Unit]): IsEq[F[A]] =
//     F.onError(F.raiseError[A](e)) { case err => fb } <-> F.map2(fb, F.raiseError[A](e))((_, b) => b)
//
//   def adaptErrorPure[A](a: A, f: E => E): IsEq[F[A]] =
//     F.adaptError(F.pure(a)) { case x => f(x) } <-> F.pure(a)
//
//   def adaptErrorRaise[A](e: E, f: E => E): IsEq[F[A]] =
//     F.adaptError(F.raiseError[A](e)) { case x => f(x) } <-> F.raiseError(f(e))
//
//   def redeemDerivedFromAttemptMap[A, B](fa: F[A], fe: E => B, fs: A => B): IsEq[F[B]] =
//     F.redeem(fa)(fe, fs) <-> F.map(F.attempt(fa))(_.fold(fe, fs))
//
//   /*
//    * These laws, taken together with applicativeErrorHandle, show that errors dominate in
//    * ap, *and* show that handle has lexical semantics over ap. F.unit is used in both laws
//    * because we don't have another way of expressing "an F[_] which does *not* contain any
//    * errors". We could make these laws considerably stronger if such a thing were
//    * expressible. Specifically, what we're missing here is the ability to say that
//    * raiseError distributes over an *arbitrary* number of aps.
//    */
//
//   def raiseErrorDistributesOverApLeft[A](h: E => F[A], e: E) =
//     F.handleErrorWith(F.ap(F.raiseError[Unit => A](e))(F.unit))(h) <-> h(e)
//
//   def raiseErrorDistributesOverApRight[A](h: E => F[A], e: E) =
//     F.handleErrorWith(F.ap(F.pure((a: A) => a))(F.raiseError[A](e)))(h) <-> h(e)
pub trait ApplicativeError: Applicative {
    /// Type of the error for ApplicativeError.
    type ErrorT;

    /// If the inner value is an error, the function is applied, handling the error.
    /// Otherwise, nothing happens. Example:
    /// ```rust
    /// # use rats::core::applicative::ApplicativeError;
    /// let handler = |_| Ok(99);
    /// assert_eq!(Err("error").handle_error_with(handler), Ok(99));
    /// assert_eq!(Ok(12).handle_error_with(handler), Ok(12));
    /// ```
    fn handle_error_with<F>(self, f: F) -> Self::Outter<Self::Inner>
    where
        F: FnMut(Self::ErrorT) -> Self::Outter<Self::Inner>;

    /// Contructs an error with the given value.
    fn raise_error(error: Self::ErrorT) -> Self::Outter<Self::Inner>;
}

impl<A, E> ApplicativeError for Result<A, E> {
    type ErrorT = E;

    fn handle_error_with<F>(self, mut f: F) -> Self::Outter<Self::Inner>
    where
        F: FnMut(Self::ErrorT) -> Self::Outter<Self::Inner>,
    {
        match self {
            Err(e) => f(e),
            _ => self,
        }
    }

    fn raise_error(error: Self::ErrorT) -> Self::Outter<Self::Inner> {
        Err(error)
    }
}

impl<A> ApplicativeError for Option<A> {
    type ErrorT = ();

    fn handle_error_with<F>(self, mut f: F) -> Self::Outter<Self::Inner>
    where
        F: FnMut(Self::ErrorT) -> Self::Outter<Self::Inner>,
    {
        match self {
            None => f(()),
            _ => self,
        }
    }

    fn raise_error(_error: Self::ErrorT) -> Self::Outter<Self::Inner> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Applicative, ApplicativeError};

    #[test]
    fn option() {
        let value = 3;
        assert_eq!(Option::pure(value), Some(value));
    }

    #[test]
    fn result() {
        let value = 3;
        assert_eq!(Result::<i32, ()>::pure(value), Ok(value));
    }

    #[test]
    fn vec() {
        let value = 3;
        assert_eq!(Vec::pure(value), vec![value]);
    }

    #[test]
    fn id() {
        use crate::kernel::prelude::Id;

        let value = 3;
        assert_eq!(Id::pure(value), Id(3));
    }

    #[test]
    fn handle_error_with_for_result() {
        let value = Err(());
        let handler = |_err| Ok(3);
        assert_eq!(value.handle_error_with(handler), Ok(3));
    }

    #[test]
    fn raise_error_for_result() {
        let err = Result::<u64, String>::raise_error("ERROR!".to_string());
        assert_eq!(err, Err("ERROR!".to_string()));
    }

    #[test]
    fn handle_error_with_for_option() {
        let value = None;
        let handler = |_| Some(3);
        assert_eq!(value.handle_error_with(handler), Some(3));
    }

    #[test]
    fn raise_error_for_option() {
        let err = Option::<u64>::raise_error(());
        assert_eq!(err, None);
    }
}
