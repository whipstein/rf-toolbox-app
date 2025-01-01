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
pub struct Rlc {
    res: f64,
    ind: f64,
    cap: f64,
    res_unit: Unit,
    ind_unit: Unit,
    cap_unit: Unit,
    res_tol: f64,
    ind_tol: f64,
    cap_tol: f64,
    orientation: Orientation,
}

impl Rlc {
    pub fn new(
        res: f64,
        ind: f64,
        cap: f64,
        res_unit: Unit,
        ind_unit: Unit,
        cap_unit: Unit,
        res_tol: f64,
        ind_tol: f64,
        cap_tol: f64,
        orientation: Orientation,
    ) -> Self {
        Rlc {
            res,
            ind,
            cap,
            res_unit,
            ind_unit,
            cap_unit,
            res_tol,
            ind_tol,
            cap_tol,
            orientation,
        }
    }

    pub fn res(&self) -> f64 {
        self.res
    }

    pub fn ind(&self) -> f64 {
        self.ind
    }

    pub fn cap(&self) -> f64 {
        self.cap
    }

    pub fn res_unit(&self) -> Unit {
        self.res_unit
    }

    pub fn ind_unit(&self) -> Unit {
        self.ind_unit
    }

    pub fn cap_unit(&self) -> Unit {
        self.cap_unit
    }

    pub fn res_tol(&self) -> f64 {
        self.res_tol
    }

    pub fn ind_tol(&self) -> f64 {
        self.ind_tol
    }

    pub fn cap_tol(&self) -> f64 {
        self.cap_tol
    }

    pub fn set_res(&mut self, val: f64) -> &Self {
        self.res = val;
        self
    }

    pub fn set_ind(&mut self, val: f64) -> &Self {
        self.ind = val;
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

    pub fn set_ind_unscaled(&mut self, val: f64) -> &Self {
        self.ind = scale(val, &self.ind_unit);
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

    pub fn set_ind_unit(&mut self, val: Unit) -> &Self {
        self.ind_unit = val;
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

    pub fn set_ind_tol(&mut self, val: f64) -> &Self {
        self.ind_tol = val;
        self
    }

    pub fn set_cap_tol(&mut self, val: f64) -> &Self {
        self.cap_tol = val;
        self
    }
}

impl Default for Rlc {
    fn default() -> Self {
        Self {
            res: 1.0,
            ind: 10.0,
            cap: 20.0,
            res_unit: Unit::Base,
            ind_unit: Unit::Pico,
            cap_unit: Unit::Femto,
            res_tol: 0.0,
            ind_tol: 0.0,
            cap_tol: 0.0,
            orientation: Orientation::Shunt,
        }
    }
}

impl Element for Rlc {
    fn labels(&self) -> Vec<&str> {
        vec!["res", "ind", "cap"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.res, self.ind, self.cap]
    }

    fn units(&self) -> Vec<Unit> {
        vec![self.res_unit, self.ind_unit, self.cap_unit]
    }

    fn tols(&self) -> Vec<f64> {
        vec![self.res_tol, self.ind_tol, self.cap_tol]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn z(&self, freq: Frequency) -> Complex<f64> {
        if approx_eq!(f64, self.cap, 0_f64, F64Margin::default()) {
            c64(
                unscale(self.res, &self.res_unit),
                freq.w() * unscale(self.ind, &self.ind_unit),
            )
        } else {
            c64(
                unscale(self.res, &self.res_unit),
                freq.w() * unscale(self.ind, &self.ind_unit)
                    - 1.0 / (freq.w() * unscale(self.cap, &self.cap_unit)),
            )
        }
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
    fn test_rlc_shunt() {
        let testname = "rlc_shunt";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 1.0;
        let ind = 10.0;
        let cap = 20.0;
        let res_unit = Unit::Base;
        let ind_unit = Unit::Pico;
        let cap_unit = Unit::Femto;
        let zout = c64(1.0, -10.827606692021327);
        let zout_norm = zout / z0;
        let element = Rlc::new(
            1.0,
            10.0,
            20.0,
            Unit::Base,
            Unit::Pico,
            Unit::Femto,
            0.0,
            0.0,
            0.0,
            Orientation::Shunt,
        );

        let margin = F64Margin::from((1e-12, 1));

        comp_f64(&element.res(), &res, margin, testname, "res()");
        comp_f64(&element.ind(), &ind, margin, testname, "ind()");
        comp_f64(&element.cap(), &cap, margin, testname, "cap()");
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.ind_unit(), &ind_unit);
        assert_eq!(&element.cap_unit(), &cap_unit);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = -0.8194271640921553;
        let end_y_coord = -0.34124750175185664;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.4228792324199453;
        let y2 = 4.578770006867039;
        let x_coord = vec![
            0.0,
            -0.06757431629692738,
            -0.19577508660626358,
            -0.33643457106565,
            -0.46172495965578997,
            -0.5634633971754767,
            -0.6430229509000751,
            -0.7045900167360067,
            -0.7523685277655275,
            -0.7897837408934103,
            -0.8194271640921553,
        ];
        let y_coord = vec![
            0.0,
            -0.20904803409867867,
            -0.35329593964483896,
            -0.4285624956585593,
            -0.45448875118566406,
            -0.4519229666641672,
            -0.43514997539305933,
            -0.4123796529914703,
            -0.38792145267144057,
            -0.3638922433254095,
            -0.34124750175185664,
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
