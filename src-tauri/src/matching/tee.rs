#![allow(unused)]
use crate::matching::PiTee;
use crate::rf_utils::{calc_gamma, calc_rc, calc_z, scale, unscale, Complex2Return, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

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
    fn test_calc_tee() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
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
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_tee()",
            "c",
        );
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_tee()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_tee()",
            "ll",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_tee()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 1.99;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
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
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
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
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
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
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_tee()",
            "c",
        );
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_tee()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_tee()",
            "ll",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_tee()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, -183.168);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
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
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(
            &test.c,
            &exemplar.c,
            F64Margin::default(),
            "calc_tee()",
            "c",
        );
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_tee()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_tee()",
            "ll",
        );
        comp_f64(
            &test.q,
            &exemplar.q,
            F64Margin::default(),
            "calc_tee()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }
}
