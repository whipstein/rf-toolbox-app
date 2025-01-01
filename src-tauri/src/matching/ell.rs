#![allow(unused)]
use crate::matching::CL;
use crate::rf_utils::{calc_gamma, calc_rc, calc_z, scale, unscale, Complex2Return, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::comp_f64;
    use std::f64::consts::PI;

    #[test]
    fn test_calc_hp_ell_cl() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 8.58125245724517,
            l: 69.18681390709257,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(62.4, -14.6);
        let zl = Complex::new(202.3, 23.2);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 11.408503434826747,
            l: 133.4483264614267,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 175.0e9;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
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
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 8.58125245724517,
            l: 69.18681390709257,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(202.3, 23.2);
        let zl = Complex::new(62.4, -14.6);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 11.408503434826747,
            l: 133.4483264614267,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
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
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 5.906505625073422,
            l: 61.719118523742445,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(202.3, 23.2);
        let zl = Complex::new(62.4, -14.6);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 7.2157251698188345,
            l: 99.0557187033109,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
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
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 5.906505625073422,
            l: 61.719118523742445,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(62.4, -14.6);
        let zl = Complex::new(202.3, 23.2);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 7.2157251698188345,
            l: 99.0557187033109,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "c",
        );
        comp_f64(
            &test.l,
            &exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "l",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }
}
