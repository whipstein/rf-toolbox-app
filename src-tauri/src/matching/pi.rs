#![allow(unused)]
use crate::matching::PiTee;
use crate::rf_utils::{calc_gamma, calc_rc, calc_z, scale, unscale, Complex2Return, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

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
    fn test_calc_pi() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
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
        let test = calc_pi(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        comp_f64(
            &test.cs,
            &exemplar.cs,
            F64Margin::default(),
            "calc_pi()",
            "cs",
        );
        comp_f64(
            &test.cl,
            &exemplar.cl,
            F64Margin::default(),
            "calc_pi()",
            "cl",
        );
        comp_f64(&test.l, &exemplar.l, F64Margin::default(), "calc_pi()", "l");
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        comp_f64(&test.q, &exemplar.q, F64Margin::default(), "calc_pi()", "q");
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
        let test = calc_pi(zs, zl, w, q, &c_scale, &l_scale).unwrap();
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
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
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
        let test = calc_pi(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(&test.c, &exemplar.c, F64Margin::default(), "calc_pi()", "c");
        comp_f64(
            &test.cs,
            &exemplar.cs,
            F64Margin::default(),
            "calc_pi()",
            "cs",
        );
        comp_f64(
            &test.cl,
            &exemplar.cl,
            F64Margin::default(),
            "calc_pi()",
            "cl",
        );
        comp_f64(&test.l, &exemplar.l, F64Margin::default(), "calc_pi()", "l");
        comp_f64(
            &test.ls,
            &exemplar.ls,
            F64Margin::default(),
            "calc_pi()",
            "ls",
        );
        comp_f64(
            &test.ll,
            &exemplar.ll,
            F64Margin::default(),
            "calc_pi()",
            "ll",
        );
        comp_f64(&test.q, &exemplar.q, F64Margin::default(), "calc_pi()", "q");
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }
}
