#![allow(unused)]
use std::error::Error;
use std::str::FromStr;
use std::string::ToString;

pub enum UnitType {
    Farad,
    Henry,
    Ohm,
    Hz,
}

impl FromStr for UnitType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "cap" | "capacitor" | "F" => Ok(UnitType::Farad),
            "l" | "ind" | "inductor" | "H" => Ok(UnitType::Henry),
            "r" | "res" | "resistor" | "Ω" => Ok(UnitType::Ohm),
            "f" | "freq" | "frequency" | "Hz" | "hz" => Ok(UnitType::Hz),
            _ => Err("UnitType not recognize".to_string().into()),
        }
    }
}

impl ToString for UnitType {
    fn to_string(&self) -> String {
        match self {
            UnitType::Farad => "F".to_string(),
            UnitType::Henry => "H".to_string(),
            UnitType::Ohm => "Ω".to_string(),
            UnitType::Hz => "Hz".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    Lambda(f64, f64),
    Q,
    K,
    N,
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
            "micro" | "u" | "uΩ" | "μΩ" | "uF" | "μF" | "uH" | "μH" | "um" | "μm" => {
                Ok(Unit::Micro)
            }
            "nano" | "n" | "nΩ" | "nF" | "nH" => Ok(Unit::Nano),
            "pico" | "p" | "pΩ" | "pF" | "pH" => Ok(Unit::Pico),
            "femto" | "f" | "fΩ" | "fF" | "fH" => Ok(Unit::Femto),
            "lambda" | "λ" | "wavelength" => Ok(Unit::Lambda(1.0, 1.0)),
            "Q" | "q" => Ok(Unit::Q),
            "K" => Ok(Unit::K),
            "N" => Ok(Unit::N),
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
            Unit::Lambda(_, _) => "λ".to_string(),
            Unit::Q => "Q".to_string(),
            Unit::K => "K".to_string(),
            Unit::N => "N".to_string(),
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
            Unit::Lambda(freq, er) => 3e8 / (freq * er.sqrt()),
            Unit::Q | Unit::K | Unit::N => 1.0,
        }
    }

    pub fn unscale(&self) -> f64 {
        1.0 / self.scale()
    }
}

pub fn get_unit(unit: &Unit, unit_type: &UnitType) -> String {
    if *unit == Unit::Micro {
        return format!("μ{}", unit_type.to_string());
    }
    format!("{}{}", unit.to_string(), unit_type.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_unit_scale(unit: &str, freq: f64, er: f64) -> f64 {
    let mut val = Unit::from_str(unit).unwrap();
    if val == Unit::Lambda(1.0, 1.0) {
        val = Unit::Lambda(freq, er);
    }
    val.scale()
}
