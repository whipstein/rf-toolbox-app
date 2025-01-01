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
pub struct Capacitor {
    res: f64,
    cap: f64,
    res_unit: Unit,
    cap_unit: Unit,
    res_tol: f64,
    cap_tol: f64,
    orientation: Orientation,
}

impl Capacitor {
    pub fn new(
        res: f64,
        cap: f64,
        res_unit: Unit,
        cap_unit: Unit,
        res_tol: f64,
        cap_tol: f64,
        orientation: Orientation,
    ) -> Self {
        Capacitor {
            res,
            cap,
            res_unit,
            cap_unit,
            res_tol,
            cap_tol,
            orientation,
        }
    }

    pub fn res(&self) -> f64 {
        self.res
    }

    pub fn cap(&self) -> f64 {
        self.cap
    }

    pub fn res_unit(&self) -> Unit {
        self.res_unit
    }

    pub fn cap_unit(&self) -> Unit {
        self.cap_unit
    }

    pub fn res_tol(&self) -> f64 {
        self.res_tol
    }

    pub fn cap_tol(&self) -> f64 {
        self.cap_tol
    }

    pub fn set_res(&mut self, val: f64) -> &Self {
        self.res = val;
        self
    }

    pub fn set_cap(&mut self, val: f64) -> &Self {
        self.cap = val;
        self
    }

    pub fn set_res_unscaled(&mut self, val: f64) -> &Self {
        self.res = scale(val, &self.res_unit);
        self
    }

    pub fn set_cap_unscaled(&mut self, val: f64) -> &Self {
        self.cap = scale(val, &self.cap_unit);
        self
    }

    pub fn set_res_unit(&mut self, val: Unit) -> &Self {
        self.res_unit = val;
        self
    }

    pub fn set_cap_unit(&mut self, val: Unit) -> &Self {
        self.cap_unit = val;
        self
    }

    pub fn set_res_tol(&mut self, val: f64) -> &Self {
        self.res_tol = val;
        self
    }

    pub fn set_cap_tol(&mut self, val: f64) -> &Self {
        self.cap_tol = val;
        self
    }

    pub fn set_orientation(&mut self, val: Orientation) -> &Self {
        self.orientation = val;
        self
    }
}

impl Default for Capacitor {
    fn default() -> Self {
        Self {
            res: 0.0,
            cap: 20.0,
            res_unit: Unit::Base,
            cap_unit: Unit::Femto,
            res_tol: 0.0,
            cap_tol: 0.0,
            orientation: Orientation::Series,
        }
    }
}

impl Element for Capacitor {
    fn labels(&self) -> Vec<&str> {
        vec!["res", "cap"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.res, self.cap]
    }

    fn units(&self) -> Vec<Unit> {
        vec![self.res_unit, self.cap_unit]
    }

    fn tols(&self) -> Vec<f64> {
        vec![self.res_tol, self.cap_tol]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn z(&self, freq: Frequency) -> Complex<f64> {
        let res = match self.res_unit {
            Unit::Q => {
                if approx_eq!(f64, self.res, 0_f64, F64Margin::default()) {
                    0.0
                } else {
                    1.0 / (freq.w() * unscale(self.cap, &self.cap_unit) * self.res)
                }
            }
            _ => unscale(self.res, &self.res_unit),
        };
        c64(res, -1.0 / (freq.w() * unscale(self.cap, &self.cap_unit)))
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
    fn test_capacitor_series() {
        let testname = "capacitor_series";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 0.0;
        let cap = 20.0;
        let res_unit = Unit::Q;
        let cap_unit = Unit::Femto;
        let orientation = Orientation::Series;
        let zout = c64(0.0, -28.420525552124168);
        let zout_norm = zout / z0;
        let element = Capacitor::new(
            0.0,
            20.0,
            Unit::Q,
            Unit::Femto,
            0.0,
            0.0,
            Orientation::Series,
        );

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        comp_f64(&element.cap(), &cap, margin, testname, "cap()");
        comp_f64(
            &unscale(element.cap(), &element.cap_unit()),
            &(cap * 1e-15),
            margin,
            testname,
            "cap_unscale()",
        );
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.cap_unit(), &cap_unit);
        assert_eq!(&element.orientation(), &orientation);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = 0.07473600388106642;
        let end_y_coord = -0.2629648904415866;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.0;
        let y2 = -0.5684105110424832;
        let x_coord = vec![
            0.0,
            0.0008070743774802617,
            0.003220499960917648,
            0.007217071688202465,
            0.012758731362111475,
            0.019793464288161443,
            0.0282564992114107,
            0.03807176084762063,
            0.04915351593258457,
            0.06140814908058681,
            0.07473600388106642,
        ];
        let y_coord = vec![
            0.0,
            -0.02839758807415652,
            -0.056657994501388566,
            -0.08464623774539427,
            -0.11223166280573833,
            -0.13928992447278707,
            -0.16570476596563613,
            -0.19136954270098003,
            -0.2161884543726832,
            -0.24007746313863623,
            -0.2629648904415866,
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

    #[test]
    fn test_capacitor_shunt() {
        let testname = "capacitor_shunt";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 0.0;
        let cap = 20.0;
        let res_unit = Unit::Q;
        let cap_unit = Unit::Femto;
        let orientation = Orientation::Shunt;
        let zout = c64(0.0, -28.420525552124168);
        let zout_norm = zout / z0;
        let element = Capacitor::new(
            0.0,
            20.0,
            Unit::Q,
            Unit::Femto,
            0.0,
            0.0,
            Orientation::Shunt,
        );

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        comp_f64(&element.cap(), &cap, margin, testname, "cap()");
        comp_f64(
            &unscale(element.cap(), &element.cap_unit()),
            &(cap * 1e-15),
            margin,
            testname,
            "cap_unscale()",
        );
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.cap_unit(), &cap_unit);
        assert_eq!(&element.orientation(), &orientation);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = -0.4362312689639497;
        let end_y_coord = -0.49591687704901904;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let x2 = 1.0;
        let x_coord = vec![
            0.0,
            -0.007678356495065491,
            -0.03002187011606294,
            -0.06510595461943307,
            -0.1101653692440527,
            -0.16208905180727595,
            -0.21786992928987509,
            -0.27491608889389896,
            -0.331200876436476,
            -0.3852809297503982,
            -0.4362312689639497,
        ];
        let y1 = 0.0;
        let y2 = 1.7592918860102849;
        let y_coord = vec![
            0.0,
            -0.08728917078653108,
            -0.17064746535122405,
            -0.24671272624760437,
            -0.3130957691559792,
            -0.3685324830886609,
            -0.4127985261614919,
            -0.44647198451994824,
            -0.4706451485824391,
            -0.4866616226096597,
            -0.49591687704901904,
        ];
        let freq_int = Frequency::new(280.0, Unit::Giga);
        let zin = c64(1.0, 0.0);
        let z0 = 50.0;
        let npts = 10;
        let test = element.calc_arc(freq_int, zin, z0, npts, false);

        let margin = F64Margin::from((1e-15, 1));

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
