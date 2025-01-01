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
pub struct OpenStub {
    z0: f64,
    zl: Complex<f64>,
    er: f64,
    length: f64,
    length_unit: Unit,
    orientation: Orientation,
}

impl OpenStub {
    pub fn new(z0: f64, zl: Complex<f64>, er: f64, length: f64, length_unit: Unit) -> Self {
        OpenStub {
            z0,
            zl,
            er,
            length,
            length_unit,
            orientation: Orientation::Shunt,
        }
    }

    pub fn z0(&self) -> f64 {
        self.z0
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

impl Default for OpenStub {
    fn default() -> Self {
        Self {
            z0: 50.0,
            zl: c64(50.0, 0.0),
            er: 1.0,
            length: 1.0,
            length_unit: Unit::Micro,
            orientation: Orientation::Shunt,
        }
    }
}

impl Element for OpenStub {
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
        c64(0, -1) * self.z0 / self.betal(freq).tan()
    }

    fn calc_arc(
        &self,
        freq: Frequency,
        zin_norm: Complex<f64>,
        z0: f64,
        npts: usize,
        verbose: bool,
    ) -> (Vec<f64>, Vec<f64>, (f64, f64), (f64, f64)) {
        let mut gx: Vec<f64> = vec![0.0; npts + 1];
        let mut gy: Vec<f64> = vec![0.0; npts + 1];
        let yout_norm = c64(
            zin_norm.re,
            zin_norm.im + self.betal(freq).tan() / (self.z0 / z0),
        );

        for i in 0..=npts {
            let tan_beta_arg = self.betal(freq) * (i as f64) / (npts as f64);

            let pt = find_smith_coord(
                zin_norm.re,
                zin_norm.im + tan_beta_arg.tan() / (self.z0 / z0),
                self.orientation.into(),
                verbose,
            )
            .unwrap();

            gx[i] = pt.re;
            gy[i] = pt.im;
        }

        (
            gx,
            gy,
            (zin_norm.re, zin_norm.im),
            (yout_norm.re, yout_norm.im),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::{calc_z, comp_c64, comp_f64, comp_vec_f64};

    #[test]
    fn test_open_stub() {
        let testname = "open_stub";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        let zl = c64(1.0, 0.0);
        let zl_denorm = zl * z0;
        let line_z0 = 100.0;
        let length = 100.0;
        let er = 1.0;
        let length_unit = Unit::Micro;
        let betal = 2.0 * PI * 280e9 / 3e8 * 100e-6;
        let zout = c64(0.0, -150.51209976895348);
        let zout_norm = zout / z0;
        let element = OpenStub::new(line_z0, zl_denorm, er, length, length_unit);

        let margin = F64Margin::default();

        comp_f64(&element.z0(), &line_z0, margin, testname, "z0()");
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

        let end_x_coord = -0.026848356667989796;
        let end_y_coord = -0.1616401014977974;
        // let real_old = 1.0;
        // let imag_old = 0.3321992057565702;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.0;
        let y2 = 0.3321992057565702;
        let x_coord = vec![
            0.0,
            -0.0002153853951533664,
            -0.0008669463825723525,
            -0.00197117617976264,
            -0.0035565165989995015,
            -0.005664873765432374,
            -0.008353923265671647,
            -0.011700415514613685,
            -0.015804796950192775,
            -0.020797610908692466,
            -0.026848356667989796,
        ];
        let y_coord = vec![
            0.0,
            -0.014674433695542577,
            -0.029431187311118066,
            -0.04435415024809974,
            -0.05953039382265587,
            -0.07505186853539411,
            -0.09101722491783025,
            -0.10753378906836233,
            -0.12471970711782476,
            -0.14270623773746924,
            -0.1616401014977974,
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
