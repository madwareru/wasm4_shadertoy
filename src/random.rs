use std::ops::{RangeInclusive};

pub struct RndGen {
    seed: u16
}
impl Default for RndGen {
    fn default() -> Self {
        Self { seed: 0b1010_1100_1110_0001 }
    }
}
impl RndGen {
    pub fn new() -> Self{
        Self { seed: 0b1010_1100_1110_0001 }
    }
    pub fn gen_range_i(&mut self, bounds: RangeInclusive<i16>) -> i16 {
        let bit = (self.seed & 0x2d).count_ones() as u16;
        self.seed = (self.seed >> 1) | (bit << 15);
        let (lo, hi) = (*bounds.start(), *bounds.end());
        let (lo, hi) = if hi < lo { (hi, lo) } else { (lo, hi) };
        let rng = (hi as i32 - lo as i32) as u16;
        if rng == 0 {
            return lo;
        }
        (lo as i32 + (self.seed % rng) as i32) as i16
    }

    pub fn gen_range(&mut self, bounds: RangeInclusive<u16>) -> u16 {
        let bit = (self.seed & 0x2d).count_ones() as u16;
        self.seed = (self.seed >> 1) | (bit << 15);
        let (lo, hi) = (*bounds.start(), *bounds.end());
        let (lo, hi) = if hi < lo { (hi, lo) } else { (lo, hi) };
        let rng = hi - lo;
        if rng == 0 {
            return lo;
        }

        lo + (self.seed % rng)
    }
}