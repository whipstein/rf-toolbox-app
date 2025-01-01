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
pub struct ShortedStub {
    z0: f64,
    er: f64,
    length: f64,
    length_unit: Unit,
    orientation: Orientation,
}

impl ShortedStub {
    pub fn new(z0: f64, er: f64, length: f64, length_unit: Unit) -> Self {
        ShortedStub {
            z0,
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

    pub fn betal_rot(&self, freq: Frequency) -> f64 {
        match unscale(self.length, &self.length_unit) < (0.5 * freq.wavelength(self.er())) {
            true => {
                self.beta(freq)
                    * (freq.wavelength(self.er) / 4.0
                        + (unscale(self.length, &self.length_unit)
                            - freq.wavelength(self.er) / 4.0))
            }
            false => self.betal(freq),
        }
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

impl Default for ShortedStub {
    fn default() -> Self {
        Self {
            z0: 50.0,
            er: 1.0,
            length: 1.0,
            length_unit: Unit::Micro,
            orientation: Orientation::Shunt,
        }
    }
}

impl Element for ShortedStub {
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
        c64(0, 1) * self.z0 * self.betal(freq).tan()
    }

    fn calc_arc(
        &self,
        freq: Frequency,
        zin_norm: Complex<f64>,
        z0: f64,
        npts: usize,
        verbose: bool,
    ) -> (Vec<f64>, Vec<f64>, (f64, f64), (f64, f64)) {
        let wave =
            match unscale(self.length, &self.length_unit) < (0.5 * freq.wavelength(self.er())) {
                true => freq.wavelength(self.er) / 4.0,
                false => 0.0,
            };

        let mut gx: Vec<f64> = vec![0.0; npts + 1];
        let mut gy: Vec<f64> = vec![0.0; npts + 1];

        for i in 0..=npts {
            let tan_beta = match approx_eq!(f64, wave, 0_f64, F64Margin::default()) {
                true => (self.betal(freq) * (i as f64) / (npts as f64)).tan(),
                false => (self.beta(freq)
                    * (wave
                        + (unscale(self.length, &self.length_unit) - wave) * (i as f64)
                            / (npts as f64)))
                    .tan(),
            };

            let stub_admittance_im = -1.0 / ((tan_beta * self.z0) / z0);
            let pt = find_smith_coord(
                zin_norm.re,
                zin_norm.im + stub_admittance_im,
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
            (
                zin_norm.re,
                zin_norm.im - 1.0 / ((self.betal_rot(freq).tan() * self.z0) / z0),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::{calc_z, comp_c64, comp_f64, comp_vec_f64};

    #[test]
    fn test_shorted_stub() {
        let testname = "shorted_stub";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let line_z0 = 100.0;
        let length = 100.0;
        let er = 1.0;
        let length_unit = Unit::Micro;
        let betal = 2.0 * PI * 280e9 / 3e8 * 100e-6;
        let zout = c64(0.0, 66.43984115131404);
        let zout_norm = zout / z0;
        let element = ShortedStub::new(line_z0, er, length, length_unit);

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

        let wave = 0.00026785714285714287;
        let end_x_coord = -0.12402633147792;
        let end_y_coord = 0.32961159047892885;
        // let real_old = 1.0;
        // let imag_old = -0.7525604988447677;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.0;
        let y2 = -0.7525604988447677;
        let x_coord = vec![
            0.000000000000000000000000000000001616286535251879,
            -0.0006091723074986721,
            -0.0024802699030011905,
            -0.005750516125133088,
            -0.010671833106812956,
            -0.017651898673378862,
            -0.027326495711777227,
            -0.04068482842236281,
            -0.05928905932505083,
            -0.08566695030296562,
            -0.12402633147792,
        ];
        let y_coord = vec![
            -0.000000000000000040203066241915915,
            0.024673897474829988,
            0.049740508282579976,
            0.07561380620910241,
            0.10275186171040049,
            0.1316826076086118,
            0.16303299771485347,
            0.19755903714739476,
            0.23616491434885384,
            0.2798716204418641,
            0.32961159047892885,
        ];
        let freq_int = Frequency::new(280.0, Unit::Giga);
        let zin = c64(1.0, 0.0);
        let z0 = 50.0;
        let npts = 10;
        let test = element.calc_arc(freq_int, zin, z0, npts, false);

        comp_f64(
            &(freq.wavelength(er) / 4.0),
            &wave,
            margin,
            testname,
            "wave",
        );
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
