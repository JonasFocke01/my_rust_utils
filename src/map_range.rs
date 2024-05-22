use super::clamp_cast::ClampCast;

// TODO: Consider: There is currently no need to this traits functions to return a result, but
// there might be in the future

#[allow(clippy::module_name_repetitions)]
pub trait MapRange<T: Ord>: Sized + Ord {
    #[must_use]
    /// Maps the input range to the output range
    fn map_range_from_to(self, from: (Self, Self), to: (T, T)) -> T;
    #[must_use]
    /// Maps the given range to the output type
    /// The output range is infered as (T::MIN, T::MAX)
    fn map_range_from(self, from: (Self, Self)) -> T;
    #[must_use]
    /// Maps the given type to the output range
    /// The input range is infered as (Self::MIN, Self::MAX)
    fn map_range_to(self, to: (T, T)) -> T;
    #[must_use]
    /// Maps the input type to the output type
    /// The input range is infered as (Self::MIN, Self::MAX)
    /// The output range is infered as (T::MIN, T::MAX)
    fn map_range(self) -> T;
    /// This is a helper functon, other trait members are using
    fn check_aggainst_boundaries(self, from: (Self, Self), to: (T, T)) -> bool {
        from.0 <= self && self <= from.1 && to.0 <= to.1
    }
}

impl MapRange<usize> for u8 {
    fn map_range_from_to(self, from: (u8, u8), to: (usize, usize)) -> usize {
        if self.check_aggainst_boundaries(from, to) {
            let value = f64::from(self);
            let from = (f64::from(from.0), f64::from(from.1));
            let to: (f64, f64) = (to.0.clamp_cast(), to.1.clamp_cast());
            (to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)).clamp_cast()
        } else {
            self.into()
        }
    }
    fn map_range_from(self, from: (u8, u8)) -> usize {
        if self.check_aggainst_boundaries(from, (usize::MIN, usize::MAX)) {
            let value = f64::from(self);
            let from = (f64::from(from.0), f64::from(from.1));
            let to: (f64, f64) = (usize::MIN.clamp_cast(), usize::MAX.clamp_cast());
            (to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)).clamp_cast()
        } else {
            self.into()
        }
    }
    fn map_range_to(self, to: (usize, usize)) -> usize {
        if self.check_aggainst_boundaries((u8::MIN, u8::MAX), to) {
            let value = f64::from(self);
            let from = (f64::from(u8::MIN), f64::from(u8::MAX));
            let to: (f64, f64) = (to.0.clamp_cast(), to.1.clamp_cast());
            (to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)).clamp_cast()
        } else {
            self.into()
        }
    }
    fn map_range(self) -> usize {
        if self.check_aggainst_boundaries((u8::MIN, u8::MAX), (usize::MIN, usize::MAX)) {
            let value = f64::from(self);
            let from = (f64::from(u8::MIN), f64::from(u8::MAX));
            let to: (f64, f64) = (usize::MIN.clamp_cast(), usize::MAX.clamp_cast());
            (to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)).clamp_cast()
        } else {
            self.into()
        }
    }
}

impl MapRange<u8> for u8 {
    fn map_range_from_to(self, from: (u8, u8), to: (u8, u8)) -> u8 {
        if self.check_aggainst_boundaries(from, to) {
            to.0 + (self - from.0) * (to.1 - to.0) / (from.1 - from.0)
        } else {
            self
        }
    }
    fn map_range_from(self, from: (u8, u8)) -> u8 {
        if self.check_aggainst_boundaries(from, (u8::MIN, u8::MAX)) {
            u8::MIN + (self - from.0) * (u8::MAX - u8::MIN) / (from.1 - from.0)
        } else {
            self
        }
    }
    fn map_range_to(self, to: (u8, u8)) -> u8 {
        if self.check_aggainst_boundaries((u8::MIN, u8::MAX), to) {
            to.0 + (self - u8::MIN) * (to.1 - to.0) / (u8::MAX - u8::MIN)
        } else {
            self
        }
    }
    fn map_range(self) -> u8 {
        if self.check_aggainst_boundaries((u8::MIN, u8::MAX), (u8::MIN, u8::MAX)) {
            u8::MIN + (self - u8::MIN) * (u8::MAX - u8::MIN) / (u8::MAX - u8::MIN)
        } else {
            self
        }
    }
}

impl MapRange<u8> for usize {
    fn map_range_from_to(self, from: (usize, usize), to: (u8, u8)) -> u8 {
        if self.check_aggainst_boundaries(from, to) {
            (usize::from(to.0) + (self - from.0) * usize::from(to.1 - to.0) / (from.1 - from.0))
                .clamp_cast()
        } else {
            self.clamp_cast()
        }
    }
    fn map_range_from(self, from: (usize, usize)) -> u8 {
        if self.check_aggainst_boundaries(from, (u8::MIN, u8::MAX)) {
            (usize::from(u8::MIN)
                + (self - from.0) * usize::from(u8::MAX - u8::MIN) / (from.1 - from.0))
                .clamp_cast()
        } else {
            self.clamp_cast()
        }
    }
    fn map_range_to(self, to: (u8, u8)) -> u8 {
        if self.check_aggainst_boundaries((usize::MIN, usize::MAX), to) {
            (usize::from(to.0)
                + (self - usize::from(u8::MIN)) * usize::from(to.1 - to.0)
                    / (usize::MAX - usize::MIN))
                .clamp_cast()
        } else {
            self.clamp_cast()
        }
    }
    fn map_range(self) -> u8 {
        if self.check_aggainst_boundaries((usize::MIN, usize::MAX), (u8::MIN, u8::MAX)) {
            (usize::from(u8::MIN)
                + (self - usize::MIN) * usize::from(u8::MAX - u8::MIN) / (usize::MAX - usize::MIN))
                .clamp_cast()
        } else {
            self.clamp_cast()
        }
    }
}
