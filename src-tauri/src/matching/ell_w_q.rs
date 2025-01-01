#![allow(unused)]
use crate::matching::CLQ;
use crate::rf_utils::{calc_gamma, calc_rc, calc_z, scale, unscale, Complex2Return, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

// ---CAP---------
//          |
//         RES
//          |
//         IND
//          |
//         GND
pub fn calc_hp_ell_cl_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = zs.im / zs.re;
        let rp = (1.0 + qs.powi(2)) * zl.re;
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / rs - 1.0).sqrt();
        let xp = -((q
            * (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            - q * xl.powi(2)
            + 2.0 * q.powi(2) * rs * xl
            + 2.0 * q * rl * rs
            - q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl));
        let xc = ((2.0 * q * rl - 2.0 * xl) * xs
            + (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            - xl.powi(2)
            - rl.powi(2))
            / (2.0 * xl - 2.0 * q * rl);

        l = xp / w;
        c = -1.0 / (w * xc);
        sol = 1;

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
    })
}

// --------CAP----
//     |
//    RES
//     |
//    IND
//     |
//    GND
pub fn calc_hp_ell_lc_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = zs.im / zs.re;
        let rp = (1.0 + qs.powi(2)) * zs.re;
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / rs - 1.0).sqrt();
        let xp = (q
            * (xs.powi(4) - 4.0 * q * rl * xs.powi(3)
                + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
                + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            - q * xs.powi(2)
            + 2.0 * q.powi(2) * rl * xs
            - q * rs.powi(2)
            + 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl);
        let xc = ((xs.powi(4) - 4.0 * q * rl * xs.powi(3)
            + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
            + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            - xs.powi(2)
            - 2.0 * xl * xs
            + 2.0 * q * rs * xl
            - rs.powi(2))
            / (2.0 * xs - 2.0 * q * rs);

        l = xp / w;
        c = -1.0 / (w * xc);
        sol = 1;

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
    })
}

// --------RES--IND----
//     |
//    CAP
//     |
//    GND
pub fn calc_lp_ell_cl_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = -zs.im / zs.re;
        let rp = zs.re * (1.0 + qs.powi(2));
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / zl.re - 1.0).sqrt();
        let xp = (q
            * (xs.powi(4)
                + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs) * xs.powi(2)
                - 4.0 * rs.powi(2) * xl.powi(2)
                + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            - q * xs.powi(2)
            - 2.0 * q.powi(2) * rs * xl
            - q * rs.powi(2)
            + 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs);
        let xc = ((xs.powi(4)
            + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs) * xs.powi(2)
            - 4.0 * rs.powi(2) * xl.powi(2)
            + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            - xs.powi(2)
            + (-(2.0 * xl) - 2.0 * q * rl) * xs
            - rs.powi(2))
            / (2.0 * xs + 2.0 * xl - 2.0 * q * rs + 2.0 * q * rl);

        l = xp / w;
        c = -1.0 / (w * xc);
        sol = 1;

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
    })
}

// ---RES--IND---------
//          |
//         CAP
//          |
//         GND
pub fn calc_lp_ell_lc_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = -zl.im / zl.re;
        let rp = zl.re * (1.0 + qs.powi(2));
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / zs.re - 1.0).sqrt();

        let xp = -((q
            * (-(4.0 * rl.powi(2) * xs.powi(2))
                + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3))
                    * xs
                + xl.powi(4)
                + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            + 2.0 * q.powi(2) * rl * xs
            + q * xl.powi(2)
            - 2.0 * q * rl * rs
            + q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rl));
        let xc = -(((-(4.0 * rl.powi(2) * xs.powi(2))
            + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3))
                * xs
            + xl.powi(4)
            + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
            + 4.0 * q.powi(2) * rl.powi(3) * rs
            + rl.powi(4))
        .sqrt()
            + 2.0 * xl * xs
            + xl.powi(2)
            + 2.0 * q * rs * xl
            + rl.powi(2))
            / (2.0 * xs + 2.0 * xl + 2.0 * q * rs - 2.0 * q * rl));

        l = xp / w;
        c = -1.0 / (w * xc);
        sol = 1;

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
    })
}
