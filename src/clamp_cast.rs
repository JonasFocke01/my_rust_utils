#[must_use]
pub fn map_range(value: f64, from_range: (f64, f64), to_range: (f64, f64)) -> f64 {
    to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub trait ClampCast<T> {
    fn clamp_cast(self) -> T;
}

impl ClampCast<isize> for usize {
    #[allow(clippy::cast_possible_wrap)]
    fn clamp_cast(self) -> isize {
        self.clamp(usize::MIN, isize::MAX as usize) as isize
    }
}

impl ClampCast<u16> for usize {
    #[allow(clippy::cast_possible_truncation)]
    fn clamp_cast(self) -> u16 {
        self.clamp(usize::MIN, u16::MAX as usize) as u16
    }
}

impl ClampCast<u8> for u16 {
    fn clamp_cast(self) -> u8 {
        self.clamp(u16::MIN, u8::MAX.into()) as u8
    }
}
impl ClampCast<usize> for isize {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn clamp_cast(self) -> usize {
        self.clamp(usize::MIN as isize, isize::MAX) as usize
    }
}

impl ClampCast<i8> for isize {
    #[allow(clippy::cast_possible_truncation)]
    fn clamp_cast(self) -> i8 {
        self.clamp(i8::MIN as isize, i8::MAX as isize) as i8
    }
}

impl ClampCast<usize> for u64 {
    #[allow(clippy::cast_possible_truncation)]
    fn clamp_cast(self) -> usize {
        self.clamp(u64::MIN, usize::MAX as u64) as usize
    }
}
impl ClampCast<usize> for f64 {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    fn clamp_cast(self) -> usize {
        self.clamp(usize::MIN as f64, f64::MAX) as usize
    }
}

impl ClampCast<u8> for f64 {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_lossless
    )]
    fn clamp_cast(self) -> u8 {
        self.clamp(u8::MIN as f64, u8::MAX as f64) as u8
    }
}

impl ClampCast<i8> for f64 {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_lossless
    )]
    fn clamp_cast(self) -> i8 {
        self.clamp(i8::MIN as f64, i8::MAX as f64) as i8
    }
}

impl ClampCast<f64> for u64 {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss
    )]
    fn clamp_cast(self) -> f64 {
        self.clamp(u64::MIN, f64::MAX as u64) as f64
    }
}

impl ClampCast<f64> for usize {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    fn clamp_cast(self) -> f64 {
        self.clamp(usize::MIN, f64::MAX as usize) as f64
    }
}

impl ClampCast<f64> for isize {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    fn clamp_cast(self) -> f64 {
        self.clamp(f64::MIN as isize, f64::MAX as isize) as f64
    }
}

impl ClampCast<isize> for f64 {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    fn clamp_cast(self) -> isize {
        self.clamp(isize::MIN as f64, isize::MAX as f64) as isize
    }
}

impl ClampCast<u8> for f32 {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_lossless
    )]
    fn clamp_cast(self) -> u8 {
        self.clamp(u8::MIN as f32, u8::MAX as f32) as u8
    }
}

impl ClampCast<u8> for usize {
    #[allow(clippy::cast_possible_truncation)]
    fn clamp_cast(self) -> u8 {
        self.clamp(usize::MIN, u8::MAX as usize) as u8
    }
}

impl ClampCast<i8> for usize {
    #[allow(clippy::cast_possible_truncation)]
    fn clamp_cast(self) -> i8 {
        self.clamp(usize::MIN, i8::MAX as usize) as i8
    }
}

impl ClampCast<u8> for i64 {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_lossless
    )]
    fn clamp_cast(self) -> u8 {
        self.clamp(u8::MIN as i64, u8::MAX as i64) as u8
    }
}

impl ClampCast<u8> for i8 {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn clamp_cast(self) -> u8 {
        self.clamp(u8::MIN as i8, i8::MAX) as u8
    }
}

impl ClampCast<i8> for u8 {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn clamp_cast(self) -> i8 {
        self.clamp(u8::MIN, i8::MAX as u8) as i8
    }
}
