use crate::rf_utils::{get_unit, scale, Element, Unit};
use num_complex::Complex;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct CCLL {
    cs: f64,
    cl: f64,
    ls: f64,
    ll: f64,
    c_scale: String,
    l_scale: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct PiTee {
    c: f64,
    cs: f64,
    cl: f64,
    l: f64,
    ls: f64,
    ll: f64,
    q: f64,
    c_scale: String,
    l_scale: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct CL {
    c: f64,
    l: f64,
    q: f64,
    c_scale: String,
    l_scale: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct CLQ {
    c: f64,
    l: f64,
    q: f64,
    q_net: f64,
    sol: usize,
    c_scale: String,
    l_scale: String,
}

// ---CAP---------
//          |
//         IND
//          |
//         GND
pub fn calc_hp_ell_cl(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = zl.im / zl.re;
        let c1 = -1.0 / (w * zs.im);
        let l1 = (1.0 + qs.powi(2)) * zl.im / (w * qs.powi(2));
        let rp = (1.0 + qs.powi(2)) * zl.re;

        if zs.re > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / zs.re - 1.0).sqrt();
            l = rp / (w * q);
            c = 1.0 / (q * w * zs.re);

            if zs.im != 0.0 {
                if c1 == c {
                    c = INFINITY;
                } else {
                    c *= c1 / (c1 - c);
                }
            }

            if zl.im != 0.0 {
                if l1 == l {
                    l = INFINITY;
                } else {
                    l *= l1 / (l1 - l);
                }
            }

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

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
        let xp1 = (q
            * (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            + q * xl.powi(2)
            - 2.0 * q.powi(2) * rs * xl
            - 2.0 * q * rl * rs
            + q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl);
        let xc1 = -(((2.0 * xl - 2.0 * q * rl) * xs
            + (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            + xl.powi(2)
            + rl.powi(2))
            / (2.0 * xl - 2.0 * q * rl));
        let xp2 = -((q
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
        let xc2 = ((2.0 * q * rl - 2.0 * xl) * xs
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

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

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
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------CAP----
//     |
//    IND
//     |
//    GND
pub fn calc_hp_ell_lc(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = zs.im / zs.re;
        let c1 = -1.0 / (w * zl.im);
        let l1 = (1.0 + qs.powi(2)) * zs.im / (w * qs.powi(2));
        let rp = (1.0 + qs.powi(2)) * zs.re;
        let rs = zl.re + 0.0;

        if rs > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / rs - 1.0).sqrt();
            l = rp / (w * q);
            c = 1.0 / (q * w * rs);

            if zl.im != 0.0 {
                if c1 == c {
                    c = INFINITY;
                } else {
                    c *= c1 / (c1 - c);
                }
            }

            if zs.im != 0.0 {
                if l1 == l {
                    l = INFINITY;
                } else {
                    l *= l1 / (l1 - l);
                }
            }

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
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
        let xp1 = -((q
            * (xs.powi(4) - 4.0 * q * rl * xs.powi(3)
                + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
                + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            + q * xs.powi(2)
            - 2.0 * q.powi(2) * rl * xs
            + q * rs.powi(2)
            - 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl));
        let xc1 = -(((xs.powi(4) - 4.0 * q * rl * xs.powi(3)
            + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
            + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            + xs.powi(2)
            + 2.0 * xl * xs
            - 2.0 * q * rs * xl
            + rs.powi(2))
            / (2.0 * xs - 2.0 * q * rs));
        let xp2 = (q
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
        let xc2 = ((xs.powi(4) - 4.0 * q * rl * xs.powi(3)
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

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

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
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------IND----
//     |
//    CAP
//     |
//    GND
pub fn calc_lp_ell_cl(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = -zs.im / zs.re;
        let rp = zs.re * (1.0 + qs.powi(2));

        if zl.re > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / zl.re - 1.0).sqrt();
            let cp = q / (rp * w);
            let c1 = qs / (rp * w);
            c = cp - c1;
            let ls = q * zl.re / w;
            let l1 = zl.im / w;
            l = ls - l1;

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
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
        let xp1 = -((q
            * (xs.powi(4)
                + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs)
                    * xs.powi(2)
                - 4.0 * rs.powi(2) * xl.powi(2)
                + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            + q * xs.powi(2)
            + 2.0 * q.powi(2) * rs * xl
            + q * rs.powi(2)
            - 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs));
        let xc1 = -(((xs.powi(4)
            + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs) * xs.powi(2)
            - 4.0 * rs.powi(2) * xl.powi(2)
            + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            + xs.powi(2)
            + (2.0 * xl + 2.0 * q * rl) * xs
            + rs.powi(2))
            / (2.0 * xs + 2.0 * xl - 2.0 * q * rs + 2.0 * q * rl));
        let xp2 = (q
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
        let xc2 = ((xs.powi(4)
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

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

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
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---IND---------
//          |
//         CAP
//          |
//         GND
pub fn calc_lp_ell_lc(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = -zl.im / zl.re;
        let rp = zl.re * (1.0 + qs.powi(2));

        if zs.re > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / zs.re - 1.0).sqrt();
            let cp = q / (rp * w);
            let c1 = qs / (rp * w);
            c = cp - c1;
            let ls = q * zs.re / w;
            let l1 = zs.im / w;
            l = ls - l1;

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
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

        let xp1 = -((q
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
        let xc1 = -(((-(4.0 * rl.powi(2) * xs.powi(2))
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
        let xp2 = (q
            * (-(4.0 * rl.powi(2) * xs.powi(2))
                + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3))
                    * xs
                + xl.powi(4)
                + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            - 2.0 * q.powi(2) * rl * xs
            - q * xl.powi(2)
            + 2.0 * q * rl * rs
            - q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rl);
        let xc2 = ((-(4.0 * rl.powi(2) * xs.powi(2))
            + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3)) * xs
            + xl.powi(4)
            + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
            + 4.0 * q.powi(2) * rl.powi(3) * rs
            + rl.powi(4))
        .sqrt()
            - 2.0 * xl * xs
            - xl.powi(2)
            - 2.0 * q * rs * xl
            - rl.powi(2))
            / (2.0 * xs + 2.0 * xl + 2.0 * q * rs - 2.0 * q * rl);

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

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
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_tee(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    q_tgt: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<PiTee, String> {
    let mut c: f64;
    let mut cs: f64;
    let mut cl: f64;
    let mut l: f64;
    let mut ls: f64;
    let mut ll: f64;
    let mut q = q_tgt;

    if q_tgt < 0.0 {
        c = NAN;
        cs = NAN;
        cl = NAN;
        l = NAN;
        ls = NAN;
        ll = NAN;
        q = NAN;
    } else {
        if (q_tgt == 0.0) && (zs.re == zl.re) {
            cs = 0.0;
            cl = 0.0;
            l = 0.0;
            ls = 0.0;
            ll = 0.0;
            c = 0.0;
            q = 0.0;
        } else {
            if q_tgt < (zs.re.max(zl.re) / zs.re.min(zl.re) - 1.0).sqrt() {
                c = NAN;
                cs = NAN;
                cl = NAN;
                l = NAN;
                ls = NAN;
                ll = NAN;
                q = NAN;
            } else {
                let rv = zs.re.min(zl.re) * (q_tgt.powi(2) + 1.0);

                let mut qx = (rv / zs.re - 1.0).sqrt();
                cs = 1.0 / (w * zs.re * qx);
                if zs.im != 0.0 {
                    if cs == -1.0 / (w * zs.im) {
                        cs = INFINITY;
                    } else {
                        cs *= -1.0 / (w * zs.im) / (cs + 1.0 / (w * zs.im));
                    }
                }

                let l5 = rv / (w * qx);
                qx = (rv / zl.re - 1.0).sqrt();
                cl = 1.0 / (w * zl.re * qx);
                if zl.im != 0.0 {
                    if cl == -1.0 / (w * zs.im) {
                        cl = INFINITY;
                    } else {
                        cl *= -1.0 / (w * zs.im) / (cl + 1.0 / (w * zs.im));
                    }
                }

                let l1 = rv / (w * qx);
                l = l1 * l5 / (l1 + l5);

                qx = (rv / zs.re - 1.0).sqrt();
                ls = qx * zs.re / w - zs.im / w;
                let c5 = qx / (w * rv);
                qx = (rv / zl.re - 1.0).sqrt();
                ll = qx * zl.re / w - zl.im / w;
                c = c5 + (qx / (w * rv));

                c = scale(c, c_scale);
                cs = scale(cs, c_scale);
                cl = scale(cl, c_scale);
                l = scale(l, l_scale);
                ls = scale(ls, l_scale);
                ll = scale(ll, l_scale);

                if (c < 0.0) || (ls < 0.0) || (ll < 0.0) {
                    c = NAN;
                    ls = NAN;
                    ll = NAN;
                }
                if (l < 0.0) || (cs < 0.0) || (cl < 0.0) || (cs == INFINITY) || (cl == INFINITY) {
                    l = NAN;
                    cs = NAN;
                    cl = NAN;
                }
            }
        }
    }

    Ok(PiTee {
        c: c,
        cs: cs,
        cl: cl,
        l: l,
        ls: ls,
        ll: ll,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_pi(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    q_tgt: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<PiTee, String> {
    let mut c: f64;
    let mut cs: f64;
    let mut cl: f64;
    let mut l: f64;
    let mut ls: f64;
    let mut ll: f64;
    let mut q = q_tgt;

    if q_tgt < 0.0 {
        c = NAN;
        cs = NAN;
        cl = NAN;
        l = NAN;
        ls = NAN;
        ll = NAN;
        q = NAN;
    } else {
        if (q_tgt == 0.0) && (zs.re == zl.re) {
            cs = 0.0;
            cl = 0.0;
            l = 0.0;
            ls = 0.0;
            ll = 0.0;
            c = 0.0;
            q = 0.0;
        } else {
            if q_tgt < (zs.re.max(zl.re) / zs.re.min(zl.re) - 1.0).sqrt() {
                c = NAN;
                cs = NAN;
                cl = NAN;
                l = NAN;
                ls = NAN;
                ll = NAN;
                q = NAN;
            } else {
                let rv = zs.re.max(zl.re) / (q_tgt.powi(2) + 1.0);

                let qs = -zs.im / zs.re;
                let ql = -zl.im / zl.re;
                let rps = zs.re * (1.0 + qs.powi(2));
                let rpl = zl.re * (1.0 + ql.powi(2));

                //cs-l-cl pi network matching
                let cps = qs / (rps * w);
                let cpl = ql / (rpl * w);
                let mut qx = (rps / rv - 1.0).sqrt();
                cs = qx / (w * rps) - cps;
                let l5 = qx * rv / w;
                qx = (rpl / rv - 1.0).sqrt();
                cl = qx / (w * rpl) - cpl;
                l = l5 + (qx * rv / w);

                //ls-c-ll pi network matching
                qx = (rps / rv - 1.0).sqrt();
                ls = rps / (w * qx);
                if qs != 0.0 {
                    let lps = rps / (qs * w);
                    ls *= lps / (ls - lps);
                }
                let c5 = 1.0 / (w * qx * rv);
                qx = (rpl / rv - 1.0).sqrt();
                ll = rpl / (w * qx);
                if ql != 0.0 {
                    let lpl = rpl / (ql * w);
                    ll *= lpl / (ll - lpl);
                }
                let c1 = 1.0 / (w * qx * rv);
                c = c1 * c5 / (c1 + c5);

                c = scale(c, c_scale);
                cs = scale(cs, c_scale);
                cl = scale(cl, c_scale);
                l = scale(l, l_scale);
                ls = scale(ls, l_scale);
                ll = scale(ll, l_scale);

                if (c < 0.0) || (ls < 0.0) || (ll < 0.0) {
                    c = NAN;
                    ls = NAN;
                    ll = NAN;
                }
                if (l < 0.0) || (cs < 0.0) || (cl < 0.0) {
                    l = NAN;
                    cs = NAN;
                    cl = NAN;
                }
            }
        }
    }

    Ok(PiTee {
        c: c,
        cs: cs,
        cl: cl,
        l: l,
        ls: ls,
        ll: ll,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_lp1(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();

    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = zl.re * ql / w - zl.im / w;
        cl = ql / (w * rv);
        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_lp2(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = zs.re * ql / w - zs.im / w;
        cl = ql / (w * rv);

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_hp1(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();
    if rp <= rv {
        ls = NAN;
        ll = NAN;
        cs = NAN;
        cl = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zs.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        let c5 = -1.0 / (w * zl.im);
        ll = rv / (w * ql);
        cl = 1.0 / (w * zl.re * ql);
        if zl.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_hp2(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zl.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        let c5 = -1.0 / (w * zs.im);
        ll = rv / (w * ql);
        cl = 1.0 / (w * zs.re * ql);
        if zs.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------CAP-------IND--
//     |         |
//    IND       CAP
//     |         |
//    GND       GND
pub fn calc_bp1(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zs.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        ll = ql * zl.re / w - zl.im / w;
        cl = ql / (w * rv);

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---IND-------CAP-------
//          |         |
//         CAP       IND
//          |         |
//         GND       GND
pub fn calc_bp2(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zl.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        ll = ql * zs.re / w - zs.im / w;
        cl = ql / (w * rv);

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------IND-------CAP--
//     |         |
//    CAP       IND
//     |         |
//    GND       GND
pub fn calc_bp3(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = rv / (w * ql);
        let c5 = -1.0 / (w * zl.im);
        cl = 1.0 / (w * zl.re * ql);
        if zl.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---CAP-------IND-------
//          |         |
//         IND       CAP
//          |         |
//         GND       GND
pub fn calc_bp4(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = rv / (w * ql);
        let c5 = -1.0 / (w * zs.im);
        cl = 1.0 / (w * zs.re * ql);
        if zs.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_calc_hp_ell_cl() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: 8.58125245724517,
            l: 69.18681390709257,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(
            calc_hp_ell_cl(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(62.4, -14.6);
        let zl = Complex::new(202.3, 23.2);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = "pico";
        let l_scale = "nano";
        let exemplar = CL {
            c: 11.408503434826747,
            l: 133.4483264614267,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        assert_eq!(
            calc_hp_ell_cl(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 175.0e9;
        let c_scale = "pico";
        let l_scale = "nano";
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_hp_ell_lc() {
        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: 8.58125245724517,
            l: 69.18681390709257,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(
            calc_hp_ell_lc(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(202.3, 23.2);
        let zl = Complex::new(62.4, -14.6);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = "pico";
        let l_scale = "nano";
        let exemplar = CL {
            c: 11.408503434826747,
            l: 133.4483264614267,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        assert_eq!(
            calc_hp_ell_lc(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_lp_ell_cl() {
        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: 5.906505625073422,
            l: 61.719118523742445,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(
            calc_lp_ell_cl(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(202.3, 23.2);
        let zl = Complex::new(62.4, -14.6);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = "pico";
        let l_scale = "nano";
        let exemplar = CL {
            c: 7.2157251698188345,
            l: 99.0557187033109,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        assert_eq!(
            calc_lp_ell_cl(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_lp_ell_lc() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: 5.906505625073422,
            l: 61.719118523742445,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(
            calc_lp_ell_lc(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(62.4, -14.6);
        let zl = Complex::new(202.3, 23.2);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = "pico";
        let l_scale = "nano";
        let exemplar = CL {
            c: 7.2157251698188345,
            l: 99.0557187033109,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        assert_eq!(
            calc_lp_ell_lc(zs, zl, w, c_scale, l_scale).unwrap(),
            exemplar
        );

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_tee() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: 4.186603177852454,
            cs: -3.5382547173462546,
            cl: -1.6843173326757916,
            l: 80.00425342422247,
            ls: 117.35101636675431,
            ll: 185.20322518485523,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, c_scale, l_scale).unwrap();
        assert_eq!(test.c, exemplar.c);
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert_eq!(test.ls, exemplar.ls);
        assert_eq!(test.ll, exemplar.ll);
        assert_eq!(test.q, exemplar.q);
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 1.99;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: NAN,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: NAN,
            ll: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, -363.20820041403255);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: 4.186603177852454,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: 420.4100397629459,
            ll: 117.35101636675431,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, c_scale, l_scale).unwrap();
        assert_eq!(test.c, exemplar.c);
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert_eq!(test.ls, exemplar.ls);
        assert_eq!(test.ll, exemplar.ll);
        assert_eq!(test.q, exemplar.q);
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, -183.168);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: 4.186603177852454,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: 316.2126293951322,
            ll: 117.35101636675431,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, c_scale, l_scale).unwrap();
        assert_eq!(test.c, exemplar.c);
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert_eq!(test.ls, exemplar.ls);
        assert_eq!(test.ll, exemplar.ll);
        assert_eq!(test.q, exemplar.q);
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_pi() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: 8.435997609374349,
            cs: 16.62637373190316,
            cl: 12.08508737222243,
            l: 39.704380813877926,
            ls: -20.145466896660622,
            ll: -27.71565081088584,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_pi(zs, zl, w, q, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert_eq!(test.cs, exemplar.cs);
        assert_eq!(test.cl, exemplar.cl);
        assert_eq!(test.l, exemplar.l);
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.q, exemplar.q);
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 1.99;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: NAN,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: NAN,
            ll: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_pi(zs, zl, w, q, c_scale, l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(42.4, 0.0);
        let zl = Complex::new(212.3, 0.0);
        let w = 2.0 * PI * 275.0e9;
        let q = 3.88;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = PiTee {
            c: 8.157016433395613,
            cs: 20.27486954435912,
            cl: 10.57716232084193,
            l: 41.06232522178837,
            ls: 16.520257301519933,
            ll: 31.666911357459586,
            q: 3.88,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_pi(zs, zl, w, q, c_scale, l_scale).unwrap(), exemplar);
    }

    #[test]
    fn test_calc_lp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp1(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 6.772022183008002,
            ls: 63.48256505664435,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_lp1(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);
    }

    #[test]
    fn test_calc_lp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 6.772022183008002,
            ls: 63.48256505664435,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_lp2(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp2(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_hp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp1(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 20.352712959723295,
            ls: 137.66998607438342,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_hp1(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);
    }

    #[test]
    fn test_calc_hp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 20.352712959723295,
            ls: 137.66998607438342,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_hp2(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp2(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp1(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 6.772022183008002,
            ls: 137.66998607438342,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_bp1(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);
    }

    #[test]
    fn test_calc_bp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 6.772022183008002,
            ls: 137.66998607438342,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_bp2(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp2(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp3() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp3(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 20.352712959723295,
            ls: 63.48256505664435,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_bp3(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);
    }

    #[test]
    fn test_calc_bp4() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 20.352712959723295,
            ls: 63.48256505664435,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        assert_eq!(calc_bp4(zs, zl, w, c_scale, l_scale).unwrap(), exemplar);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = "femto";
        let l_scale = "pico";
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp4(zs, zl, w, c_scale, l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }
}
