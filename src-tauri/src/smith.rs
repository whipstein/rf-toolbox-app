#![allow(dead_code, unused_variables, unused_imports)]
use crate::rf_utils::{calc_z, comp_vec_f64, unscale, ComplexReturn, Unit};
use float_cmp::{approx_eq, F64Margin};
use num_complex::{c64, Complex};
use serde::Serialize;
use std::error::Error;
use std::f64::consts::PI;
use std::f64::{EPSILON, NAN};
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq)]
pub enum Element {
    SeriesCapacitor,
    ShuntCapacitor,
    SeriesInductor,
    ShuntInductor,
    SeriesResistor,
    ShuntResistor,
    TransmissionLine,
    OpenStub,
    ShortedStub,
    Transformer,
    ResistorInductorCapacitor,
    CustomZ,
}

impl FromStr for Element {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sc" | "ser_cap" | "series_cap" | "series_capacitor" => Ok(Element::SeriesCapacitor),
            "pc" | "shnt_cap" | "shunt_cap" | "shunt_capacitor" => Ok(Element::ShuntCapacitor),
            "si" | "ser_ind" | "series_ind" | "series_inductor" => Ok(Element::SeriesInductor),
            "pi" | "shnt_ind" | "shunt_ind" | "shunt_inductor" => Ok(Element::ShuntInductor),
            "sr" | "ser_res" | "series_res" | "series_resistor" => Ok(Element::SeriesResistor),
            "pr" | "shnt_res" | "shunt_res" | "shunt_resistor" => Ok(Element::ShuntResistor),
            "tl" | "tline" | "transmission_line" => Ok(Element::TransmissionLine),
            "os" | "open_stub" => Ok(Element::OpenStub),
            "ss" | "short_stub" | "shorted_stub" => Ok(Element::ShortedStub),
            "xfmr" | "transformer" => Ok(Element::Transformer),
            "rlc" | "res_ind_cap" | "resistor_inductor_capacitor" => {
                Ok(Element::ResistorInductorCapacitor)
            }
            "custom_z" | "customZ" => Ok(Element::CustomZ),
            _ => Err("Element not recognize".to_string().into()),
        }
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        match self {
            Element::SeriesCapacitor => "series_capacitor".to_string(),
            Element::ShuntCapacitor => "shunt_capacitor".to_string(),
            Element::SeriesInductor => "series_inductor".to_string(),
            Element::ShuntInductor => "shunt_inductor".to_string(),
            Element::SeriesResistor => "series_resistor".to_string(),
            Element::ShuntResistor => "shunt_resistor".to_string(),
            Element::TransmissionLine => "transmission_line".to_string(),
            Element::OpenStub => "open_stub".to_string(),
            Element::ShortedStub => "shorted_stub".to_string(),
            Element::Transformer => "transformer".to_string(),
            Element::ResistorInductorCapacitor => "rlc".to_string(),
            Element::CustomZ => "custom_z".to_string(),
        }
    }
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub struct ArcReturn {
    pub x_coord: Vec<f64>,
    pub y_coord: Vec<f64>,
    pub end_x_coord: f64,
    pub end_y_coord: f64,
    pub real_old: f64,
    pub imag_old: f64,
    pub start_x_coord: f64,
    pub start_y_coord: f64,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[tauri::command(rename_all = "snake_case")]
pub fn find_smith_coord(re: f64, im: f64, rotate: bool, verbose: bool) -> Result<Vec<f64>, String> {
    if verbose {
        println!("\nfind_smith_coord({:?}, {:?}, {:?})", re, im, rotate);
    }

    let mut new_im: f64 = im;
    if !im.is_finite() {
        new_im = 0.0;
    }

    let mut z: Complex<f64> = Complex::new(re, new_im);
    if rotate {
        z = z.inv();
    }
    let g = (z - c64(1.0, 0.0)) / (z + c64(1.0, 0.0));

    if verbose {
        println!("z = {:?}", z);
        println!("g = {:?}", g);
    }
    Ok(vec![g.re, g.im])
}

fn find_smith_coord_c64(
    val: Complex<f64>,
    rotate: bool,
    verbose: bool,
) -> Result<Vec<f64>, String> {
    if verbose {
        println!("\nfind_smith_coord_c64({:?}, {:?})", val, rotate);
    }

    find_smith_coord(val.re, val.im, rotate, verbose)
}

#[tauri::command(rename_all = "snake_case")]
pub fn arc_smith_points(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    type_: &str,
    rotate: bool,
    beta: f64,
    start_at_qtr_wl: f64,
    z0: f64,
    resolution: usize,
    verbose: bool,
) -> Result<ArcReturn, String> {
    if verbose {
        println!(
            "\narc_smith_points({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            x1, y1, x2, y2, type_, rotate, beta, start_at_qtr_wl
        );
        println!("z0 = {z0}, resolution = {resolution}");
    }
    let mut x_coord: Vec<f64> = vec![0.0; resolution + 1];
    let mut y_coord: Vec<f64> = vec![0.0; resolution + 1];
    let mut end_x_coord: f64 = 0.0;
    let mut end_y_coord: f64 = 0.0;
    if verbose {
        println!("x1 = {}, y1 = {}", x1, y1);
    }
    let mut temp_array = find_smith_coord(x1, y1, rotate, false).unwrap();
    let start_x_coord: f64 = temp_array[0];
    let start_y_coord: f64 = temp_array[1];
    let mut real_old: f64 = 0.0;
    let mut imag_old: f64 = 0.0;
    let mut stub_admittance_im: f64 = 0.0;
    let mut real_answer: f64 = 0.0;
    let mut imag_answer: f64 = 0.0;

    //used for transmission lines and stubs
    let line_zo = y2;
    let line_length = x2;
    let top_real_temp = x1 * line_zo;
    let zl: Complex<f64> = calc_z(Complex::new(start_x_coord, start_y_coord), z0);

    for i in 0..=resolution {
        if type_ == "transmission_line" {
            let betal = (beta * (i as f64) * line_length) / (resolution as f64);
            let zi = line_zo
                * ((zl + Complex::<f64>::I * line_zo * betal.tan())
                    / (line_zo + Complex::<f64>::I * zl * betal.tan()))
                / z0;
            temp_array = find_smith_coord_c64(zi, rotate, false).unwrap();

            x_coord[i] = temp_array[0];
            y_coord[i] = temp_array[1];
            real_answer = zi.re;
            imag_answer = zi.im;
        } else if type_ == "ss" {
            let mut tan_beta_arg: f64 = 0.0;
            if approx_eq!(f64, start_at_qtr_wl, 0_f64, F64Margin::default()) {
                tan_beta_arg = (beta * (i as f64) * line_length) / (resolution as f64);
            } else {
                tan_beta_arg = beta
                    * ((start_at_qtr_wl as f64)
                        + ((i as f64) * (line_length - (start_at_qtr_wl as f64)))
                            / (resolution as f64));
            }
            stub_admittance_im = -1.0 / ((tan_beta_arg.tan() * line_zo) / z0);
            temp_array = find_smith_coord(x1, y1 + stub_admittance_im, rotate, false).unwrap();
            x_coord[i] = temp_array[0];
            y_coord[i] = temp_array[1];
        } else if type_ == "so" {
            let tan_beta_arg = (beta * (i as f64) * line_length) / (resolution as f64);
            stub_admittance_im = tan_beta_arg.tan() / (line_zo / z0);
            temp_array = find_smith_coord(x1, y1 + stub_admittance_im, rotate, false).unwrap();
            x_coord[i] = temp_array[0];
            y_coord[i] = temp_array[1];
        } else {
            temp_array = find_smith_coord(
                x1 + ((x2 - x1) * (i as f64)) / (resolution as f64),
                y1 + ((y2 - y1) * (i as f64)) / (resolution as f64),
                rotate,
                false,
            )
            .unwrap();
            x_coord[i] = temp_array[0];
            y_coord[i] = temp_array[1];
        }
    }

    if type_ == "transmission_line" {
        temp_array = find_smith_coord(real_answer, imag_answer, rotate, false).unwrap();
        real_old = real_answer;
        imag_old = imag_answer;
    } else if (type_ == "so") || (type_ == "ss") {
        real_old = x1;
        imag_old = y1 + stub_admittance_im;
    }

    let end_x_coord = temp_array[0];
    let end_y_coord = temp_array[1];

    if verbose {
        println!("=");
        print!("[[");
        for i in 0..x_coord.len() {
            print!("{}", x_coord[i]);
            if i != x_coord.len() - 1 {
                print!(", ");
            }
        }
        print!("], [");
        for i in 0..y_coord.len() {
            print!("{}", y_coord[i]);
            if i != x_coord.len() - 1 {
                print!(", ");
            }
        }
        println!(
            "], {}, {}, {}, {}, {}, {}, {}, {}, {}, {}]",
            end_x_coord,
            end_y_coord,
            real_old,
            imag_old,
            start_x_coord,
            start_y_coord,
            x1,
            y1,
            x2,
            y2
        );
    }
    Ok(ArcReturn {
        x_coord,
        y_coord,
        end_x_coord,
        end_y_coord,
        real_old,
        imag_old,
        start_x_coord,
        start_y_coord,
        x1,
        y1,
        x2,
        y2,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_ri(
    vals: Vec<f64>,
    units: Vec<&str>,
    lut: Vec<[f64; 3]>,
    type_: &str,
    freq: f64,
    z0: f64,
    diff: bool,
    verbose: bool,
) -> Result<Vec<f64>, String> {
    if verbose {
        print!("\ncalc_ri([");
        for i in 0..vals.len() {
            print!("{:?}", vals[i]);
            if i != vals.len() - 1 {
                print!(", ");
            }
        }
        print!("], [");
        for i in 0..units.len() {
            print!("{:?}", units[i]);
            if i != units.len() - 1 {
                print!(", ");
            }
        }
        print!("], [");
        for i in 0..lut.len() {
            print!("[{:?}, {:?}, {:?}]", lut[i][0], lut[i][1], lut[i][2]);
            if i != lut.len() - 1 {
                print!(", ");
            }
        }
        println!("], {:?}, {:?}, {:?}, {:?})", type_, freq, z0, diff);
    }

    let (re, im, ln, r1, r2) = match type_ {
        "bb" => {
            if diff && type_ == "bb" {
                (vals[0] / 2.0 / z0, vals[1] / 2.0 / z0, 0.0, 0.0, 0.0)
            } else {
                (vals[0] / z0, vals[1] / z0, 0.0, 0.0, 0.0)
            }
        }
        "sr" | "pr" => (
            unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0,
            0.0,
            0.0,
            0.0,
            0.0,
        ),
        "sc" | "pc" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = 1.0
                    / (2.0
                        * PI
                        * freq
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                        * vals[0]);
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            (
                tmp / z0,
                -1.0 / (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                    / z0,
                0.0,
                0.0,
                0.0,
            )
        }
        "si" | "pi" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                    / vals[0];
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            (
                tmp / z0,
                2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()) / z0,
                0.0,
                0.0,
                0.0,
            )
        }
        "xfmr" => {
            let mut l1_tee: f64 = 0.0;
            let mut l2_tee: f64 = 0.0;
            let mut ls_tee: f64 = 0.0;
            let n: f64 = 0.0;
            let mut r1: f64 = 0.0;
            let mut r2: f64 = 0.0;
            let l1 = unscale(vals[1], &Unit::from_str(units[1]).unwrap());
            let mut l2 = unscale(vals[2], &Unit::from_str(units[2]).unwrap());
            let mut k = vals[3];
            if units[3] == "k" {
                l1_tee = (1.0 - k) * l1;
                l2_tee = (1.0 - k) * l2;
                ls_tee = k * l1;
            }
            if units[2] == "n" {
                let n = vals[2];
                l2 = n.powi(2) * l1;
                l2_tee = (1.0 - k) * l2;
            }
            if units[3] != "k" {
                let m = unscale(vals[3], &Unit::from_str(units[3]).unwrap());
                k = m / l1;
                l1_tee = (1.0 - k) * l1;
                l2_tee = (1.0 - k) * l2;
                ls_tee = m;
            }
            if units[0] == "Q" && !approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                r1 = 2.0 * PI * freq * l1 / vals[0];
                r2 = 2.0 * PI * freq * l2 / vals[0];
            } else {
                r1 = vals[0];
                r2 = vals[0];
            }
            let z1: Complex<f64> = c64(r1, 2.0 * PI * freq * l1_tee);
            let z2: Complex<f64> = c64(0.0, 2.0 * PI * freq * ls_tee);
            let z3: Complex<f64> = c64(r2, 2.0 * PI * freq * l2_tee);
            let zout: Complex<f64> = ((z1.inv() + z2.inv()).inv() + z3) / z0;
            (l1_tee, l2_tee, ls_tee, r1, r2)
        }
        "rlc" => (
            unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0,
            (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                - 1.0 / (2.0 * PI * freq * unscale(vals[2], &Unit::from_str(units[2]).unwrap())))
                / z0,
            0.0,
            0.0,
            0.0,
        ),
        "rl" => (
            unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0,
            2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()) / z0,
            0.0,
            0.0,
            0.0,
        ),
        "rc" => (
            1.0 / unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0,
            -1.0 / (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap())) / z0,
            0.0,
            0.0,
            0.0,
        ),
        "tl" | "so" | "ss" => (
            0.0,
            0.0,
            unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
            0.0,
            0.0,
        ),
        "customZ" => {
            let mut index_res: usize = lut.len() - 1;
            let mut tmp_re: f64 = 0.0;
            let mut tmp_im: f64 = 0.0;
            for i in 0..lut.len() {
                if lut[i][0] > freq {
                    index_res = i;
                    break;
                }
            }

            if (index_res == lut.len() - 1) || (index_res == 0) {
                tmp_re = lut[index_res][1];
                tmp_im = lut[index_res][2];
            } else {
                let f1 = lut[index_res - 1][0];
                let f2 = lut[index_res][0];
                let frac = (freq - f1) / (f2 - f1);
                tmp_re = lut[index_res - 1][1] + frac * (lut[index_res][1] - lut[index_res - 1][1]);
                tmp_im = lut[index_res - 1][2] + frac * (lut[index_res][2] - lut[index_res - 1][2]);
            }

            (tmp_re / z0, tmp_im / z0, 0.0, 0.0, 0.0)
        }
        _ => return Err("element not recognized".to_string()),
    };

    if verbose {
        println!("[{:?}, {:?}, {:?}]", re, im, ln);
    }
    Ok(vec![re, im, ln])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_ri_new(
    vals: Vec<f64>,
    units: Vec<&str>,
    length: f64,
    line_z0: f64,
    lut: Vec<[f64; 3]>,
    type_: &str,
    freq: f64,
    freq_unit: &str,
    z0: f64,
    er: f64,
    rin: f64,
    xin: f64,
    diff: bool,
    verbose: bool,
) -> Result<Vec<f64>, String> {
    if verbose {
        print!("\ncalc_ri_new([");
        for i in 0..vals.len() {
            print!("{:?}", vals[i]);
            if i != vals.len() - 1 {
                print!(", ");
            }
        }
        print!("], [");
        for i in 0..units.len() {
            print!("{:?}", units[i]);
            if i != units.len() - 1 {
                print!(", ");
            }
        }
        print!("], {:?}, {:?}, [", length, line_z0);
        for i in 0..lut.len() {
            print!("[{:?}, {:?}, {:?}]", lut[i][0], lut[i][1], lut[i][2]);
            if i != lut.len() - 1 {
                print!(", ");
            }
        }
        println!(
            "], {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            type_, freq, freq_unit, z0, er, rin, xin, diff
        );
    }

    let zin: Complex<f64> = c64(rin * z0, xin * z0);

    let (rout, xout) = match type_ {
        "bb" => {
            if diff && type_ == "bb" {
                (rin + vals[0] / 2.0 / z0, xin + vals[1] / 2.0 / z0)
            } else {
                (rin + vals[0] / z0, xin + vals[1] / z0)
            }
        }
        "sr" => (
            rin + unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0,
            xin,
        ),
        "pr" => {
            let zout = (zin.inv()
                + c64(unscale(vals[0], &Unit::from_str(units[0]).unwrap()), 0.0).inv())
            .inv()
                / z0;
            (zout.re, zout.im)
        }
        "sc" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = 1.0
                    / (2.0
                        * PI
                        * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                        * vals[0]);
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            let zout =
                (zin + c64(
                    tmp,
                    -1.0 / (2.0
                        * PI
                        * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap())),
                )) / z0;
            (zout.re, zout.im)
        }
        "pc" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = 1.0
                    / (2.0
                        * PI
                        * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                        * vals[0]);
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            let zout = (zin.inv()
                + c64(
                    tmp,
                    -1.0 / (2.0
                        * PI
                        * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap())),
                )
                .inv())
            .inv()
                / z0;
            (zout.re, zout.im)
        }
        "si" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = (2.0
                    * PI
                    * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                    * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                    / vals[0];
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            let zout =
                (zin + c64(
                    tmp,
                    2.0 * PI
                        * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap()),
                )) / z0;
            (zout.re, zout.im)
        }
        "pi" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = (2.0
                    * PI
                    * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                    * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                    / vals[0];
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            let x = 2.0
                * PI
                * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                * unscale(vals[1], &Unit::from_str(units[1]).unwrap());
            let zout = (zin.inv() + c64(tmp, x).inv()).inv() / z0;
            (zout.re, zout.im)
        }
        "xfmr" => {
            let mut l1_tee: f64 = 0.0;
            let mut l2_tee: f64 = 0.0;
            let mut ls_tee: f64 = 0.0;
            let n: f64 = 0.0;
            let mut r1: f64 = 0.0;
            let mut r2: f64 = 0.0;
            let l1 = unscale(vals[1], &Unit::from_str(units[1]).unwrap());
            let mut l2 = unscale(vals[2], &Unit::from_str(units[2]).unwrap());
            let mut k = vals[3];
            if units[3] == "k" {
                l1_tee = (1.0 - k) * l1;
                l2_tee = (1.0 - k) * l2;
                ls_tee = k * l1;
            }
            if units[2] == "n" {
                let n = vals[2];
                l2 = n.powi(2) * l1;
                l2_tee = (1.0 - k) * l2;
            }
            if units[3] != "k" {
                let m = unscale(vals[3], &Unit::from_str(units[3]).unwrap());
                k = m / l1;
                l1_tee = (1.0 - k) * l1;
                l2_tee = (1.0 - k) * l2;
                ls_tee = m;
            }
            if units[0] == "Q" && !approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                r1 = 2.0 * PI * freq * l1 / vals[0];
                r2 = 2.0 * PI * freq * l2 / vals[0];
            } else {
                r1 = vals[0];
                r2 = vals[0];
            }
            let z1: Complex<f64> = c64(
                r1,
                2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * l1_tee,
            );
            let z2: Complex<f64> = c64(
                0.0,
                2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * ls_tee,
            );
            let z3: Complex<f64> = c64(
                r2,
                2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * l2_tee,
            );
            let zout: Complex<f64> = (zin
                + (c64(
                    r1,
                    2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * l1_tee,
                )
                .inv()
                    + c64(
                        0.0,
                        2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * ls_tee,
                    )
                    .inv())
                .inv()
                + c64(
                    r2,
                    2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * l2_tee,
                ))
                / z0;
            (zout.re, zout.im)
        }
        "rlc" => {
            let zrlc = c64(
                unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
                2.0 * PI
                    * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                    * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                    - 1.0
                        / (2.0
                            * PI
                            * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                            * unscale(vals[2], &Unit::from_str(units[2]).unwrap())),
            );
            let zout = (zin.inv() + zrlc.inv()).inv() / z0;
            (zout.re, zout.im)
        }
        "rl" => {
            let zrl = c64(
                unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
                2.0 * PI
                    * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                    * unscale(vals[1], &Unit::from_str(units[1]).unwrap()),
            );
            let zout = (zin.inv() + zrl.inv()).inv() / z0;
            (zout.re, zout.im)
        }
        "rc" => {
            let zrc = c64(
                1.0 / unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
                -1.0 / (2.0
                    * PI
                    * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                    * unscale(vals[1], &Unit::from_str(units[1]).unwrap())),
            );
            let zout = (zin.inv() + zrc.inv()).inv() / z0;
            (zout.re, zout.im)
        }
        "tl" => {
            let betal =
                2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * er.sqrt() * length;
            let zout = line_z0
                * ((zin + Complex::<f64>::I * line_z0 * betal.tan())
                    / (line_z0 + Complex::<f64>::I * zin * betal.tan()))
                / z0;
            (zout.re, zout.im)
        }
        "so" => {
            let betalinv = 1.0
                / (2.0
                    * PI
                    * unscale(freq, &Unit::from_str(freq_unit).unwrap())
                    * er.sqrt()
                    * length);
            let zstub = -Complex::<f64>::I * line_z0 * betalinv.tan();
            let zout = (zin.inv() + zstub.inv()).inv() / z0;
            (zout.re, zout.im)
        }
        "ss" => {
            let betal =
                2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap()) * er.sqrt() * length;
            let zstub = Complex::<f64>::I * line_z0 * betal.tan();
            let zout = (zin.inv() + zstub.inv()).inv() / z0;
            (zout.re, zout.im)
        }
        "customZ" => {
            let mut index_res: usize = lut.len() - 1;
            let mut tmp_re: f64 = 0.0;
            let mut tmp_im: f64 = 0.0;
            for i in 0..lut.len() {
                if lut[i][0] > freq {
                    index_res = i;
                    break;
                }
            }

            if (index_res == lut.len() - 1) || (index_res == 0) {
                tmp_re = lut[index_res][1];
                tmp_im = lut[index_res][2];
            } else {
                let f1 = lut[index_res - 1][0];
                let f2 = lut[index_res][0];
                let frac = (freq - f1) / (f2 - f1);
                tmp_re = lut[index_res - 1][1] + frac * (lut[index_res][1] - lut[index_res - 1][1]);
                tmp_im = lut[index_res - 1][2] + frac * (lut[index_res][2] - lut[index_res - 1][2]);
            }

            let zout = (zin + c64(tmp_re, tmp_im)) / z0;
            (zout.re, zout.im)
        }
        _ => return Err("element not recognized".to_string()),
    };

    if verbose {
        println!("[{:?}, {:?}]", rout, xout);
    }
    Ok(vec![rout, xout])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_ri_bb(vals: Vec<f64>, z0: f64, diff: &str, verbose: bool) -> Result<Vec<f64>, String> {
    if verbose {
        print!("\ncalc_ri_tline([");
        for i in 0..vals.len() {
            print!("{:?}", vals[i]);
            if i != vals.len() - 1 {
                print!(", ");
            }
        }
        println!("], {:?})", z0);
    }

    let (r, x) = match diff {
        "diff" => (vals[0] / 2.0, vals[1] / 2.0),
        _ => (vals[0], vals[1]),
    };

    let (rout, xout) = (r / z0, x / z0);
    if verbose {
        println!("[{:?}, {:?}]", rout, xout);
    }
    Ok(vec![rout, xout])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_ri_lumped(
    vals: Vec<f64>,
    units: Vec<&str>,
    type_: &str,
    freq: f64,
    freq_unit: &str,
    z0: f64,
    verbose: bool,
) -> Result<Vec<f64>, String> {
    if verbose {
        print!("\ncalc_ri_lumped([");
        for i in 0..vals.len() {
            print!("{:?}", vals[i]);
            if i != vals.len() - 1 {
                print!(", ");
            }
        }
        print!("], [");
        for i in 0..units.len() {
            print!("{:?}", units[i]);
            if i != units.len() - 1 {
                print!(", ");
            }
        }
        println!("], {:?}, {:?}, {:?}, {:?})", type_, freq, freq_unit, z0);
    }

    let freq_unscaled = unscale(freq, &Unit::from_str(freq_unit).unwrap());
    let w = 2.0 * PI * freq_unscaled;

    let (r, x) = match type_ {
        "sr" => (unscale(vals[0], &Unit::from_str(units[0]).unwrap()), 0.0),
        "pr" => (unscale(vals[0], &Unit::from_str(units[0]).unwrap()), 0.0),
        "sc" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = 1.0 / (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap()) * vals[0]);
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            (
                tmp,
                -1.0 / (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap())),
            )
        }
        "pc" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = 1.0 / (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap()) * vals[0]);
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            (
                tmp,
                -1.0 / (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap())),
            )
        }
        "si" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap())) / vals[0];
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            (
                tmp,
                w * unscale(vals[1], &Unit::from_str(units[1]).unwrap()),
            )
        }
        "pi" => {
            let mut tmp: f64 = 0.0;
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                tmp = 0.0;
            } else if units[0] == "Q" {
                tmp = (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap())) / vals[0];
            } else {
                tmp = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
            }
            (
                tmp,
                w * unscale(vals[1], &Unit::from_str(units[1]).unwrap()),
            )
        }
        "xfmr" => {
            let mut l1_tee: f64 = 0.0;
            let mut l2_tee: f64 = 0.0;
            let mut ls_tee: f64 = 0.0;
            let n: f64 = 0.0;
            let mut r1: f64 = 0.0;
            let mut r2: f64 = 0.0;
            let l1 = unscale(vals[1], &Unit::from_str(units[1]).unwrap());
            let mut l2 = unscale(vals[2], &Unit::from_str(units[2]).unwrap());
            let mut k = vals[3];
            if units[3] == "k" {
                l1_tee = (1.0 - k) * l1;
                l2_tee = (1.0 - k) * l2;
                ls_tee = k * l1;
            }
            if units[2] == "n" {
                let n = vals[2];
                l2 = n.powi(2) * l1;
                l2_tee = (1.0 - k) * l2;
            }
            if units[3] != "k" {
                let m = unscale(vals[3], &Unit::from_str(units[3]).unwrap());
                k = m / l1;
                l1_tee = (1.0 - k) * l1;
                l2_tee = (1.0 - k) * l2;
                ls_tee = m;
            }
            if units[0] == "Q" && !approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                r1 = w * l1 / vals[0];
                r2 = w * l2 / vals[0];
            } else {
                r1 = vals[0];
                r2 = vals[0];
            }
            let zout: Complex<f64> = (c64(r1, w * l1_tee).inv() + c64(0.0, w * ls_tee).inv()).inv()
                + c64(r2, w * l2_tee);
            (zout.re, zout.im)
        }
        "rlc" => (
            unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
            w * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                - 1.0 / (w * unscale(vals[2], &Unit::from_str(units[2]).unwrap())),
        ),
        "rl" => (
            unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
            w * unscale(vals[1], &Unit::from_str(units[1]).unwrap()),
        ),
        "rc" => (
            1.0 / unscale(vals[0], &Unit::from_str(units[0]).unwrap()),
            -1.0 / (w * unscale(vals[1], &Unit::from_str(units[1]).unwrap())),
        ),
        _ => return Err("element not recognized".to_string()),
    };

    if verbose {
        println!("[{:?}, {:?}]", r / z0, x / z0);
    }
    Ok(vec![r / z0, x / z0])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_ri_custom(
    lut: Vec<[f64; 3]>,
    interp: &str,
    freq: f64,
    freq_unit: &str,
    z0: f64,
    verbose: bool,
) -> Result<Vec<f64>, String> {
    if verbose {
        print!("\ncalc_ri_custom([");
        for i in 0..lut.len() {
            print!("{:?}", lut[i]);
            if i != lut.len() - 1 {
                print!(", ");
            }
        }
        println!("], {:?}, {:?}, {:?}, {:?})", interp, freq, freq_unit, z0);
    }

    let freq_unscaled = unscale(freq, &Unit::from_str(freq_unit).unwrap());

    let mut index_res: usize = lut.len() - 1;
    let mut r: f64 = 0.0;
    let mut x: f64 = 0.0;
    for i in 0..lut.len() {
        if lut[i][0] > freq_unscaled {
            index_res = i;
            break;
        }
    }

    if (index_res == lut.len() - 1) || (index_res == 0) {
        r = lut[index_res][1];
        x = lut[index_res][2];
    } else {
        let f1 = lut[index_res - 1][0];
        let f2 = lut[index_res][0];
        let frac = (freq_unscaled - f1) / (f2 - f1);
        r = lut[index_res - 1][1] + frac * (lut[index_res][1] - lut[index_res - 1][1]);
        x = lut[index_res - 1][2] + frac * (lut[index_res][2] - lut[index_res - 1][2]);
    }

    if verbose {
        println!("[{:?}, {:?}]", r / z0, x / z0);
    }
    Ok(vec![r / z0, x / z0])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_ri_tline(
    vals: Vec<f64>,
    units: Vec<&str>,
    line_z0: f64,
    type_: &str,
    freq: f64,
    freq_unit: &str,
    z0: f64,
    er: f64,
    verbose: bool,
) -> Result<Vec<f64>, String> {
    if verbose {
        print!("\ncalc_ri_tline([");
        for i in 0..vals.len() {
            print!("{:?}", vals[i]);
            if i != vals.len() - 1 {
                print!(", ");
            }
        }
        print!("], [");
        for i in 0..units.len() {
            print!("{:?}", units[i]);
            if i != units.len() - 1 {
                print!(", ");
            }
        }
        println!(
            "], {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            line_z0, type_, freq, freq_unit, z0, er
        );
    }

    let w = 2.0 * PI * unscale(freq, &Unit::from_str(freq_unit).unwrap());
    let betal = w * er.sqrt() * unscale(vals[0], &Unit::from_str(units[0]).unwrap());

    let (r, x) = match type_ {
        "tl" => {
            let zout: Complex<f64> = line_z0
                * ((c64(50.0, 0.0) + Complex::<f64>::I * line_z0 * betal.tan())
                    / (line_z0 + Complex::<f64>::I * c64(50.0, 0.0) * betal.tan()));
            (zout.re, zout.im)
        }
        "so" => {
            let betalinv = 1.0 / betal;
            let zout: Complex<f64> = -Complex::<f64>::I * line_z0 * betalinv.tan();
            (zout.re, zout.im)
        }
        "ss" => {
            let zout: Complex<f64> = Complex::<f64>::I * line_z0 * betal.tan();
            (zout.re, zout.im)
        }
        _ => return Err("element not recognized".to_string()),
    };

    let (rout, xout) = (r / z0, x / z0);
    if verbose {
        println!("[{:?}, {:?}]", rout, xout);
    }
    Ok(vec![rout, xout])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_z_to_gamma(re: f64, im: f64, z0: f64) -> Result<Vec<f64>, String> {
    let z: Complex<f64> = c64(re, im);

    let g = (z - z0) / (z + z0);

    Ok(vec![g.re, g.im, g.norm(), g.arg() * 180.0 / PI])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_from_str() {
        let sc = ["sc", "ser_cap", "series_cap", "series_capacitor"];
        let pc = ["pc", "shnt_cap", "shunt_cap", "shunt_capacitor"];
        let si = ["si", "ser_ind", "series_ind", "series_inductor"];
        let pi = ["pi", "shnt_ind", "shunt_ind", "shunt_inductor"];
        let sr = ["sr", "ser_res", "series_res", "series_resistor"];
        let pr = ["pr", "shnt_res", "shunt_res", "shunt_resistor"];
        let tl = ["tl", "tline", "transmission_line"];
        let os = ["os", "open_stub"];
        let ss = ["ss", "short_stub", "shorted_stub"];
        let xfmr = ["xfmr", "transformer"];
        let rlc = ["rlc", "res_ind_cap", "resistor_inductor_capacitor"];
        let cz = ["custom_z", "customZ"];
        let nada = ["", "google", ".sfwe"];

        for val in sc.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::SeriesCapacitor);
        }

        for val in pc.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::ShuntCapacitor);
        }

        for val in si.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::SeriesInductor);
        }

        for val in pi.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::ShuntInductor);
        }

        for val in sr.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::SeriesResistor);
        }

        for val in pr.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::ShuntResistor);
        }

        for val in tl.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::TransmissionLine);
        }

        for val in os.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::OpenStub);
        }

        for val in ss.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::ShortedStub);
        }

        for val in xfmr.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::Transformer);
        }

        for val in rlc.iter() {
            assert_eq!(
                Element::from_str(val).unwrap(),
                Element::ResistorInductorCapacitor
            );
        }

        for val in cz.iter() {
            assert_eq!(Element::from_str(val).unwrap(), Element::CustomZ);
        }
    }

    #[test]
    fn test_element_to_str() {
        let sc = ["sc", "ser_cap", "series_cap", "series_capacitor"];
        let pc = ["pc", "shnt_cap", "shunt_cap", "shunt_capacitor"];
        let si = ["si", "ser_ind", "series_ind", "series_inductor"];
        let pi = ["pi", "shnt_ind", "shunt_ind", "shunt_inductor"];
        let sr = ["sr", "ser_res", "series_res", "series_resistor"];
        let pr = ["pr", "shnt_res", "shunt_res", "shunt_resistor"];
        let tl = ["tl", "tline", "transmission_line"];
        let os = ["os", "open_stub"];
        let ss = ["ss", "short_stub", "shorted_stub"];
        let xfmr = ["xfmr", "transformer"];
        let rlc = ["rlc", "res_ind_cap", "resistor_inductor_capacitor"];
        let cz = ["custom_z", "customZ"];
        let nada = ["", "google", ".sfwe"];

        assert_eq!(
            Element::SeriesCapacitor.to_string(),
            "series_capacitor".to_string()
        );
        assert_eq!(
            Element::ShuntCapacitor.to_string(),
            "shunt_capacitor".to_string()
        );
        assert_eq!(
            Element::SeriesInductor.to_string(),
            "series_inductor".to_string()
        );
        assert_eq!(
            Element::ShuntInductor.to_string(),
            "shunt_inductor".to_string()
        );
        assert_eq!(
            Element::SeriesResistor.to_string(),
            "series_resistor".to_string()
        );
        assert_eq!(
            Element::ShuntResistor.to_string(),
            "shunt_resistor".to_string()
        );
        assert_eq!(
            Element::TransmissionLine.to_string(),
            "transmission_line".to_string()
        );
        assert_eq!(Element::OpenStub.to_string(), "open_stub".to_string());
        assert_eq!(Element::ShortedStub.to_string(), "shorted_stub".to_string());
        assert_eq!(Element::Transformer.to_string(), "transformer".to_string());
        assert_eq!(
            Element::ResistorInductorCapacitor.to_string(),
            "rlc".to_string()
        );
        assert_eq!(Element::CustomZ.to_string(), "custom_z".to_string());
    }

    #[test]
    fn test_find_smith_coord() {
        let real: f64 = 1.0;
        let imaginary: f64 = 0.0;
        let rotate = true;
        let ix: f64 = 0.0;
        let iy: f64 = 0.0005;
        let exemplar: Vec<f64> = vec![ix, iy];
        let test = find_smith_coord_int(real, imaginary, rotate, false).unwrap();
        comp_vec_f64(
            test,
            exemplar,
            F64Margin::from((1e-5, 1)),
            "find_smith_coord()",
            "",
        );

        let real = 0.7490939362604676;
        let imaginary = 0.43353455562188586;
        let rotate = true;
        let ix = 0.07727;
        let iy = -0.26701;
        let exemplar: Vec<f64> = vec![ix, iy];
        let test = find_smith_coord_int(real, imaginary, rotate, false).unwrap();
        comp_vec_f64(
            test,
            exemplar,
            F64Margin::from((1e-5, 1)),
            "find_smith_coord()",
            "",
        );
    }

    #[test]
    fn test_arc_smith_points() {
        let x1: f64 = 1.0;
        let y1: f64 = 0.0;
        let x2: f64 = 1.1202712484636101;
        let y2: f64 = -2.4054249692722034;
        let type_ = "pi";
        let rotate = true;
        let beta: f64 = NAN;
        let start_at_qtr_wl: f64 = 0.0;
        let z0: f64 = 100.0;
        let resolution: usize = 100;
        let exemplar = ArcReturn {
            x_coord: vec![
                0.0, -0.00075, -0.00178, -0.00309, -0.00469, -0.00657, -0.00872, -0.01114,
                -0.01383, -0.01678, -0.01998, -0.02344, -0.02714, -0.03108, -0.03525, -0.03964,
                -0.04425, -0.04907, -0.0541, -0.05932, -0.06472, -0.07031, -0.07606, -0.08199,
                -0.08806, -0.09429, -0.10065, -0.10715, -0.11376, -0.1205, -0.12734, -0.13428,
                -0.14131, -0.14843, -0.15563, -0.16289, -0.17022, -0.17761, -0.18505, -0.19252,
                -0.20004, -0.20758, -0.21515, -0.22274, -0.23035, -0.23796, -0.24557, -0.25318,
                -0.26079, -0.26838, -0.27596, -0.28352, -0.29106, -0.29856, -0.30604, -0.31349,
                -0.32089, -0.32826, -0.33559, -0.34287, -0.3501, -0.35728, -0.36441, -0.37148,
                -0.3785, -0.38547, -0.39237, -0.39921, -0.40599, -0.41271, -0.41937, -0.42596,
                -0.43248, -0.43894, -0.44533, -0.45165, -0.4579, -0.46409, -0.47021, -0.47626,
                -0.48224, -0.48815, -0.49399, -0.49976, -0.50546, -0.5111, -0.51666, -0.52216,
                -0.52759, -0.53295, -0.53824, -0.54347, -0.54863, -0.55372, -0.55875, -0.56371,
                -0.56861, -0.57344, -0.57821, -0.58292, -0.58756,
            ],
            y_coord: vec![
                0.0005, 0.01201, 0.02398, 0.0359, 0.04777, 0.05956, 0.07128, 0.0829, 0.09443,
                0.10585, 0.11716, 0.12835, 0.1394, 0.15032, 0.16109, 0.17171, 0.18217, 0.19246,
                0.20258, 0.21253, 0.2223, 0.23188, 0.24128, 0.25048, 0.25949, 0.26829, 0.2769,
                0.28531, 0.29351, 0.3015, 0.30929, 0.31687, 0.32424, 0.33141, 0.33836, 0.34512,
                0.35166, 0.358, 0.36414, 0.37007, 0.37581, 0.38135, 0.38669, 0.39184, 0.3968,
                0.40157, 0.40615, 0.41055, 0.41478, 0.41882, 0.4227, 0.4264, 0.42994, 0.43331,
                0.43653, 0.43958, 0.44249, 0.44525, 0.44786, 0.45033, 0.45265, 0.45485, 0.45691,
                0.45885, 0.46066, 0.46235, 0.46392, 0.46537, 0.46672, 0.46796, 0.46909, 0.47012,
                0.47105, 0.47189, 0.47263, 0.47328, 0.47385, 0.47434, 0.47474, 0.47506, 0.47531,
                0.47548, 0.47559, 0.47562, 0.47559, 0.4755, 0.47535, 0.47513, 0.47486, 0.47454,
                0.47416, 0.47373, 0.47326, 0.47273, 0.47217, 0.47155, 0.4709, 0.47021, 0.46948,
                0.46871, 0.46791,
            ],
            end_x_coord: -0.58756,
            end_y_coord: 0.46791,
            real_old: 0.0,
            imag_old: 0.0,
            start_x_coord: 0.0,
            start_y_coord: 0.0005,
            x1: 1.0,
            y1: 0.0,
            x2: 1.1202712484636101,
            y2: -2.4054249692722034,
        };
        let test = arc_smith_points(
            x1,
            y1,
            x2,
            y2,
            type_,
            rotate,
            beta,
            start_at_qtr_wl,
            z0,
            resolution,
            false,
        )
        .unwrap();
        assert_eq!(&test.x_coord, &exemplar.x_coord);
        assert_eq!(&test.y_coord, &exemplar.y_coord);
        assert_eq!(&test.end_x_coord, &exemplar.end_x_coord);
        assert_eq!(&test.end_y_coord, &exemplar.end_y_coord);
        assert_eq!(&test.real_old, &exemplar.real_old);
        assert_eq!(&test.imag_old, &exemplar.imag_old);
        assert_eq!(&test.start_x_coord, &exemplar.start_x_coord);
        assert_eq!(&test.start_y_coord, &exemplar.start_y_coord);
        assert_eq!(&test.x1, &exemplar.x1);
        assert_eq!(&test.y1, &exemplar.y1);
        assert_eq!(&test.x2, &exemplar.x2);
        assert_eq!(&test.y2, &exemplar.y2);
    }
}
