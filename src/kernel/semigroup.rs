pub trait Semigroup {
    fn combine(self, b: Self) -> Self;
}

impl<A> Semigroup for Vec<A> {
    fn combine(mut self, mut b: Self) -> Self {
        self.append(&mut b);
        self
    }
}

impl<A: Semigroup> Semigroup for Option<A> {
    fn combine(self, b: Self) -> Self {
        match (self, b) {
            (None, None) => None,
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => Some(a.combine(b)),
        }
    }
}

impl<A: Semigroup, E> Semigroup for Result<A, E> {
    fn combine(self, b: Self) -> Self {
        match (self, b) {
            (a @ Err(_), _) => a,
            (_, b @ Err(_)) => b,
            (Ok(a), Ok(b)) => Ok(a.combine(b)),
        }
    }
}

macro_rules! impl_semigroup {
    ($t:ident) => {
        impl Semigroup for $t {
            fn combine(self, b: Self) -> Self {
                self + b
            }
        }
    };
}

impl_semigroup!(i8);
impl_semigroup!(i16);
impl_semigroup!(i32);
impl_semigroup!(i64);
impl_semigroup!(u8);
impl_semigroup!(u16);
impl_semigroup!(u32);
impl_semigroup!(u64);
impl_semigroup!(usize);
impl_semigroup!(f32);
impl_semigroup!(f64);
