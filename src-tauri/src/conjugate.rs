#![allow(unused)]
use crate::rf_utils::{calc_rc, calc_z, gen_complex, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::str::FromStr;

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct ImpedanceReturn {
    pub gamma: ComplexReturn,
    pub z: ComplexReturn,
    pub r: f64,
    pub c: f64,
    pub z0: f64,
    pub freq: f64,
    pub freq_unit: String,
    pub res_unit: String,
    pub cap_unit: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct ResultsReturn {
    pub k: f64,
    pub b1: f64,
    pub b2: f64,
    pub mag: f64,
    pub src: ImpedanceReturn,
    pub load: ImpedanceReturn,
}

#[tauri::command]
pub fn calc_match(
    s11re: f64,
    s11im: f64,
    s12re: f64,
    s12im: f64,
    s21re: f64,
    s21im: f64,
    s22re: f64,
    s22im: f64,
    imp: &str,
    z0: f64,
    freq: f64,
    fscale: &str,
    cscale: &str,
) -> Result<ResultsReturn, String> {
    let freq_unit = Unit::from_str(fscale).unwrap();
    let cap_unit = Unit::from_str(cscale).unwrap();

    let s11 = gen_complex(s11re, s11im, imp)?;
    let s12 = gen_complex(s12re, s12im, imp)?;
    let s21 = gen_complex(s21re, s21im, imp)?;
    let s22 = gen_complex(s22re, s22im, imp)?;

    let ds = s11 * s22 - s12 * s21;

    let k: f64 = (1.0 + ds.norm().powi(2) - s11.norm().powi(2) - s22.norm().powi(2))
        / (2.0 * s12.norm() * s21.norm());

    let b1: f64 = 1.0 + s11.norm().powi(2) - s22.norm().powi(2) - ds.norm().powi(2);

    let mag: f64 = 10.0 * (s21.norm() / s12.norm()).log10()
        + 10.0 * (k - b1.signum() * (k.powi(2) - 1.0).sqrt()).abs().log10();

    let b2: f64 = 1.0 + s22.norm().powi(2) - s11.norm().powi(2) - ds.norm().powi(2);

    let c2 = s22 - ds * s11.conj();

    let gamma_load_mag =
        (b2 - b2.signum() * (b2.powi(2) - 4.0 * c2.norm().powi(2)).sqrt()) / (2.0 * c2.norm());
    let gamma_load_ang = -1.0 * c2.arg();

    let gamma_load = Complex::from_polar(gamma_load_mag, gamma_load_ang);
    let z_load = calc_z(gamma_load, z0);
    let (rl, cl) = calc_rc(gamma_load, freq, &freq_unit, &Unit::Base, &cap_unit);

    let gamma_src = (s11 + (s12 * s21 * gamma_load / (1.0 - gamma_load * s22))).conj();
    let z_src = calc_z(gamma_src, z0);
    let (rs, cs) = calc_rc(gamma_src, freq, &freq_unit, &Unit::Base, &cap_unit);

    Ok(ResultsReturn {
        k: k,
        b1: b1,
        b2: b2,
        mag: mag,
        src: ImpedanceReturn {
            gamma: ComplexReturn {
                re: gamma_src.re,
                im: gamma_src.im,
            },
            z: ComplexReturn {
                re: z_src.re,
                im: z_src.im,
            },
            r: rs,
            c: cs,
            z0: z0,
            freq: freq,
            freq_unit: get_unit(&freq_unit, &UnitType::Hz),
            res_unit: get_unit(&Unit::Base, &UnitType::Ohm),
            cap_unit: get_unit(&cap_unit, &UnitType::Farad),
        },
        load: ImpedanceReturn {
            gamma: ComplexReturn {
                re: gamma_load.re,
                im: gamma_load.im,
            },
            z: ComplexReturn {
                re: z_load.re,
                im: z_load.im,
            },
            r: rl,
            c: cl,
            z0: z0,
            freq: freq,
            freq_unit: get_unit(&freq_unit, &UnitType::Hz),
            res_unit: get_unit(&Unit::Base, &UnitType::Ohm),
            cap_unit: get_unit(&cap_unit, &UnitType::Farad),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::comp_f64;

    #[test]
    fn test_calc_match() {
        let s11 = Complex::new(0.34, 0.21);
        let s12 = Complex::new(0.0434, -0.0052);
        let s21 = Complex::new(0.32, -3.4);
        let s22 = Complex::new(0.34, -0.52);
        let imp = "ri";
        let z0 = 100.0;
        let freq = 275.0;
        let fscale = "giga";
        let cscale = "femto";
        let exemplar = ResultsReturn {
            k: 1.7031802961437423,
            b1: 0.7195251545599999,
            b2: 1.1721251545600002,
            mag: 14.039928315508192,
            src: ImpedanceReturn {
                gamma: ComplexReturn {
                    re: 0.5040400052246673,
                    im: -0.13478919243703535,
                },
                z: ComplexReturn {
                    re: 275.52180881729475,
                    im: -102.05718583392367,
                },
                r: 0.5400850139729908,
                c: 286.5598722530983,
                z0: 100.0,
                freq: 275.0,
                freq_unit: "GHz".to_string(),
                res_unit: "Ω".to_string(),
                cap_unit: "fF".to_string(),
            },
            load: ImpedanceReturn {
                gamma: ComplexReturn {
                    re: 0.31959462490960494,
                    im: 0.6148725683749898,
                },
                z: ComplexReturn {
                    re: 61.804850661047205,
                    im: 146.22072038786013,
                },
                r: 1.502556558161738,
                c: -741.0410407114609,
                z0: 100.0,
                freq: 275.0,
                freq_unit: "GHz".to_string(),
                res_unit: "Ω".to_string(),
                cap_unit: "fF".to_string(),
            },
        };
        let test = calc_match(
            s11.re, s11.im, s12.re, s12.im, s21.re, s21.im, s22.re, s22.im, imp, z0, freq, fscale,
            cscale,
        )
        .unwrap();

        comp_f64(
            &test.k,
            &exemplar.k,
            F64Margin::default(),
            "calc_match()",
            "k",
        );
        comp_f64(
            &test.b1,
            &exemplar.b1,
            F64Margin::default(),
            "calc_match()",
            "b1",
        );
        comp_f64(
            &test.b2,
            &exemplar.b2,
            F64Margin::default(),
            "calc_match()",
            "b2",
        );
        comp_f64(
            &test.mag,
            &exemplar.mag,
            F64Margin::default(),
            "calc_match()",
            "mag",
        );
        comp_f64(
            &test.src.gamma.re,
            &exemplar.src.gamma.re,
            F64Margin::default(),
            "calc_match()",
            "src.gamma.re",
        );
        comp_f64(
            &test.src.gamma.im,
            &exemplar.src.gamma.im,
            F64Margin::default(),
            "calc_match()",
            "src.gamma.im",
        );
        comp_f64(
            &test.src.z.re,
            &exemplar.src.z.re,
            F64Margin::default(),
            "calc_match()",
            "src.z.re",
        );
        comp_f64(
            &test.src.z.im,
            &exemplar.src.z.im,
            F64Margin::default(),
            "calc_match()",
            "src.z.im",
        );
        comp_f64(
            &test.src.r,
            &exemplar.src.r,
            F64Margin::default(),
            "calc_match()",
            "src.r",
        );
        comp_f64(
            &test.src.c,
            &exemplar.src.c,
            F64Margin::default(),
            "calc_match()",
            "src.c",
        );
        comp_f64(
            &test.src.z0,
            &exemplar.src.z0,
            F64Margin::default(),
            "calc_match()",
            "src.z0",
        );
        comp_f64(
            &test.src.freq,
            &exemplar.src.freq,
            F64Margin::default(),
            "calc_match()",
            "src.freq",
        );
        assert_eq!(test.src.freq_unit, exemplar.src.freq_unit);
        assert_eq!(test.src.res_unit, exemplar.src.res_unit);
        assert_eq!(test.src.cap_unit, exemplar.src.cap_unit);
        comp_f64(
            &test.load.gamma.re,
            &exemplar.load.gamma.re,
            F64Margin::default(),
            "calc_match()",
            "load.gamma.re",
        );
        comp_f64(
            &test.load.gamma.im,
            &exemplar.load.gamma.im,
            F64Margin::default(),
            "calc_match()",
            "load.gamma.im",
        );
        comp_f64(
            &test.load.z.re,
            &exemplar.load.z.re,
            F64Margin::default(),
            "calc_match()",
            "load.z.re",
        );
        comp_f64(
            &test.load.z.im,
            &exemplar.load.z.im,
            F64Margin::default(),
            "calc_match()",
            "load.z.im",
        );
        comp_f64(
            &test.load.r,
            &exemplar.load.r,
            F64Margin::default(),
            "calc_match()",
            "load.r",
        );
        comp_f64(
            &test.load.c,
            &exemplar.load.c,
            F64Margin::default(),
            "calc_match()",
            "load.c",
        );
        comp_f64(
            &test.load.z0,
            &exemplar.load.z0,
            F64Margin::default(),
            "calc_match()",
            "load.z0",
        );
        comp_f64(
            &test.load.freq,
            &exemplar.load.freq,
            F64Margin::default(),
            "calc_match()",
            "load.freq",
        );
        assert_eq!(test.load.freq_unit, exemplar.load.freq_unit);
        assert_eq!(test.load.res_unit, exemplar.load.res_unit);
        assert_eq!(test.load.cap_unit, exemplar.load.cap_unit);
    }
}
