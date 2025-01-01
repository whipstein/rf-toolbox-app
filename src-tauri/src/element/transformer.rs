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
pub struct Transformer {
    res: f64,
    indp: f64,
    inds: f64,
    m: f64,
    res_unit: Unit,
    indp_unit: Unit,
    inds_unit: Unit,
    m_unit: Unit,
    res_tol: f64,
    indp_tol: f64,
    inds_tol: f64,
    m_tol: f64,
    orientation: Orientation,
}

impl Transformer {
    pub fn new(
        res: f64,
        indp: f64,
        inds: f64,
        m: f64,
        res_unit: Unit,
        indp_unit: Unit,
        inds_unit: Unit,
        m_unit: Unit,
        res_tol: f64,
        indp_tol: f64,
        inds_tol: f64,
        m_tol: f64,
    ) -> Self {
        Transformer {
            res,
            indp,
            inds,
            m,
            res_unit,
            indp_unit,
            inds_unit,
            m_unit,
            res_tol,
            indp_tol,
            inds_tol,
            m_tol,
            orientation: Orientation::Series,
        }
    }

    pub fn res(&self) -> f64 {
        self.res
    }

    pub fn indp(&self) -> f64 {
        self.indp
    }

    pub fn inds(&self) -> f64 {
        self.inds
    }

    pub fn m(&self) -> f64 {
        self.m
    }

    pub fn res_unit(&self) -> Unit {
        self.res_unit
    }

    pub fn indp_unit(&self) -> Unit {
        self.indp_unit
    }

    pub fn inds_unit(&self) -> Unit {
        self.inds_unit
    }

    pub fn m_unit(&self) -> Unit {
        self.m_unit
    }

    pub fn res_tol(&self) -> f64 {
        self.res_tol
    }

    pub fn indp_tol(&self) -> f64 {
        self.indp_tol
    }

    pub fn inds_tol(&self) -> f64 {
        self.inds_tol
    }

    pub fn m_tol(&self) -> f64 {
        self.m_tol
    }

    pub fn set_res(&mut self, val: f64) -> &Self {
        self.res = val;
        self
    }

    pub fn set_indp(&mut self, val: f64) -> &Self {
        self.indp = val;
        self
    }

    pub fn set_inds(&mut self, val: f64) -> &Self {
        self.inds = val;
        self
    }

    pub fn set_m(&mut self, val: f64) -> &Self {
        self.m = val;
        self
    }

    pub fn set_res_unscaled(&mut self, val: f64) -> &Self {
        self.res = scale(val, &self.res_unit);
        self
    }

    pub fn set_indp_unscaled(&mut self, val: f64) -> &Self {
        self.indp = scale(val, &self.indp_unit);
        self
    }

    pub fn set_inds_unscaled(&mut self, val: f64) -> &Self {
        self.inds = scale(val, &self.inds_unit);
        self
    }

    pub fn set_m_unscaled(&mut self, val: f64) -> &Self {
        self.m = scale(val, &self.m_unit);
        self
    }

    pub fn set_res_unit(&mut self, val: Unit) -> &Self {
        self.res_unit = val;
        self
    }

    pub fn set_indp_unit(&mut self, val: Unit) -> &Self {
        self.indp_unit = val;
        self
    }

    pub fn set_inds_unit(&mut self, val: Unit) -> &Self {
        self.inds_unit = val;
        self
    }

    pub fn set_m_unit(&mut self, val: Unit) -> &Self {
        self.m_unit = val;
        self
    }

    pub fn set_res_tol(&mut self, val: f64) -> &Self {
        self.res_tol = val;
        self
    }

    pub fn set_indp_tol(&mut self, val: f64) -> &Self {
        self.indp_tol = val;
        self
    }

    pub fn set_inds_tol(&mut self, val: f64) -> &Self {
        self.inds_tol = val;
        self
    }

    pub fn set_m_tol(&mut self, val: f64) -> &Self {
        self.m_tol = val;
        self
    }

    fn z_tee(&self, freq: Frequency) -> (Complex<f64>, Complex<f64>, Complex<f64>) {
        let lp: f64 = unscale(self.indp, &self.indp_unit);

        let ls = match self.inds_unit {
            Unit::N => self.inds.powi(2) * lp,
            _ => unscale(self.inds, &self.inds_unit),
        };
        let (lp_tee, ls_tee, m_tee) = match self.m_unit {
            Unit::K => (
                lp - self.m * (lp * ls).sqrt(),
                ls - self.m * (lp * ls).sqrt(),
                self.m * (lp * ls).sqrt(),
            ),
            _ => (lp, ls, unscale(self.m, &self.m_unit)),
        };
        let (rp, rs) = match self.res_unit {
            Unit::Q => (freq.w() * lp / self.res, freq.w() * ls / self.res),
            _ => (
                unscale(self.res, &self.res_unit),
                unscale(self.res, &self.res_unit),
            ),
        };

        (
            Complex::new(rp, freq.w() * lp_tee),
            Complex::new(0.0, freq.w() * m_tee),
            Complex::new(rs, freq.w() * ls_tee),
        )
    }

    pub fn z_cascade(&self, freq: Frequency, zin: Complex<f64>) -> Complex<f64> {
        let (zp, zm, zs) = self.z_tee(freq);
        ((zin + zp).inv() + zm.inv()).inv() + zs
    }

    pub fn z_cascade_norm(&self, freq: Frequency, zin_norm: Complex<f64>, z0: f64) -> Complex<f64> {
        self.z_cascade(freq, zin_norm * z0) / z0
    }
}

impl Default for Transformer {
    fn default() -> Self {
        Self {
            res: 0.0,
            indp: 20.0,
            inds: 20.0,
            m: 0.5,
            res_unit: Unit::Base,
            indp_unit: Unit::Pico,
            inds_unit: Unit::Pico,
            m_unit: Unit::K,
            res_tol: 0.0,
            indp_tol: 0.0,
            inds_tol: 0.0,
            m_tol: 0.0,
            orientation: Orientation::Series,
        }
    }
}

impl Element for Transformer {
    fn labels(&self) -> Vec<&str> {
        vec!["res", "indp", "inds", "m"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.res, self.indp, self.inds, self.m]
    }

    fn units(&self) -> Vec<Unit> {
        vec![self.res_unit, self.indp_unit, self.inds_unit, self.m_unit]
    }

    fn tols(&self) -> Vec<f64> {
        vec![self.res_tol, self.indp_tol, self.inds_tol, self.m_tol]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn z(&self, freq: Frequency) -> Complex<f64> {
        let mut rp: f64 = unscale(self.res, &self.res_unit);
        let mut rs: f64 = unscale(self.res, &self.res_unit);
        let lp: f64 = unscale(self.indp, &self.indp_unit);
        let mut ls: f64 = unscale(self.inds, &self.inds_unit);
        let mut lp_tee: f64 = 0.0;
        let mut ls_tee: f64 = 0.0;
        let mut lm_tee: f64 = unscale(self.m, &self.m_unit);

        if self.inds_unit == Unit::N {
            ls = ls.powi(2) * lp;
        }
        if self.m_unit == Unit::K {
            lm_tee = (lp * ls).sqrt() * self.m();
            lp_tee = lp - lm_tee;
            ls_tee = ls - lm_tee;
        }
        if self.res_unit == Unit::Q {
            rp = freq.w() * lp / rp;
            rs = freq.w() * ls / rs;
        }

        let z1 = Complex::new(rp, freq.w() * lp_tee);
        let z2 = Complex::new(0.0, freq.w() * lm_tee);
        let z3 = Complex::new(rs, freq.w() * ls_tee);

        (z1.inv() + z2.inv()).inv() + z3
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

        let start = zin_norm;
        let end = self.z_cascade_norm(freq, zin_norm, z0);
        for i in 0..=npts {
            let xpt = start.re + ((end.re - start.re) * (i as f64)) / (npts as f64);
            let ypt = start.im + ((end.im - start.im) * (i as f64)) / (npts as f64);
            let pt = find_smith_coord(xpt, ypt, false, verbose).unwrap();
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
    fn test_transformer() {
        let testname = "transformer";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 20.0;
        let indp = 10.0;
        let inds = 25.0;
        let m = 0.35;
        let res_unit = Unit::Q;
        let indp_unit = Unit::Pico;
        let inds_unit = Unit::Pico;
        let m_unit = Unit::K;
        let zout = c64(2.467834628131633, 38.607901737881555);
        let zout_norm = zout / z0;
        let element = Transformer::new(
            res, indp, inds, m, res_unit, indp_unit, inds_unit, m_unit, 0.0, 0.0, 0.0, 0.0,
        );

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        comp_f64(&element.indp(), &indp, margin, testname, "indp()");
        comp_f64(&element.inds(), &inds, margin, testname, "inds()");
        comp_f64(&element.m(), &m, margin, testname, "m()");
        comp_f64(
            &unscale(element.indp(), &element.indp_unit()),
            &(indp * 1e-12),
            margin,
            testname,
            "indp_unscale()",
        );
        comp_f64(
            &unscale(element.inds(), &element.inds_unit()),
            &(inds * 1e-12),
            margin,
            testname,
            "inds_unscale()",
        );
        comp_f64(
            &unscale(element.m(), &element.m_unit()),
            &m,
            margin,
            testname,
            "m_unscale()",
        );
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.indp_unit(), &indp_unit);
        assert_eq!(&element.inds_unit(), &inds_unit);
        assert_eq!(&element.m_unit(), &m_unit);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = -0.12557273923087667;
        let end_y_coord = 0.9070700735284939;
        // let real_old = 0.0;
        // let imag_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 0.07726282865103465;
        let y2 = 0.8681383611526633;
        let x_coord = vec![
            0.0,
            -0.04620191289340646,
            -0.091668432746296,
            -0.13472472183338424,
            -0.17312828829235238,
            -0.20402989225662477,
            -0.22403100581093902,
            -0.2293990143631755,
            -0.21648979179286468,
            -0.18237492324617782,
            -0.12557273923087667,
        ];
        let y_coord = vec![
            0.0,
            0.04760892703800349,
            0.10440583929498334,
            0.17150246380546133,
            0.2497846502470636,
            0.3396734681582891,
            0.4408155493411283,
            0.5517397169425321,
            0.6695659309420381,
            0.7899013747369822,
            0.9070700735284939,
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
