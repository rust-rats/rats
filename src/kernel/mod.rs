pub mod monoid;
pub mod semigroup;

pub trait Kind {
    type Ty<T>;
}

pub mod default_kinds {
    use super::Kind;

    pub struct VecKind;
    impl Kind for VecKind {
        type Ty<T> = Vec<T>;
    }

    pub struct OptionKind;
    impl Kind for OptionKind {
        type Ty<T> = Option<T>;
    }
}

pub mod prelude {
    pub use super::default_kinds::*;
    pub use super::monoid::*;
    pub use super::semigroup::*;
}
