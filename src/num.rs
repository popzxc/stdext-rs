pub trait FloatConvert<To>: Sized {
    fn checked_floor(self) -> Option<To>;
    fn checked_ceil(self) -> Option<To>;
    fn checked_round(self) -> Option<To>;

    fn saturated_floor(self) -> To;
    fn saturated_ceil(self) -> To;
    fn saturated_round(self) -> To;
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

impl FloatConvert<u8> for f32 {
    fn checked_floor(self) -> Option<u8> {
        checked_impl!(self.floor(), u8)
    }

    fn checked_ceil(self) -> Option<u8> {
        checked_impl!(self.ceil(), u8)
    }

    fn checked_round(self) -> Option<u8> {
        checked_impl!(self.round(), u8)
    }

    fn saturated_floor(self) -> u8 {
        saturated_impl!(self.floor(), u8)
    }

    fn saturated_ceil(self) -> u8 {
        saturated_impl!(self.ceil(), u8)
    }

    fn saturated_round(self) -> u8 {
        saturated_impl!(self.round(), u8)
    }
}
