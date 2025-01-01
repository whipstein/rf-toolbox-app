#![allow(unused)]
use crate::matching::CCLL;
use crate::rf_utils::{calc_gamma, calc_rc, calc_z, scale, unscale, Complex2Return, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
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
        c_scale: get_unit(c_scale, &UnitType::Farad),
        l_scale: get_unit(l_scale, &UnitType::Henry),
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
    fn test_calc_bp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 6.772022183008002,
            ls: 137.66998607438342,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.cs,
            &exemplar.cs,
            F64Margin::default(),
            "calc_bp1()",
            "cs",
        );
        comp_f64(
            &test.cl,
            &exemplar.cl,
            F64Margin::default(),
            "calc_bp1()",
            "cl",
        );
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_bp1()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_bp1()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 6.772022183008002,
            ls: 137.66998607438342,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.cs,
            &exemplar.cs,
            F64Margin::default(),
            "calc_bp2()",
            "cs",
        );
        comp_f64(
            &test.cl,
            &exemplar.cl,
            F64Margin::default(),
            "calc_bp2()",
            "cl",
        );
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_bp2()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_bp2()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp2(zs, zl, w, &c_scale, &l_scale).unwrap();
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
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp3(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 20.352712959723295,
            ls: 63.48256505664435,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp3(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.cs,
            &exemplar.cs,
            F64Margin::default(),
            "calc_bp3()",
            "cs",
        );
        comp_f64(
            &test.cl,
            &exemplar.cl,
            F64Margin::default(),
            "calc_bp3()",
            "cl",
        );
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_bp3()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_bp3()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp4() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 20.352712959723295,
            ls: 63.48256505664435,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp4(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.cs,
            &exemplar.cs,
            F64Margin::default(),
            "calc_bp4()",
            "cs",
        );
        comp_f64(
            &test.cl,
            &exemplar.cl,
            F64Margin::default(),
            "calc_bp4()",
            "cl",
        );
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_bp4()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_bp4()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp4(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }
}
