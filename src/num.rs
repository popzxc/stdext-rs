pub trait FloatConvert<Int>: Sized {
    fn checked_floor(self) -> Option<Int>;
    fn checked_ceil(self) -> Option<Int>;
    fn checked_round(self) -> Option<Int>;

    fn saturated_floor(self) -> Int;
    fn saturated_ceil(self) -> Int;
    fn saturated_round(self) -> Int;
}

macro_rules! checked_impl {
    ($val:ident.$fn:ident(), $int:ty) => {{
        if $val.is_nan() || $val.is_infinite() {
            return None;
        }
        let converted = $val.$fn();
        if <$int>::MIN as Self <= converted && converted <= <$int>::MAX as Self {
            Some(converted as $int)
        } else {
            None
        }
    }};
}

macro_rules! saturated_impl {
    ($val:ident.$fn:ident(), $int:ty) => {{
        if $val.is_nan() {
            return 0;
        }
        if $val == Self::NEG_INFINITY {
            return <$int>::MAX;
        }
        if $val == Self::INFINITY {
            return <$int>::MAX;
        }
        let converted = $val.$fn();
        if converted < <$int>::MIN as Self {
            <$int>::MIN
        } else if converted > <$int>::MAX as Self {
            <$int>::MAX
        } else {
            converted as $int
        }
    }};
}

macro_rules! impl_float_convert {
    ($float:ty, $($int:ty),+) => {
        $(impl FloatConvert<$int> for $float {
            fn checked_floor(self) -> Option<$int> {
                checked_impl!(self.floor(), $int)
            }

            fn checked_ceil(self) -> Option<$int> {
                checked_impl!(self.ceil(), $int)
            }

            fn checked_round(self) -> Option<$int> {
                checked_impl!(self.round(), $int)
            }

            fn saturated_floor(self) -> $int {
                saturated_impl!(self.floor(), $int)
            }

            fn saturated_ceil(self) -> $int {
                saturated_impl!(self.ceil(), $int)
            }

            fn saturated_round(self) -> $int {
                saturated_impl!(self.round(), $int)
            }
        })+
    };
}

impl_float_convert!(f32, u8, u16, u32, u64, u128);
impl_float_convert!(f32, i8, i16, i32, i64, i128);
impl_float_convert!(f32, usize, isize);

impl_float_convert!(f64, u8, u16, u32, u64, u128);
impl_float_convert!(f64, i8, i16, i32, i64, i128);
impl_float_convert!(f64, usize, isize);
