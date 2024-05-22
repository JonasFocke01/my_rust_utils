use super::clamp_cast::ClampCast;

#[allow(clippy::module_name_repetitions)]
pub trait MapRange: Sized {
    #[must_use]
    fn map_range(self, from: (Self, Self), to: (usize, usize)) -> usize;
    #[must_use]
    fn map_range_signed(self, from: (Self, Self), to: (isize, isize)) -> isize;
}

impl MapRange for u8 {
    fn map_range(self, from: (Self, Self), to: (usize, usize)) -> usize {
        if from.0 < self && self < from.1 {
            let value = f64::from(self);
            let from = (f64::from(from.0), f64::from(from.1));
            let to: (f64, f64) = (to.0.clamp_cast(), to.1.clamp_cast());
            (to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)).clamp_cast()
        } else {
            self.into()
        }
    }
    fn map_range_signed(self, from: (Self, Self), to: (isize, isize)) -> isize {
        if from.0 < self && self < from.1 {
            let value = f64::from(self);
            let from = (f64::from(from.0), f64::from(from.1));
            let to: (f64, f64) = (to.0.clamp_cast(), to.1.clamp_cast());
            (to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)).clamp_cast()
        } else {
            self.into()
        }
    }
}
