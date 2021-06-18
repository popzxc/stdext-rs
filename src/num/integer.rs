// This interface copies `std` one, thus we must discard clippy complains.
#![allow(clippy::wrong_self_convention)]

use std::num::ParseIntError;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// Built-in integers interface exposed as a trait.
///
/// This trait is implemented for all the built-in integer types and copies their interface completely,
/// so it's possible to write generic code that accepts any integer number.
///
/// ## Caveats
///
/// - `<to/from/as>_<be/ne/le>_bytes` are not implemented, as the return type (array of generic const length that
///   depends on the trait itself) cannot in be expressed in stable rust.
///
/// - `is_power_of_two` / `next_power_of_two` / `checked_next_power_of_two` methods are not implemented,
///   as they exist for the unsigned numbers only.
pub trait Integer:
    Sized
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Shr
    + ShrAssign
    + Shl
    + ShlAssign
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Div
    + DivAssign
    + Mul
    + MulAssign
    + Copy
{
    const MIN: Self;
    const MAX: Self;
    const BITS: u32;

    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_ones(self) -> u32;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_zeros(self) -> u32;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_zeros(self) -> u32;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_zeros(self) -> u32;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_ones(self) -> u32;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_ones(self) -> u32;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_left(self, n: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_right(self, n: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn swap_bytes(self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn reverse_bits(self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_be(x: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_le(x: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_be(self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_le(self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_neg(self) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_shl(self, rhs: u32) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_shr(self, rhs: u32) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_pow(self, exp: u32) -> Option<Self>;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_mul(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_pow(self, exp: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_add(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_sub(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_mul(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div_euclid(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_rem(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_neg(self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_shl(self, rhs: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_shr(self, rhs: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_pow(self, exp: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_div(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_neg(self) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn pow(self, exp: u32) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn div_euclid(self, rhs: Self) -> Self;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! impl_integer {
    ($($int:ty),+) => {
        $(
        impl Integer for $int {
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;
            const BITS: u32 = Self::BITS;
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$int>::from_str_radix(src, radix)
            }
            fn count_ones(self) -> u32 {
                <$int>::count_ones(self)
            }
            fn count_zeros(self) -> u32 {
                <$int>::count_zeros(self)
            }
            fn leading_zeros(self) -> u32 {
                <$int>::leading_zeros(self)
            }
            fn trailing_zeros(self) -> u32 {
                <$int>::trailing_zeros(self)
            }
            fn leading_ones(self) -> u32 {
                <$int>::leading_ones(self)
            }
            fn trailing_ones(self) -> u32 {
                <$int>::trailing_ones(self)
            }
            fn rotate_left(self, n: u32) -> Self {
                <$int>::rotate_left(self, n)
            }
            fn rotate_right(self, n: u32) -> Self {
                <$int>::rotate_right(self, n)
            }
            fn swap_bytes(self) -> Self {
                <$int>::swap_bytes(self)
            }
            fn reverse_bits(self) -> Self {
                <$int>::reverse_bits(self)
            }
            fn from_be(x: Self) -> Self {
                <$int>::from_be(x)
            }
            fn from_le(x: Self) -> Self {
                <$int>::from_le(x)
            }
            fn to_be(self) -> Self {
                <$int>::to_be(self)
            }
            fn to_le(self) -> Self {
                <$int>::to_le(self)
            }
            fn checked_add(self, rhs: Self) -> Option<Self> {
                <$int>::checked_add(self, rhs)
            }
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                <$int>::checked_sub(self, rhs)
            }
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                <$int>::checked_mul(self, rhs)
            }
            fn checked_div(self, rhs: Self) -> Option<Self> {
                <$int>::checked_div(self, rhs)
            }
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                <$int>::checked_div_euclid(self, rhs)
            }
            fn checked_rem(self, rhs: Self) -> Option<Self> {
                <$int>::checked_rem(self, rhs)
            }
            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                <$int>::checked_rem_euclid(self, rhs)
            }
            fn checked_neg(self) -> Option<Self> {
                <$int>::checked_neg(self)
            }
            fn checked_shl(self, rhs: u32) -> Option<Self> {
                <$int>::checked_shl(self, rhs)
            }
            fn checked_shr(self, rhs: u32) -> Option<Self> {
                <$int>::checked_shr(self, rhs)
            }
            fn checked_pow(self, exp: u32) -> Option<Self> {
                <$int>::checked_pow(self, exp)
            }
            fn saturating_add(self, rhs: Self) -> Self {
                <$int>::saturating_add(self, rhs)
            }
            fn saturating_sub(self, rhs: Self) -> Self {
                <$int>::saturating_sub(self, rhs)
            }
            fn saturating_mul(self, rhs: Self) -> Self {
                <$int>::saturating_mul(self, rhs)
            }
            fn saturating_pow(self, exp: u32) -> Self {
                <$int>::saturating_pow(self, exp)
            }
            fn wrapping_add(self, rhs: Self) -> Self {
                <$int>::wrapping_add(self, rhs)
            }
            fn wrapping_sub(self, rhs: Self) -> Self {
                <$int>::wrapping_sub(self, rhs)
            }
            fn wrapping_mul(self, rhs: Self) -> Self {
                <$int>::wrapping_mul(self, rhs)
            }
            fn wrapping_div(self, rhs: Self) -> Self {
                <$int>::wrapping_div(self, rhs)
            }
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                <$int>::wrapping_div_euclid(self, rhs)
            }
            fn wrapping_rem(self, rhs: Self) -> Self {
                <$int>::wrapping_rem(self, rhs)
            }
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                <$int>::wrapping_rem_euclid(self, rhs)
            }
            fn wrapping_neg(self) -> Self {
                <$int>::wrapping_neg(self)
            }
            fn wrapping_shl(self, rhs: u32) -> Self {
                <$int>::wrapping_shl(self, rhs)
            }
            fn wrapping_shr(self, rhs: u32) -> Self {
                <$int>::wrapping_shr(self, rhs)
            }
            fn wrapping_pow(self, exp: u32) -> Self {
                <$int>::wrapping_pow(self, exp)
            }
            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_add(self, rhs)
            }
            fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_sub(self, rhs)
            }
            fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_mul(self, rhs)
            }
            fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_div(self, rhs)
            }
            fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_div_euclid(self, rhs)
            }
            fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_rem(self, rhs)
            }
            fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                <$int>::overflowing_rem_euclid(self, rhs)
            }
            fn overflowing_neg(self) -> (Self, bool) {
                <$int>::overflowing_neg(self)
            }
            fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                <$int>::overflowing_shr(self, rhs)
            }
            fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                <$int>::overflowing_pow(self, exp)
            }
            fn pow(self, exp: u32) -> Self {
                <$int>::pow(self, exp)
            }
            fn div_euclid(self, rhs: Self) -> Self {
                <$int>::div_euclid(self, rhs)
            }
            fn rem_euclid(self, rhs: Self) -> Self {
                <$int>::rem_euclid(self, rhs)
            }
        }
    )+
    };
}

impl_integer!(u8, u16, u32, u64, u128);
impl_integer!(i8, i16, i32, i64, i128);
impl_integer!(usize, isize);
