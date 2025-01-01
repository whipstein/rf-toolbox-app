#![allow(unused)]
use crate::element::{Element, Orientation};
use crate::frequency::Frequency;
use crate::rf_utils::{calc_z_norm, scale, unscale};
use crate::smith::{find_smith_coord, find_smith_coord_c64};
use crate::unit::Unit;
use float_cmp::{approx_eq, F64Margin};
use num_complex::{c64, Complex};
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TLine {
    z0: f64,
    zl: Complex<f64>,
    er: f64,
    length: f64,
    length_unit: Unit,
    orientation: Orientation,
}

impl TLine {
    pub fn new(z0: f64, zl: Complex<f64>, er: f64, length: f64, length_unit: Unit) -> Self {
        TLine {
            z0,
            zl,
            er,
            length,
            length_unit,
            orientation: Orientation::Series,
        }
    }

    pub fn z0(&self) -> f64 {
        self.z0
    }

    pub fn zl(&self) -> Complex<f64> {
        self.zl
    }

    pub fn er(&self) -> f64 {
        self.er
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn length_unit(&self) -> Unit {
        self.length_unit
    }

    pub fn beta(&self, freq: Frequency) -> f64 {
        freq.w() * self.er().sqrt() / 3e8
    }

    pub fn betal(&self, freq: Frequency) -> f64 {
        self.beta(freq) * unscale(self.length, &self.length_unit)
    }

    pub fn set_z0(&mut self, val: f64) -> &Self {
        self.z0 = val;
        self
    }

    pub fn set_zl(&mut self, val: Complex<f64>) -> &Self {
        self.zl = val;
        self
    }

    pub fn set_er(&mut self, val: f64) -> &Self {
        self.er = val;
        self
    }

    pub fn set_length(&mut self, val: f64) -> &Self {
        self.length = val;
        self
    }

    pub fn set_length_unscaled(&mut self, val: f64) -> &Self {
        self.length = scale(val, &self.length_unit);
        self
    }

    pub fn set_length_unit(&mut self, val: Unit) -> &Self {
        self.length_unit = val;
        self
    }
}

impl Default for TLine {
    fn default() -> Self {
        Self {
            z0: 50.0,
            zl: c64(50.0, 0.0),
            er: 1.0,
            length: 1.0,
            length_unit: Unit::Micro,
            orientation: Orientation::Series,
        }
    }
}

impl Element for TLine {
    fn labels(&self) -> Vec<&str> {
        vec!["z0", "length"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.z0, self.length]
    }

    fn units(&self) -> Vec<Unit> {
        vec![Unit::Base, self.length_unit]
    }

    fn tols(&self) -> Vec<f64> {
        vec![]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn z(&self, freq: Frequency) -> Complex<f64> {
        self.z0 * (self.zl + c64(0, 1) * self.z0 * self.betal(freq).tan())
            / (self.z0() + c64(0, 1) * self.zl * self.betal(freq).tan())
    }

    fn calc_arc(
        &self,
        freq: Frequency,
        _zin_norm: Complex<f64>,
        z0: f64,
        npts: usize,
        verbose: bool,
    ) -> (Vec<f64>, Vec<f64>, (f64, f64), (f64, f64)) {
        let mut gx: Vec<f64> = vec![0.0; npts + 1];
        let mut gy: Vec<f64> = vec![0.0; npts + 1];

        for i in 0..=npts {
            let betal = self.betal(freq) * (i as f64) / (npts as f64);
            let zout = self.z0 * (self.zl + c64(0, 1) * self.z0 * betal.tan())
                / (self.z0 + c64(0, 1) * self.zl * betal.tan())
                / z0;
            let pt = find_smith_coord(zout.re, zout.im, self.orientation.into(), verbose).unwrap();
            gx[i] = pt.re;
            gy[i] = pt.im;
        }

        let pt1 = calc_z_norm(c64(gx[0], gy[0]), z0);
        let pt2 = calc_z_norm(c64(*gx.last().unwrap(), *gy.last().unwrap()), z0);
        (gx, gy, (pt1.re, pt1.im), (pt2.re, pt2.im))

        // let zout = self.z_norm(freq, z0);
        // (gx, gy, (zin.re, zin.im), (zout.re, zout.im))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::{calc_z, comp_c64, comp_f64, comp_vec_f64};

    #[test]
    fn test_tline() {
        let testname = "tline";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        let zl = c64(1.0, 0.0);
        let zl_denorm = zl * z0;
        let line_z0 = 100.0;
        let length = 100.0;
        let er = 1.0;
        let length_unit = Unit::Micro;
        let betal = 2.0 * PI * 280e9 / 3e8 * 100e-6;
        let zout = c64(64.90822960372651, 44.877378829891);
        let zout_norm = zout / z0;
        let element = TLine::new(line_z0, zl_denorm, er, length, length_unit);

        let margin = F64Margin::from((1e-12, 1));

        comp_f64(&element.z0(), &line_z0, margin, testname, "z0()");
        comp_c64(&element.zl(), &zl_denorm, margin, testname, "zl()");
        comp_f64(&element.er(), &er, margin, testname, "er()");
        comp_f64(&element.length(), &length, margin, testname, "length()");
        assert_eq!(&element.length_unit(), &length_unit);
        comp_f64(&element.betal(freq), &betal, margin, testname, "betal()");
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = 0.24491304389596288;
        let end_y_coord = 0.2948990119806981;
        // let real_old = 1.29816459207453;
        // let imag_old = 0.8975475765978199;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.29816459207453;
        let y2 = 0.8975475765978199;
        let x_coord = vec![
            0.0,
            0.0032141661008425397,
            0.012739136185784873,
            0.02823203513260908,
            0.04915228681087233,
            0.07480310319657636,
            0.10438105684293945,
            0.1370275345351117,
            0.17187657094873007,
            0.2080950476414447,
            0.24491304389596288,
        ];
        let y_coord = vec![
            0.0,
            0.043796903963428,
            0.08649390799767896,
            0.12705185268950858,
            0.16454611751051781,
            0.19820786480386732,
            0.2274493989398845,
            0.2518729352278561,
            0.2712644225255944,
            0.28557569877699973,
            0.2948990119806981,
        ];
        let freq_int = Frequency::new(280.0, Unit::Giga);
        let zin = c64(1.0, 0.0);
        let z0 = 50.0;
        let npts = 10;
        let test = element.calc_arc(freq_int, zin, z0, npts, false);

        comp_f64(&test.2 .0, &x1, margin, testname, "x1");
        comp_f64(&test.2 .1, &y1, margin, testname, "y1");
        comp_f64(&test.3 .0, &x2, margin, testname, "x2");
        comp_f64(&test.3 .1, &y2, margin, testname, "y2");
        comp_f64(
            test.0.last().unwrap(),
            &end_x_coord,
            margin,
            testname,
            "end_x_coord",
        );
        comp_f64(
            test.1.last().unwrap(),
            &end_y_coord,
            margin,
            testname,
            "end_y_coord",
        );
        comp_vec_f64(test.0, x_coord, margin, testname, "x_coord");
        comp_vec_f64(test.1, y_coord, margin, testname, "y_coord");
    }
}
