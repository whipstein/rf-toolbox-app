#![allow(unused)]
use crate::frequency::Frequency;
use crate::rf_utils::{calc_z_norm, scale, unscale};
use crate::smith::{find_smith_coord, find_smith_coord_c64};
use crate::unit::Unit;
use float_cmp::{approx_eq, F64Margin};
use num_complex::{c64, Complex};
use std::f64::consts::PI;

pub mod blackbox;
pub mod capacitor;
pub mod inductor;
pub mod openstub;
pub mod resistor;
pub mod rlc;
pub mod shortedstub;
pub mod tline;
pub mod transformer;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Series,
    Shunt,
}

impl From<Orientation> for bool {
    fn from(f: Orientation) -> bool {
        match f {
            Orientation::Series => false,
            Orientation::Shunt => true,
        }
    }
}

pub trait Element {
    fn labels(&self) -> Vec<&str>;
    fn vals(&self) -> Vec<f64>;
    fn units(&self) -> Vec<Unit>;
    fn tols(&self) -> Vec<f64>;
    fn orientation(&self) -> Orientation;

    fn r(&self, freq: Frequency) -> f64 {
        self.z(freq).re
    }
    fn x(&self, freq: Frequency) -> f64 {
        self.z(freq).im
    }

    fn z(&self, freq: Frequency) -> Complex<f64>;

    fn z_norm(&self, freq: Frequency, z0: f64) -> Complex<f64> {
        self.z(freq) / z0
    }

    fn calc_arc(
        &self,
        freq: Frequency,
        zin_norm: Complex<f64>,
        z0: f64,
        npts: usize,
        verbose: bool,
    ) -> (Vec<f64>, Vec<f64>, (f64, f64), (f64, f64));
}
