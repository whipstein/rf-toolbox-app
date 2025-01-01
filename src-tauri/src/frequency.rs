#![allow(unused)]
use crate::rf_utils::{scale, unscale};
use crate::unit::Unit;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Frequency {
    val: f64,
    unit: Unit,
}

impl Frequency {
    pub fn new(freq: f64, unit: Unit) -> Self {
        Frequency { val: freq, unit }
    }

    pub fn freq(&self) -> f64 {
        unscale(self.val, &self.unit)
    }

    pub fn freq_scaled(&self) -> f64 {
        self.val
    }

    pub fn set_freq(&mut self, val: f64) -> &Self {
        self.val = val;
        self
    }

    pub fn set_freq_scaled(&mut self, val: f64) -> &Self {
        self.val = unscale(val, &self.unit);
        self
    }

    pub fn set_freq_unit(&mut self, val: Unit) -> &Self {
        let tmp_val = scale(self.val, &self.unit);
        self.unit = val;
        self.val = unscale(tmp_val, &self.unit);
        self
    }

    pub fn w(&self) -> f64 {
        2.0 * PI * unscale(self.val, &self.unit)
    }

    pub fn wavelength(&self, er: f64) -> f64 {
        3e8 / (unscale(self.val, &self.unit) * er.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::comp_f64;
    use float_cmp::{approx_eq, F64Margin};

    #[test]
    fn test_frequency() {
        let val = 280.0;
        let val_unscale = 280e9;
        let w = 2.0 * PI * val_unscale;
        let er = 3.4;
        let wavelength = 3e8 / (280e9 * 3.4_f64.sqrt());
        let unit = Unit::Giga;
        let freq = Frequency::new(val, unit);

        comp_f64(
            &freq.freq(),
            &val_unscale,
            F64Margin::default(),
            "frequency",
            "freq()",
        );
        comp_f64(
            &freq.freq_scaled(),
            &val,
            F64Margin::default(),
            "frequency",
            "freq_scaled()",
        );
        comp_f64(&freq.w(), &w, F64Margin::default(), "frequency", "w()");
        comp_f64(
            &freq.wavelength(er),
            &wavelength,
            F64Margin::default(),
            "frequency",
            "wavelength()",
        );
    }
}
