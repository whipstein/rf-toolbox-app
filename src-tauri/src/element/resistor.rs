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
pub struct Resistor {
    res: f64,
    res_unit: Unit,
    res_tol: f64,
    orientation: Orientation,
}

impl Resistor {
    pub fn new(res: f64, res_unit: Unit, res_tol: f64, orientation: Orientation) -> Self {
        Resistor {
            res,
            res_unit,
            res_tol,
            orientation,
        }
    }

    pub fn res(&self) -> f64 {
        self.res
    }

    pub fn res_unit(&self) -> Unit {
        self.res_unit
    }

    pub fn res_tol(&self) -> f64 {
        self.res_tol
    }

    pub fn set_res(&mut self, val: f64) -> &Self {
        self.res = val;
        self
    }

    pub fn set_res_unscaled(&mut self, val: f64) -> &Self {
        self.res = scale(val, &self.res_unit);
        self
    }

    pub fn set_res_unit(&mut self, val: Unit) -> &Self {
        self.res_unit = val;
        self
    }

    pub fn set_res_tol(&mut self, val: f64) -> &Self {
        self.res_tol = val;
        self
    }

    pub fn set_orientation(&mut self, val: Orientation) -> &Self {
        self.orientation = val;
        self
    }
}

impl Default for Resistor {
    fn default() -> Self {
        Self {
            res: 1.0,
            res_unit: Unit::Base,
            res_tol: 0.0,
            orientation: Orientation::Series,
        }
    }
}

impl Element for Resistor {
    fn labels(&self) -> Vec<&str> {
        vec!["res"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.res]
    }

    fn units(&self) -> Vec<Unit> {
        vec![self.res_unit]
    }

    fn tols(&self) -> Vec<f64> {
        vec![self.res_tol]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn z(&self, _freq: Frequency) -> Complex<f64> {
        c64(unscale(self.res, &self.res_unit), 0.0)
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

        let (start, end) = match self.orientation {
            Orientation::Series => (zin_norm, self.z_norm(freq, z0) + zin_norm),
            Orientation::Shunt => (zin_norm.inv(), self.z_norm(freq, z0).inv() + zin_norm.inv()),
        };
        for i in 0..=npts {
            let xpt = start.re + ((end.re - start.re) * (i as f64)) / (npts as f64);
            let ypt = start.im + ((end.im - start.im) * (i as f64)) / (npts as f64);
            let pt = find_smith_coord(xpt, ypt, self.orientation.into(), verbose).unwrap();
            gx[i] = pt.re;
            gy[i] = pt.im;
        }

        (gx, gy, (start.re, start.im), (end.re, end.im))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::{calc_z, comp_c64, comp_f64, comp_vec_f64};

    #[test]
    fn test_resistor_series() {
        let testname = "resistor_series";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 10.0;
        let res_unit = Unit::Base;
        let orientation = Orientation::Series;
        let zout = c64(10.0, 0.0);
        let zout_norm = zout / z0;
        let element = Resistor::new(10.0, Unit::Base, 0.0, Orientation::Series);

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.orientation(), &orientation);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = 0.09090909090909088;
        let end_y_coord = 0.0;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let x2 = 1.2;
        let x_coord = vec![
            0.0,
            0.00990099009900991,
            0.01960784313725492,
            0.02912621359223304,
            0.03846153846153849,
            0.04761904761904766,
            0.056603773584905606,
            0.06542056074766352,
            0.07407407407407404,
            0.08256880733944952,
            0.09090909090909088,
        ];
        let y1 = 0.0;
        let y2 = 0.0;
        let y_coord = vec![0.0; 11];
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

    #[test]
    fn test_resistor_shunt() {
        let testname = "resistor_shunt";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 10.0;
        let res_unit = Unit::Base;
        let orientation = Orientation::Shunt;
        let zout = c64(10.0, 0.0);
        let zout_norm = zout / z0;
        let element = Resistor::new(10.0, Unit::Base, 0.0, Orientation::Shunt);

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.orientation(), &orientation);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = -0.7142857142857141;
        let end_y_coord = 0.0;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let x2 = 5.999999999999999;
        let x_coord = vec![
            0.0,
            -0.20000000000000004,
            -0.33333333333333326,
            -0.4285714285714286,
            -0.49999999999999994,
            -0.5555555555555554,
            -0.6,
            -0.6363636363636362,
            -0.6666666666666666,
            -0.6923076923076922,
            -0.7142857142857141,
        ];
        let y1 = 0.0;
        let y2 = 0.0;
        let y_coord = vec![0.0; 11];
        let freq_int = Frequency::new(280.0, Unit::Giga);
        let zin = c64(1.0, 0.0);
        let z0 = 50.0;
        let npts = 10;
        let test = element.calc_arc(freq_int, zin, z0, npts, false);

        let margin = F64Margin::from((1e-10, 1));

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
