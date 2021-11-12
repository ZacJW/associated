#[cfg(feature = "derive")]
pub use associated_derive::*;

pub trait Associated {
    type AssociatedType;
    fn get_associated(&self) -> &'static Self::AssociatedType;
}

pub trait TryAssociated {
    type AssociatedType;
    fn try_get_associated(&self) -> Option<&'static Self::AssociatedType>;
}

