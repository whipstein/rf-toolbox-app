#![allow(unused)]
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::{approx_eq, F64Margin};
use num_complex::Complex;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::error::Error;
use std::f64::consts::PI;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use tauri::ipc::Response;

#[derive(Default, Debug, PartialEq)]
pub struct ComplexReturn {
    pub re: f64,
    pub im: f64,
}

impl ComplexReturn {
    fn norm(&self) -> f64 {
        Complex::<f64>::new(self.re, self.im).norm()
    }

    fn arg(&self) -> f64 {
        Complex::<f64>::new(self.re, self.im).arg()
    }
}

impl Serialize for ComplexReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ComplexReturn", 4)?;
        s.serialize_field("re", &self.re)?;
        s.serialize_field("im", &self.im)?;
        s.serialize_field("mag", &(self.norm()))?;
        s.serialize_field("ang", &(self.arg() * 180.0 / PI))?;
        s.end()
    }
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct Complex2Return {
    pub src: ComplexReturn,
    pub load: ComplexReturn,
}

#[derive(Serialize, Default)]
pub struct ResultsReturn {
    pub zre: f64,
    pub zim: f64,
    pub gre: f64,
    pub gim: f64,
    pub gmag: f64,
    pub gang: f64,
    pub r: f64,
    pub c: f64,
}

#[derive(Default)]
pub struct ResponseReturn {
    pub z: Complex<f64>,
    pub g: Complex<f64>,
    pub g_mag: f64,
    pub g_ang: f64,
    pub r: f64,
    pub c: f64,
}

impl Serialize for ResponseReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ResponseReturn", 8)?;
        s.serialize_field("z_re", &self.z.re)?;
        s.serialize_field("z_im", &self.z.im)?;
        s.serialize_field("g_re", &self.g.re)?;
        s.serialize_field("g_im", &self.g.im)?;
        s.serialize_field("g_mag", &self.g_mag)?;
        s.serialize_field("g_ang", &self.g_ang)?;
        s.serialize_field("r", &self.r)?;
        s.serialize_field("c", &self.c)?;
        s.end()
    }
}

pub enum ComplexType {
    ReIm,
    MagAng,
    Db,
}

impl FromStr for ComplexType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ri" | "reim" => Ok(ComplexType::ReIm),
            "ma" | "magang" => Ok(ComplexType::MagAng),
            "db" | "dbang" => Ok(ComplexType::Db),
            _ => Err("ComplexType not recognized".to_string().into()),
        }
    }
}

pub fn gen_complex(re: f64, im: f64, imp: &str) -> Result<Complex<f64>, String> {
    match ComplexType::from_str(imp) {
        Ok(ComplexType::ReIm) => Ok(Complex::new(re, im)),
        Ok(ComplexType::MagAng) => Ok(Complex::from_polar(re, im * PI / 180.0)),
        Ok(ComplexType::Db) => Ok(Complex::from_polar(10_f64.powf(re / 20.0), im * PI / 180.0)),
        Err(_) => Err("ComplexType not recognized".to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_c64_inv(re: f64, im: f64) -> Result<ComplexReturn, String> {
    let mut val = Complex::<f64>::new(re, im);
    val = val.inv();

    Ok(ComplexReturn {
        re: val.re,
        im: val.im,
    })
}

pub fn scale(val: f64, unit: &Unit) -> f64 {
    val * unit.scale()
}

pub fn unscale(val: f64, unit: &Unit) -> f64 {
    val * unit.unscale()
}

pub fn calc_gamma(z: Complex<f64>, z0: f64) -> Complex<f64> {
    let z0: f64 = z0;

    (z - z0) / (z + z0)
}

pub fn calc_gamma_from_rc(
    r: f64,
    c: f64,
    z0: f64,
    freq: f64,
    fscale: &Unit,
    _rscale: &Unit,
    cscale: &Unit,
) -> Complex<f64> {
    let z = 1.0
        / Complex::new(
            1.0 / r,
            2.0 * std::f64::consts::PI * unscale(freq, &fscale) * unscale(c, &cscale),
        );

    (z - z0) / (z + z0)
}

pub fn calc_z(gamma: Complex<f64>, z0: f64) -> Complex<f64> {
    z0 * (1.0 + gamma) / (1.0 - gamma)
}

pub fn calc_z_norm(gamma: Complex<f64>, z0: f64) -> Complex<f64> {
    calc_z(gamma, z0) / z0
}

pub fn calc_z_from_rc(
    r: f64,
    c: f64,
    freq: f64,
    fscale: &Unit,
    _rscale: &Unit,
    cscale: &Unit,
) -> Complex<f64> {
    1.0 / Complex::new(
        1.0 / r,
        2.0 * std::f64::consts::PI * unscale(freq, &fscale) * unscale(c, &cscale),
    )
}

pub fn calc_rc(
    z: Complex<f64>,
    freq: f64,
    fscale: &Unit,
    rscale: &Unit,
    cscale: &Unit,
) -> (f64, f64) {
    let y = 1.0 / z;

    (
        1.0 / scale(y.re, &rscale),
        scale(
            y.im / (2.0 * std::f64::consts::PI * unscale(freq, &fscale)),
            &cscale,
        ),
    )
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_impedance(
    re: f64,
    im: f64,
    imp: &str,
    z0: f64,
    freq: f64,
    f_scale: &str,
    _r_scale: &str,
    c_scale: &str,
) -> Response {
    let freq_unit = Unit::from_str(f_scale).unwrap();
    let cap_unit = Unit::from_str(c_scale).unwrap();

    let (z, g) = match imp {
        "z" => (Complex::new(re, im), calc_gamma(Complex::new(re, im), z0)),
        "ri" => (calc_z(Complex::new(re, im), z0), Complex::new(re, im)),
        "ma" => (
            calc_z(Complex::from_polar(re, im * PI / 180.0), z0),
            Complex::from_polar(re, im * PI / 180.0),
        ),
        "db" => (
            calc_z(
                Complex::from_polar(10_f64.powf(re / 20.0), im * PI / 180.0),
                z0,
            ),
            Complex::from_polar(10_f64.powf(re / 20.0), im * PI / 180.0),
        ),
        "rc" => (
            calc_z_from_rc(re, im, freq, &freq_unit, &Unit::Base, &cap_unit),
            calc_gamma_from_rc(re, im, z0, freq, &freq_unit, &Unit::Base, &cap_unit),
        ),
        _ => (Complex::ONE, Complex::ONE),
    };

    let (r, c) = calc_rc(z, freq, &freq_unit, &Unit::Base, &cap_unit);

    let out = ResponseReturn {
        z: z,
        g: g,
        g_mag: g.norm(),
        g_ang: g.arg() * 180.0 / PI,
        r: r,
        c: c,
    };

    Response::new(serde_json::to_string(&out).unwrap())
}

pub fn comp_c64(
    calc: &Complex<f64>,
    exemplar: &Complex<f64>,
    precision: F64Margin,
    test: &str,
    idx: &str,
) {
    comp_f64(
        &(*calc).re,
        &(*exemplar).re,
        precision,
        test,
        &(idx.to_owned() + ".re"),
    );
    comp_f64(
        &(*calc).im,
        &(*exemplar).im,
        precision,
        test,
        &(idx.to_owned() + ".im"),
    );
}

pub fn comp_f64(calc: &f64, exemplar: &f64, precision: F64Margin, test: &str, idx: &str) {
    debug_assert!(
        // approx_eq!(f64, *calc, *exemplar, F64Margin::default()),
        approx_eq!(f64, *calc, *exemplar, precision),
        " Failed test {} at location {}\n  exemplar: {}\n      calc: {}",
        test,
        idx,
        exemplar,
        calc
    );
}

pub fn comp_vec_f64(
    calc: Vec<f64>,
    exemplar: Vec<f64>,
    precision: F64Margin,
    test: &str,
    idx: &str,
) {
    for i in 0..calc.len() {
        comp_f64(
            &calc[i],
            &exemplar[i],
            precision,
            test,
            &(idx.to_owned() + "[" + i.to_string().as_str() + "]"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_from_str() {
        let tera = ["tera", "T", "THz", "thz"];
        let giga = ["giga", "G", "GHz", "ghz", "GΩ"];
        let mega = ["mega", "M", "MHz", "mhz", "MΩ"];
        let kilo = ["kilo", "k", "kHz", "khz", "kΩ"];
        let milli = ["milli", "m", "mΩ", "mF", "mH"];
        let micro = ["micro", "u", "μΩ", "μF", "μH"];
        let nano = ["nano", "n", "nΩ", "nF", "nH"];
        let pico = ["pico", "p", "pΩ", "pF", "pH"];
        let femto = ["femto", "f", "fΩ", "fF", "fH"];
        let lambda = ["lambda", "λ"];
        let nada = ["", "google", ".sfwe"];

        for mult in tera.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Tera);
        }

        for mult in giga.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Giga);
        }

        for mult in mega.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Mega);
        }

        for mult in kilo.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Kilo);
        }

        for mult in milli.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Milli);
        }

        for mult in micro.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Micro);
        }

        for mult in nano.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Nano);
        }

        for mult in pico.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Pico);
        }

        for mult in femto.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Femto);
        }

        for mult in lambda.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Lambda(1.0, 1.0));
        }

        for mult in nada.iter() {
            assert_eq!(Unit::from_str(mult).unwrap(), Unit::Base);
        }
    }

    #[test]
    fn test_gen_complex() {
        let re = 42.4;
        let im = -19.6;
        let mag = 0.435;
        let ang = 69.3;
        let db = 15.6;
        let angdb = -127.3;
        let exemplar = [
            Complex::new(42.4, -19.6),
            Complex::new(0.15376155704397684, 0.40691815341099224),
            Complex::new(-3.65144119629969, -4.793201713570547),
        ];
        let test_ri = gen_complex(re, im, "ri").unwrap();
        let test_ma = gen_complex(mag, ang, "ma").unwrap();
        let test_db = gen_complex(db, angdb, "db").unwrap();

        comp_c64(
            &test_ri,
            &exemplar[0],
            F64Margin::default(),
            "gen_complex()",
            "ri",
        );
        comp_c64(
            &test_ma,
            &exemplar[1],
            F64Margin::default(),
            "gen_complex()",
            "ma",
        );
        comp_c64(
            &test_db,
            &exemplar[2],
            F64Margin::default(),
            "gen_complex()",
            "db",
        );
    }

    #[test]
    fn test_get_unit() {
        assert_eq!(get_unit(&Unit::Tera, &UnitType::Farad), "TF".to_string());
        assert_eq!(get_unit(&Unit::Tera, &UnitType::Henry), "TH".to_string());
        assert_eq!(get_unit(&Unit::Tera, &UnitType::Ohm), "TΩ".to_string());
        assert_eq!(get_unit(&Unit::Tera, &UnitType::Hz), "THz".to_string());

        assert_eq!(get_unit(&Unit::Giga, &UnitType::Farad), "GF".to_string());
        assert_eq!(get_unit(&Unit::Giga, &UnitType::Henry), "GH".to_string());
        assert_eq!(get_unit(&Unit::Giga, &UnitType::Ohm), "GΩ".to_string());
        assert_eq!(get_unit(&Unit::Giga, &UnitType::Hz), "GHz".to_string());

        assert_eq!(get_unit(&Unit::Mega, &UnitType::Farad), "MF".to_string());
        assert_eq!(get_unit(&Unit::Mega, &UnitType::Henry), "MH".to_string());
        assert_eq!(get_unit(&Unit::Mega, &UnitType::Ohm), "MΩ".to_string());
        assert_eq!(get_unit(&Unit::Mega, &UnitType::Hz), "MHz".to_string());

        assert_eq!(get_unit(&Unit::Kilo, &UnitType::Farad), "kF".to_string());
        assert_eq!(get_unit(&Unit::Kilo, &UnitType::Henry), "kH".to_string());
        assert_eq!(get_unit(&Unit::Kilo, &UnitType::Ohm), "kΩ".to_string());
        assert_eq!(get_unit(&Unit::Kilo, &UnitType::Hz), "kHz".to_string());

        assert_eq!(get_unit(&Unit::Milli, &UnitType::Farad), "mF".to_string());
        assert_eq!(get_unit(&Unit::Milli, &UnitType::Henry), "mH".to_string());
        assert_eq!(get_unit(&Unit::Milli, &UnitType::Ohm), "mΩ".to_string());
        assert_eq!(get_unit(&Unit::Milli, &UnitType::Hz), "mHz".to_string());

        assert_eq!(get_unit(&Unit::Micro, &UnitType::Farad), "μF".to_string());
        assert_eq!(get_unit(&Unit::Micro, &UnitType::Henry), "μH".to_string());
        assert_eq!(get_unit(&Unit::Micro, &UnitType::Ohm), "μΩ".to_string());
        assert_eq!(get_unit(&Unit::Micro, &UnitType::Hz), "μHz".to_string());

        assert_eq!(get_unit(&Unit::Nano, &UnitType::Farad), "nF".to_string());
        assert_eq!(get_unit(&Unit::Nano, &UnitType::Henry), "nH".to_string());
        assert_eq!(get_unit(&Unit::Nano, &UnitType::Ohm), "nΩ".to_string());
        assert_eq!(get_unit(&Unit::Nano, &UnitType::Hz), "nHz".to_string());

        assert_eq!(get_unit(&Unit::Pico, &UnitType::Farad), "pF".to_string());
        assert_eq!(get_unit(&Unit::Pico, &UnitType::Henry), "pH".to_string());
        assert_eq!(get_unit(&Unit::Pico, &UnitType::Ohm), "pΩ".to_string());
        assert_eq!(get_unit(&Unit::Pico, &UnitType::Hz), "pHz".to_string());

        assert_eq!(get_unit(&Unit::Femto, &UnitType::Farad), "fF".to_string());
        assert_eq!(get_unit(&Unit::Femto, &UnitType::Henry), "fH".to_string());
        assert_eq!(get_unit(&Unit::Femto, &UnitType::Ohm), "fΩ".to_string());
        assert_eq!(get_unit(&Unit::Femto, &UnitType::Hz), "fHz".to_string());

        assert_eq!(get_unit(&Unit::Base, &UnitType::Farad), "F".to_string());
        assert_eq!(get_unit(&Unit::Base, &UnitType::Henry), "H".to_string());
        assert_eq!(get_unit(&Unit::Base, &UnitType::Ohm), "Ω".to_string());
        assert_eq!(get_unit(&Unit::Base, &UnitType::Hz), "Hz".to_string());
    }

    #[test]
    fn test_scale_unscale() {
        let val: f64 = 3.24;

        assert_eq!(scale(val, &Unit::Tera), val * 1e-12);
        assert_eq!(unscale(val, &Unit::Tera), val * 1e12);

        assert_eq!(scale(val, &Unit::Giga), val * 1e-9);
        assert_eq!(unscale(val, &Unit::Giga), val * 1e9);

        assert_eq!(scale(val, &Unit::Mega), val * 1e-6);
        assert_eq!(unscale(val, &Unit::Mega), val * 1e6);

        assert_eq!(scale(val, &Unit::Kilo), val * 1e-3);
        assert_eq!(unscale(val, &Unit::Kilo), val * 1e3);

        assert_eq!(scale(val, &Unit::Milli), val * 1e3);
        assert_eq!(unscale(val, &Unit::Milli), val * 1e-3);

        assert_eq!(scale(val, &Unit::Micro), val * 1e6);
        assert_eq!(unscale(val, &Unit::Micro), val * 1e-6);

        assert_eq!(scale(val, &Unit::Nano), val * 1e9);
        assert_eq!(unscale(val, &Unit::Nano), val * 1e-9);

        assert_eq!(scale(val, &Unit::Pico), val * 1e12);
        assert_eq!(unscale(val, &Unit::Pico), val * 1e-12);

        assert_eq!(scale(val, &Unit::Femto), val * 1e15);
        assert_eq!(unscale(val, &Unit::Femto), val * 1e-15);

        assert_eq!(scale(val, &Unit::Base), val * 1e0);
        assert_eq!(unscale(val, &Unit::Base), val * 1e0);

        comp_f64(
            &scale(1.0, &Unit::Lambda(25e9, 3.4)),
            &(3e8 / (25e9 * 3.4_f64.sqrt())),
            F64Margin::default(),
            "scale()",
            "lambda(25e9, 3.4)",
        );
        comp_f64(
            &unscale(1.0, &Unit::Lambda(43e9, 2.5)),
            &(43e9 * 2.5_f64.sqrt() / 3e8),
            F64Margin::default(),
            "unscale()",
            "lambda(43e9, 2.5)",
        );
    }

    #[test]
    fn test_calc_gamma() {
        let z = Complex::new(42.4, -19.6);
        let z0 = 50.0;
        let gamma = Complex::new(-0.03565151895556114, -0.21968365553602814);
        let test = calc_gamma(z, z0);

        comp_c64(&test, &gamma, F64Margin::default(), "calc_gamma()", "gamma");
    }

    #[test]
    fn test_calc_z() {
        let gamma = Complex::new(0.2464, -0.8745);
        let z0 = 100.0;
        let z = Complex::new(13.096841624374102, -131.24096072255193);
        let test = calc_z(gamma, z0);

        comp_c64(&test, &z, F64Margin::default(), "calc_z()", "z");
    }

    #[test]
    fn test_calc_rc() {
        let z = Complex::new(42.4, -19.6);
        let f = 275.0;
        let r = 51.46037735849057;
        let c = 5.198818862788317;
        let test = calc_rc(z, f, &Unit::Giga, &Unit::Base, &Unit::Femto);

        comp_f64(&test.0, &r, F64Margin::default(), "calc_rc()", "r");
        comp_f64(&test.1, &c, F64Margin::default(), "calc_rc()", "c");
    }
}
