//! Freestanding version of `std::default::Default::default()`.

/// Freestanding version of `std::default::Default::default()`.
///
/// Used to be available under `#![feature(default_free_fn)]`,
/// removed in [rust-lang/rust#113469](https://github.com/rust-lang/rust/pull/113469).
///
/// # Examples
///
/// ```
/// use stdext::prelude::*;
///
/// #[derive(Default, PartialEq, Eq, Debug)]
/// struct StructWithLongName {
///     a: u32,
///     b: u32,
/// }
///
/// let s = StructWithLongName {
///     a: 12,
///     ..default() // Normally you have to do
///                 // `Default::default()`,
///                 // `<_>::default()` or
///                 // `StructWithLongName::default()`
/// };
///
/// assert_eq!(s, StructWithLongName { a: 12, ..<_>::default() });
/// ```
pub fn default<T: Default>() -> T {
    T::default()
}
