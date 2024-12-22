use num_complex::Complex;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::error::Error;
use std::f64::consts::PI;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

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

pub enum Element {
    Capacitor,
    Inductor,
    Resistor,
    Frequency,
}

impl FromStr for Element {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "cap" | "capacitor" => Ok(Element::Capacitor),
            "l" | "ind" | "inductor" => Ok(Element::Inductor),
            "r" | "res" | "resistor" => Ok(Element::Resistor),
            "f" | "freq" | "frequency" => Ok(Element::Frequency),
            _ => Err("Element not recognize".to_string().into()),
        }
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        match self {
            Element::Capacitor => "F".to_string(),
            Element::Inductor => "H".to_string(),
            Element::Resistor => "Ω".to_string(),
            Element::Frequency => "Hz".to_string(),
        }
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

#[derive(Debug, PartialEq)]
pub enum Unit {
    Tera,
    Giga,
    Mega,
    Kilo,
    Base,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
}

impl FromStr for Unit {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tera" | "T" | "THz" | "thz" => Ok(Unit::Tera),
            "giga" | "G" | "GHz" | "ghz" | "GΩ" => Ok(Unit::Giga),
            "mega" | "M" | "MHz" | "mhz" | "MΩ" => Ok(Unit::Mega),
            "kilo" | "k" | "kHz" | "khz" | "kΩ" => Ok(Unit::Kilo),
            "milli" | "m" | "mΩ" | "mF" | "mH" => Ok(Unit::Milli),
            "micro" | "u" | "μΩ" | "μF" | "μH" => Ok(Unit::Micro),
            "nano" | "n" | "nΩ" | "nF" | "nH" => Ok(Unit::Nano),
            "pico" | "p" | "pΩ" | "pF" | "pH" => Ok(Unit::Pico),
            "femto" | "f" | "fΩ" | "fF" | "fH" => Ok(Unit::Femto),
            _ => Ok(Unit::Base),
        }
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Tera => "T".to_string(),
            Unit::Giga => "G".to_string(),
            Unit::Mega => "M".to_string(),
            Unit::Kilo => "k".to_string(),
            Unit::Base => "".to_string(),
            Unit::Milli => "m".to_string(),
            // Unit::Micro => "μ".to_string(),
            Unit::Micro => "u".to_string(),
            Unit::Nano => "n".to_string(),
            Unit::Pico => "p".to_string(),
            Unit::Femto => "f".to_string(),
        }
    }
}

impl Unit {
    pub fn scale(&self) -> f64 {
        match self {
            Unit::Tera => 1e-12,
            Unit::Giga => 1e-9,
            Unit::Mega => 1e-6,
            Unit::Kilo => 1e-3,
            Unit::Base => 1.0,
            Unit::Milli => 1e3,
            Unit::Micro => 1e6,
            Unit::Nano => 1e9,
            Unit::Pico => 1e12,
            Unit::Femto => 1e15,
        }
    }

    pub fn unscale(&self) -> f64 {
        match self {
            Unit::Tera => 1e12,
            Unit::Giga => 1e9,
            Unit::Mega => 1e6,
            Unit::Kilo => 1e3,
            Unit::Base => 1.0,
            Unit::Milli => 1e-3,
            Unit::Micro => 1e-6,
            Unit::Nano => 1e-9,
            Unit::Pico => 1e-12,
            Unit::Femto => 1e-15,
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

pub fn get_unit(unit: &Unit, elem: &Element) -> String {
    format!("{}{}", unit.to_string(), elem.to_string())
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
    rscale: &Unit,
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

pub fn calc_z_from_rc(
    r: f64,
    c: f64,
    freq: f64,
    fscale: &Unit,
    rscale: &Unit,
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

        assert_eq!(gen_complex(re, im, "ri").unwrap(), exemplar[0]);
        assert_eq!(gen_complex(mag, ang, "ma").unwrap(), exemplar[1]);
        assert_eq!(gen_complex(db, angdb, "db").unwrap(), exemplar[2]);
    }

    #[test]
    fn test_get_unit() {
        assert_eq!(get_unit(&Unit::Tera, &Element::Capacitor), "TF".to_string());
        assert_eq!(get_unit(&Unit::Tera, &Element::Inductor), "TH".to_string());
        assert_eq!(get_unit(&Unit::Tera, &Element::Resistor), "TΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Tera, &Element::Frequency),
            "THz".to_string()
        );

        assert_eq!(get_unit(&Unit::Giga, &Element::Capacitor), "GF".to_string());
        assert_eq!(get_unit(&Unit::Giga, &Element::Inductor), "GH".to_string());
        assert_eq!(get_unit(&Unit::Giga, &Element::Resistor), "GΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Giga, &Element::Frequency),
            "GHz".to_string()
        );

        assert_eq!(get_unit(&Unit::Mega, &Element::Capacitor), "MF".to_string());
        assert_eq!(get_unit(&Unit::Mega, &Element::Inductor), "MH".to_string());
        assert_eq!(get_unit(&Unit::Mega, &Element::Resistor), "MΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Mega, &Element::Frequency),
            "MHz".to_string()
        );

        assert_eq!(get_unit(&Unit::Kilo, &Element::Capacitor), "kF".to_string());
        assert_eq!(get_unit(&Unit::Kilo, &Element::Inductor), "kH".to_string());
        assert_eq!(get_unit(&Unit::Kilo, &Element::Resistor), "kΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Kilo, &Element::Frequency),
            "kHz".to_string()
        );

        assert_eq!(
            get_unit(&Unit::Milli, &Element::Capacitor),
            "mF".to_string()
        );
        assert_eq!(get_unit(&Unit::Milli, &Element::Inductor), "mH".to_string());
        assert_eq!(get_unit(&Unit::Milli, &Element::Resistor), "mΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Milli, &Element::Frequency),
            "mHz".to_string()
        );

        assert_eq!(
            get_unit(&Unit::Micro, &Element::Capacitor),
            "μF".to_string()
        );
        assert_eq!(get_unit(&Unit::Micro, &Element::Inductor), "μH".to_string());
        assert_eq!(get_unit(&Unit::Micro, &Element::Resistor), "μΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Micro, &Element::Frequency),
            "μHz".to_string()
        );

        assert_eq!(get_unit(&Unit::Nano, &Element::Capacitor), "nF".to_string());
        assert_eq!(get_unit(&Unit::Nano, &Element::Inductor), "nH".to_string());
        assert_eq!(get_unit(&Unit::Nano, &Element::Resistor), "nΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Nano, &Element::Frequency),
            "nHz".to_string()
        );

        assert_eq!(get_unit(&Unit::Pico, &Element::Capacitor), "pF".to_string());
        assert_eq!(get_unit(&Unit::Pico, &Element::Inductor), "pH".to_string());
        assert_eq!(get_unit(&Unit::Pico, &Element::Resistor), "pΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Pico, &Element::Frequency),
            "pHz".to_string()
        );

        assert_eq!(
            get_unit(&Unit::Femto, &Element::Capacitor),
            "fF".to_string()
        );
        assert_eq!(get_unit(&Unit::Femto, &Element::Inductor), "fH".to_string());
        assert_eq!(get_unit(&Unit::Femto, &Element::Resistor), "fΩ".to_string());
        assert_eq!(
            get_unit(&Unit::Femto, &Element::Frequency),
            "fHz".to_string()
        );

        assert_eq!(get_unit(&Unit::Base, &Element::Capacitor), "F".to_string());
        assert_eq!(get_unit(&Unit::Base, &Element::Inductor), "H".to_string());
        assert_eq!(get_unit(&Unit::Base, &Element::Resistor), "Ω".to_string());
        assert_eq!(get_unit(&Unit::Base, &Element::Frequency), "Hz".to_string());
    }

    #[test]
    fn test_scale_unscale() {
        let tera = ["tera", "T", "THz", "thz"];
        let giga = ["giga", "G", "GHz", "ghz", "GΩ"];
        let mega = ["mega", "M", "MHz", "mhz", "MΩ"];
        let kilo = ["kilo", "k", "kHz", "khz", "kΩ"];
        let milli = ["milli", "m", "mΩ", "mF", "mH"];
        let micro = ["micro", "u", "μΩ", "μF", "μH"];
        let nano = ["nano", "n", "nΩ", "nF", "nH"];
        let pico = ["pico", "p", "pΩ", "pF", "pH"];
        let femto = ["femto", "f", "fΩ", "fF", "fH"];
        let nada = ["", "google", ".sfwe"];
        let val: f64 = 3.24;

        for mult in tera.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e-12);
            assert_eq!(unscale(val, &unit), val * 1e12);
        }

        for mult in giga.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e-9);
            assert_eq!(unscale(val, &unit), val * 1e9);
        }

        for mult in mega.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e-6);
            assert_eq!(unscale(val, &unit), val * 1e6);
        }

        for mult in kilo.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e-3);
            assert_eq!(unscale(val, &unit), val * 1e3);
        }

        for mult in milli.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e3);
            assert_eq!(unscale(val, &unit), val * 1e-3);
        }

        for mult in micro.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e6);
            assert_eq!(unscale(val, &unit), val * 1e-6);
        }

        for mult in nano.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e9);
            assert_eq!(unscale(val, &unit), val * 1e-9);
        }

        for mult in pico.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e12);
            assert_eq!(unscale(val, &unit), val * 1e-12);
        }

        for mult in femto.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e15);
            assert_eq!(unscale(val, &unit), val * 1e-15);
        }

        for mult in nada.iter() {
            let unit = Unit::from_str(mult).unwrap();
            assert_eq!(scale(val, &unit), val * 1e0);
            assert_eq!(unscale(val, &unit), val * 1e0);
        }
    }

    #[test]
    fn test_calc_gamma() {
        let z = Complex::new(42.4, -19.6);
        let z0 = 50.0;
        let gamma = Complex::new(-0.03565151895556114, -0.21968365553602814);

        assert_eq!(calc_gamma(z, z0), gamma);
    }

    #[test]
    fn test_calc_z() {
        let gamma = Complex::new(0.2464, -0.8745);
        let z0 = 100.0;
        let z = Complex::new(13.096841624374102, -131.24096072255193);

        assert_eq!(calc_z(gamma, z0), z);
    }

    #[test]
    fn test_calc_rc() {
        let z = Complex::new(42.4, -19.6);
        let f = 275.0;
        let r = 51.46037735849057;
        let c = 5.198818862788317;

        assert_eq!(
            calc_rc(z, f, &Unit::Giga, &Unit::Base, &Unit::Femto),
            (r, c)
        );
    }
}
