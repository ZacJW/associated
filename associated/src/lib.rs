//! A trait to associate enum variants with constants. See [associated-derive](https://docs.rs/associated-derive) for deriving this trait automatically.
//!
//! Derive support is enabled with the `"derive"` feature.

#[cfg(feature = "derive")]
pub use associated_derive::*;

/// See [`associated-derive`] for deriving this trait automatically.
/// 
/// [`associated-derive`]: https://docs.rs/associated-derive
pub trait Associated {
    /// The type of the constants associated with this enum.
    /// 
    /// If derived with associated-derive, this will be whatever `Type` is assigned to in `#[associated]`
    type AssociatedType;
    /// Returns a static lifetime reference to the constant associated with this variant.
    /// 
    /// If derived with associated-derive, this will be the argument to `#[assoc]` or `#[assoc_const]`
    fn get_associated(&self) -> &'static Self::AssociatedType;
}

/// WIP: Cannot currently be derived.
pub trait TryAssociated {
    type AssociatedType;
    fn try_get_associated(&self) -> Option<&'static Self::AssociatedType>;
}
