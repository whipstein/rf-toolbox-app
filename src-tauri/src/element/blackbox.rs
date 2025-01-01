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
pub struct BlackBox {
    z: Complex<f64>,
    z0: f64,
    tol: f64,
    orientation: Orientation,
}

impl BlackBox {
    pub fn new(z: Complex<f64>, z0: f64, tol: f64) -> Self {
        BlackBox {
            z,
            z0,
            tol,
            orientation: Orientation::Series,
        }
    }

    pub fn from_ri(re: f64, im: f64, z0: f64, tol: f64) -> Self {
        BlackBox {
            z: c64(re, im),
            z0,
            tol,
            orientation: Orientation::Series,
        }
    }

    pub fn res(&self) -> f64 {
        self.z.re
    }

    pub fn reac(&self) -> f64 {
        self.z.im
    }

    pub fn tol(&self) -> f64 {
        self.tol
    }

    pub fn set_res(&mut self, val: f64) -> &Self {
        self.z = c64(val, self.z.im);
        self
    }

    pub fn set_reac(&mut self, val: f64) -> &Self {
        self.z = c64(self.z.re, val);
        self
    }

    pub fn set_z(&mut self, val: Complex<f64>) -> &Self {
        self.z = val;
        self
    }

    pub fn set_tol(&mut self, val: f64) -> &Self {
        self.tol = val;
        self
    }
}

impl Default for BlackBox {
    fn default() -> Self {
        Self {
            z: c64(50.0, 0.0),
            z0: 50.0,
            tol: 0.0,
            orientation: Orientation::Series,
        }
    }
}

impl Element for BlackBox {
    fn labels(&self) -> Vec<&str> {
        vec!["res", "reac"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.res(), self.reac()]
    }

    fn units(&self) -> Vec<Unit> {
        vec![Unit::Base, Unit::Base]
    }

    fn tols(&self) -> Vec<f64> {
        vec![self.tol]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn r(&self, _freq: Frequency) -> f64 {
        self.res()
    }

    fn x(&self, _freq: Frequency) -> f64 {
        self.reac()
    }

    fn z(&self, _freq: Frequency) -> Complex<f64> {
        c64(self.res(), self.reac())
    }

    fn calc_arc(
        &self,
        freq: Frequency,
        _zin_norm: Complex<f64>,
        z0: f64,
        _npts: usize,
        verbose: bool,
    ) -> (Vec<f64>, Vec<f64>, (f64, f64), (f64, f64)) {
        let pt = find_smith_coord_c64(self.z_norm(freq, self.z0), self.orientation.into(), verbose)
            .unwrap();

        let pt1 = calc_z_norm(pt, z0);

        (vec![pt.re], vec![pt.im], (pt1.re, pt1.im), (pt1.re, pt1.im))
    }
}
