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
pub struct Inductor {
    res: f64,
    ind: f64,
    res_unit: Unit,
    ind_unit: Unit,
    res_tol: f64,
    ind_tol: f64,
    orientation: Orientation,
}

impl Inductor {
    pub fn new(
        res: f64,
        ind: f64,
        res_unit: Unit,
        ind_unit: Unit,
        res_tol: f64,
        ind_tol: f64,
        orientation: Orientation,
    ) -> Self {
        Inductor {
            res,
            ind,
            res_unit,
            ind_unit,
            res_tol,
            ind_tol,
            orientation,
        }
    }

    pub fn res(&self) -> f64 {
        self.res
    }

    pub fn ind(&self) -> f64 {
        self.ind
    }

    pub fn res_unit(&self) -> Unit {
        self.res_unit
    }

    pub fn ind_unit(&self) -> Unit {
        self.ind_unit
    }

    pub fn res_tol(&self) -> f64 {
        self.res_tol
    }

    pub fn ind_tol(&self) -> f64 {
        self.ind_tol
    }

    pub fn set_res(&mut self, val: f64) -> &Self {
        self.res = val;
        self
    }

    pub fn set_ind(&mut self, val: f64) -> &Self {
        self.ind = val;
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

    pub fn set_res_unit(&mut self, val: Unit) -> &Self {
        self.res_unit = val;
        self
    }

    pub fn set_ind_unit(&mut self, val: Unit) -> &Self {
        self.ind_unit = val;
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

    pub fn set_orientation(&mut self, val: Orientation) -> &Self {
        self.orientation = val;
        self
    }
}

impl Default for Inductor {
    fn default() -> Self {
        Self {
            res: 0.0,
            ind: 10.0,
            res_unit: Unit::Base,
            ind_unit: Unit::Pico,
            res_tol: 0.0,
            ind_tol: 0.0,
            orientation: Orientation::Series,
        }
    }
}

impl Element for Inductor {
    fn labels(&self) -> Vec<&str> {
        vec!["res", "ind"]
    }

    fn vals(&self) -> Vec<f64> {
        vec![self.res, self.ind]
    }

    fn units(&self) -> Vec<Unit> {
        vec![self.res_unit, self.ind_unit]
    }

    fn tols(&self) -> Vec<f64> {
        vec![self.res_tol, self.ind_tol]
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
                    freq.w() * unscale(self.ind, &self.ind_unit) / self.res
                }
            }
            _ => unscale(self.res, &self.res_unit),
        };
        c64(res, freq.w() * unscale(self.ind, &self.ind_unit))
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
    fn test_inductor_series() {
        let testname = "inductor_series";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 20.0;
        let ind = 10.0;
        let res_unit = Unit::Q;
        let ind_unit = Unit::Pico;
        let orientation = Orientation::Series;
        let zout = c64(0.8796459430051421, 17.59291886010284);
        let zout_norm = zout / z0;
        let element = Inductor::new(
            20.0,
            10.0,
            Unit::Q,
            Unit::Pico,
            0.0,
            0.0,
            Orientation::Series,
        );

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        comp_f64(&element.ind(), &ind, margin, testname, "ind()");
        comp_f64(
            &unscale(element.ind(), &element.ind_unit()),
            &(ind * 1e-12),
            margin,
            testname,
            "ind_unscale()",
        );
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.ind_unit(), &ind_unit);
        assert_eq!(&element.orientation(), &orientation);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = 0.03797835657688831;
        let end_y_coord = 0.16777188853307162;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.017592918860103;
        let y2 = 0.3518583772020568;
        let x_coord = vec![
            0.0,
            0.0011874729479519443,
            0.002986216608965362,
            0.0053880153308550885,
            0.008382532099225147,
            0.011957406185492238,
            0.016098367155771235,
            0.020789363356480274,
            0.026012702811065425,
            0.03174920433226449,
            0.03797835657688831,
        ];
        let y_coord = vec![
            0.0,
            0.017556584166845562,
            0.035019156269724995,
            0.05235621902374438,
            0.06953711046372502,
            0.0865321775220376,
            0.10331293672381298,
            0.11985222030646443,
            0.13612430637097744,
            0.15210503199099842,
            0.16777188853307162,
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
    fn test_inductor_shunt() {
        let testname = "inductor_shunt";
        let freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        // let zl = c64(1.0, 0.0);
        // let zl_denorm = zl * z0;
        let res = 20.0;
        let ind = 10.0;
        let res_unit = Unit::Q;
        let ind_unit = Unit::Pico;
        let orientation = Orientation::Shunt;
        let zout = c64(0.8796459430051421, 17.59291886010284);
        let zout_norm = zout / z0;
        let element = Inductor::new(
            20.0,
            10.0,
            Unit::Q,
            Unit::Pico,
            0.0,
            0.0,
            Orientation::Shunt,
        );

        let margin = F64Margin::default();

        comp_f64(&element.res(), &res, margin, testname, "res()");
        comp_f64(&element.ind(), &ind, margin, testname, "ind()");
        comp_f64(
            &unscale(element.ind(), &element.ind_unit()),
            &(ind * 1e-12),
            margin,
            testname,
            "ind_unscale()",
        );
        assert_eq!(&element.res_unit(), &res_unit);
        assert_eq!(&element.ind_unit(), &ind_unit);
        assert_eq!(&element.orientation(), &orientation);
        comp_c64(&element.z(freq), &zout, margin, testname, "z");
        comp_c64(
            &element.z_norm(freq, z0),
            &zout_norm,
            margin,
            testname,
            "z_norm",
        );

        let end_x_coord = -0.6606893070885922;
        let end_y_coord = 0.44913494554545125;
        // let imag_old = 0.0;
        // let real_old = 0.0;
        // let start_x_coord = 0.0;
        // let start_y_coord = 0.0;
        let x1 = 1.0;
        let x2 = 1.1417482571178263;
        let x_coord = vec![
            0.0,
            -0.026326741419400255,
            -0.0854398444401545,
            -0.16550566724301574,
            -0.2542701170487083,
            -0.3423475317367525,
            -0.4240800012974579,
            -0.49688364532064383,
            -0.5601737023676274,
            -0.6144499256284348,
            -0.6606893070885922,
        ];
        let y1 = 0.0;
        let y2 = -2.8349651423565256;
        let y_coord = vec![
            0.0,
            0.13704519155357964,
            0.25565081047842475,
            0.3474762292461895,
            0.4111671980155597,
            0.45015334288049136,
            0.46983447110827475,
            0.47561491021893504,
            0.4719950960958993,
            0.46236661203209817,
            0.44913494554545125,
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
