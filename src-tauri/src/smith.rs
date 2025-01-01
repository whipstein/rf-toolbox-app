#![allow(unused)]
use crate::element::{
    blackbox::BlackBox, capacitor::Capacitor, inductor::Inductor, openstub::OpenStub,
    resistor::Resistor, rlc::Rlc, shortedstub::ShortedStub, tline::TLine, transformer::Transformer,
    Element, Orientation,
};
use crate::frequency::Frequency;
use crate::rf_utils::{calc_z, comp_c64, comp_f64, comp_vec_f64, scale, unscale, ComplexReturn};
use crate::unit::Unit;
use float_cmp::{approx_eq, F64Margin};
use num_complex::{c64, Complex};
use serde::Serialize;
use std::error::Error;
use std::f64::consts::PI;
use std::f64::{EPSILON, NAN};
use std::str::FromStr;
use std::string::ToString;

// #[derive(Serialize)]
// pub(crate) struct SmithState {
//     schematic: Vec<Box<dyn Element>>,
//     freq: Frequency,
//     span: Frequency,
//     z0: f64,
//     er: f64,
// }

// impl Default for SmithState {
//     fn default() -> Self {
//         Self {
//             schematic: vec![BlackBox::default()],
//             freq: Frequency::new(280.0, Unit::Giga),
//             span: Frequency::new(0.0, Unit::Giga),
//             z0: 50.0,
//             er: 1.0,
//         }
//     }
// }

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

pub fn find_smith_coord(
    re: f64,
    im: f64,
    rotate: bool,
    verbose: bool,
) -> Result<Complex<f64>, String> {
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
    Ok(g)
}

pub fn find_smith_coord_c64(
    val: Complex<f64>,
    rotate: bool,
    verbose: bool,
) -> Result<Complex<f64>, String> {
    if verbose {
        println!("\nfind_smith_coord_c64({:?}, {:?})", val, rotate);
    }

    find_smith_coord(val.re, val.im, rotate, verbose)
}

#[tauri::command(rename_all = "snake_case")]
pub fn find_smith_coord_js(
    re: f64,
    im: f64,
    rotate: bool,
    verbose: bool,
) -> Result<Vec<f64>, String> {
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
    if verbose {
        println!("x1 = {}, y1 = {}", x1, y1);
    }
    let mut temp_array = find_smith_coord(x1, y1, rotate, false).unwrap();
    let start_x_coord: f64 = temp_array.re;
    let start_y_coord: f64 = temp_array.im;
    let mut real_old: f64 = 0.0;
    let mut imag_old: f64 = 0.0;
    let mut stub_admittance_im: f64 = 0.0;
    let mut real_answer: f64 = 0.0;
    let mut imag_answer: f64 = 0.0;

    //used for transmission lines and stubs
    let line_zo = y2;
    let line_length = x2;
    let zl: Complex<f64> = calc_z(Complex::new(start_x_coord, start_y_coord), z0);

    for i in 0..=resolution {
        if type_ == "transmission_line" {
            let betal = (beta * (i as f64) * line_length) / (resolution as f64);
            let zi = line_zo
                * ((zl + Complex::<f64>::I * line_zo * betal.tan())
                    / (line_zo + Complex::<f64>::I * zl * betal.tan()))
                / z0;
            temp_array = find_smith_coord_c64(zi, rotate, false).unwrap();

            x_coord[i] = temp_array.re;
            y_coord[i] = temp_array.im;
            real_answer = zi.re;
            imag_answer = zi.im;
        } else if type_ == "ss" {
            let tan_beta = match approx_eq!(f64, start_at_qtr_wl, 0_f64, F64Margin::default()) {
                true => ((beta * (i as f64) * line_length) / (resolution as f64)).tan(),
                false => (beta
                    * ((start_at_qtr_wl as f64)
                        + ((i as f64) * (line_length - (start_at_qtr_wl as f64)))
                            / (resolution as f64)))
                    .tan(),
            };
            stub_admittance_im = -1.0 / ((tan_beta * line_zo) / z0);
            temp_array = find_smith_coord(x1, y1 + stub_admittance_im, rotate, false).unwrap();
            x_coord[i] = temp_array.re;
            y_coord[i] = temp_array.im;
        } else if type_ == "so" {
            let tan_beta_arg = (beta * (i as f64) * line_length) / (resolution as f64);
            stub_admittance_im = tan_beta_arg.tan() / (line_zo / z0);
            temp_array = find_smith_coord(x1, y1 + stub_admittance_im, rotate, false).unwrap();
            x_coord[i] = temp_array.re;
            y_coord[i] = temp_array.im;
        } else {
            temp_array = find_smith_coord(
                x1 + ((x2 - x1) * (i as f64)) / (resolution as f64),
                y1 + ((y2 - y1) * (i as f64)) / (resolution as f64),
                rotate,
                false,
            )
            .unwrap();
            x_coord[i] = temp_array.re;
            y_coord[i] = temp_array.im;
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

    let end_x_coord = temp_array.re;
    let end_y_coord = temp_array.im;

    if verbose {
        println!("=");
        print!("[xpts = [");
        for i in 0..x_coord.len() {
            print!("{}", x_coord[i]);
            if i != x_coord.len() - 1 {
                print!(", ");
            }
        }
        print!("]\nypts = [");
        for i in 0..y_coord.len() {
            print!("{}", y_coord[i]);
            if i != x_coord.len() - 1 {
                print!(", ");
            }
        }
        println!(
            "]\n{}, {}, {}, {}, {}, {}, {}, {}, {}, {}]",
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

    let mut re: f64 = 0.0;
    let mut im: f64 = 0.0;
    let mut ln: f64 = 0.0;

    match type_ {
        "bb" => {
            re = vals[0] / z0;
            im = vals[1] / z0;
        }
        "sr" | "pr" => {
            re = unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0;
        }
        "sc" | "pc" => {
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                re = 0.0;
            } else if units[0] == "Q" {
                re = 1.0
                    / (2.0
                        * PI
                        * freq
                        * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                        * vals[0])
                    / z0;
            } else {
                re = unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0;
            }
            im = -1.0
                / (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                / z0;
        }
        "si" | "pi" => {
            if approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                re = 0.0;
            } else if units[0] == "Q" {
                re = (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                    / vals[0]
                    / z0;
            } else {
                re = unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0;
            }
            im = 2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()) / z0;
        }
        "xfmr" => {
            let mut l1_tee: f64 = 0.0;
            let mut l2_tee: f64 = 0.0;
            let mut ls_tee: f64 = 0.0;
            // let n: f64 = 0.0;
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
            let (z1, z2, z3) =
                match units[0] == "Q" && !approx_eq!(f64, vals[0], 0_f64, F64Margin::default()) {
                    true => (
                        c64(
                            2.0 * PI * freq * l1 / vals[0] / z0,
                            2.0 * PI * freq * l1_tee,
                        ),
                        c64(0.0, 2.0 * PI * freq * ls_tee),
                        c64(
                            2.0 * PI * freq * l2 / vals[0] / z0,
                            2.0 * PI * freq * l2_tee,
                        ),
                    ),
                    false => (
                        c64(vals[0], 2.0 * PI * freq * l1_tee),
                        c64(0.0, 2.0 * PI * freq * ls_tee),
                        c64(vals[0], 2.0 * PI * freq * l2_tee),
                    ),
                };
            let z = (z1.inv() + z2.inv()).inv() + z3;
            re = z.re;
            im = z.im;
        }
        "rlc" => {
            re = unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0;
            im = (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap())
                - 1.0 / (2.0 * PI * freq * unscale(vals[2], &Unit::from_str(units[2]).unwrap())))
                / z0;
        }
        "rl" => {
            re = unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0;
            im = 2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()) / z0;
        }
        "rc" => {
            re = 1.0 / unscale(vals[0], &Unit::from_str(units[0]).unwrap()) / z0;
            im = -1.0
                / (2.0 * PI * freq * unscale(vals[1], &Unit::from_str(units[1]).unwrap()))
                / z0;
        }
        "tl" | "so" | "ss" => {
            ln = unscale(vals[0], &Unit::from_str(units[0]).unwrap());
        }
        "customZ" => {
            let mut index_res: usize = lut.len() - 1;
            for i in 0..lut.len() {
                if lut[i][0] > freq {
                    index_res = i;
                    break;
                }
            }

            if (index_res == lut.len() - 1) || (index_res == 0) {
                re = lut[index_res][1];
                im = lut[index_res][2];
            } else {
                let f1 = lut[index_res - 1][0];
                let f2 = lut[index_res][0];
                let frac = (freq - f1) / (f2 - f1);
                re = lut[index_res - 1][1] + frac * (lut[index_res][1] - lut[index_res - 1][1]);
                im = lut[index_res - 1][2] + frac * (lut[index_res][2] - lut[index_res - 1][2]);
            }

            re /= z0;
            im /= z0;
        }
        _ => return Err("element not recognized".to_string()),
    }

    if diff && type_ == "bb" {
        re /= 2.0;
        im /= 2.0;
    }

    if verbose {
        println!("[{:?}, {:?}, {:?}]", re, im, ln);
    }
    Ok(vec![re, im, ln])
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_const_q(q: f64, npts: usize) -> Vec<(f64, f64)> {
    let mut gpts: Vec<(f64, f64)> = vec![];
    let step = (20_f64).ln() / (npts as f64);
    for i in 0..npts {
        let val = ((i as f64) * step).exp() - 1.0;
        gpts.push((val, val * q));
    }
    gpts.push((1e10, 0.0));
    for i in npts..0 {
        let val = ((i as f64) * step).exp() - 1.0;
        gpts.push((val, -val * q));
    }

    gpts
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_vswr_circle(vswr: f64, zl: Complex<f64>, _z0: f64) -> (Complex<f64>, f64) {
    let center = find_smith_coord_c64(zl, false, false).unwrap();
    let radius = (vswr - 1.0) / (vswr + 1.0) + center.norm();
    (center, radius)
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_smith_arc(
    element: &str,
    vals: Vec<f64>,
    units: Vec<&str>,
    rin: f64,
    xin: f64,
    z0: f64,
    freq: f64,
    freq_unit: &str,
    npts: usize,
    verbose: bool,
) -> Result<(Vec<f64>, Vec<f64>, (f64, f64), (f64, f64)), String> {
    if verbose {
        print!("calc_smith_arc(element: {:?}, vals: [", element);
        for val in vals.iter() {
            print!("{:?}, ", val);
        }
        print!("], [");
        for val in units.iter() {
            print!("{:?}, ", val);
        }
        println!(
            "], {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            rin, xin, z0, freq, freq_unit, npts
        );
    }

    let freq_int = Frequency::new(freq, Unit::from_str(freq_unit).unwrap());
    let zin = c64(rin, xin);

    match element {
        "si" => Ok(Inductor::new(
            vals[0],
            vals[1],
            Unit::from_str(units[0]).unwrap(),
            Unit::from_str(units[1]).unwrap(),
            0.0,
            0.0,
            Orientation::Series,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "pi" => Ok(Inductor::new(
            vals[0],
            vals[1],
            Unit::from_str(units[0]).unwrap(),
            Unit::from_str(units[1]).unwrap(),
            0.0,
            0.0,
            Orientation::Shunt,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "sc" => Ok(Capacitor::new(
            vals[0],
            vals[1],
            Unit::from_str(units[0]).unwrap(),
            Unit::from_str(units[1]).unwrap(),
            0.0,
            0.0,
            Orientation::Series,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "pc" => Ok(Capacitor::new(
            vals[0],
            vals[1],
            Unit::from_str(units[0]).unwrap(),
            Unit::from_str(units[1]).unwrap(),
            0.0,
            0.0,
            Orientation::Shunt,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "sr" => Ok(Resistor::new(
            vals[0],
            Unit::from_str(units[0]).unwrap(),
            0.0,
            Orientation::Series,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "pr" => Ok(Resistor::new(
            vals[0],
            Unit::from_str(units[0]).unwrap(),
            0.0,
            Orientation::Shunt,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "rlc" => Ok(Rlc::new(
            vals[0],
            vals[1],
            vals[2],
            Unit::from_str(units[0]).unwrap(),
            Unit::from_str(units[1]).unwrap(),
            Unit::from_str(units[2]).unwrap(),
            0.0,
            0.0,
            0.0,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "bb" => {
            if units[0] == "diff" {
                Ok(BlackBox::from_ri(vals[0] / 2.0, vals[1] / 2.0, z0, 0.0)
                    .calc_arc(freq_int, zin, z0, npts, verbose))
            } else {
                Ok(BlackBox::from_ri(vals[0], vals[1], z0, 0.0)
                    .calc_arc(freq_int, zin, z0, npts, verbose))
            }
        }
        "tl" => Ok(TLine::new(
            vals[0],
            zin * z0,
            vals[1],
            vals[2],
            Unit::from_str(units[0]).unwrap(),
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "ss" => Ok(
            ShortedStub::new(vals[0], vals[1], vals[2], Unit::from_str(units[0]).unwrap())
                .calc_arc(freq_int, zin, z0, npts, verbose),
        ),
        "so" => Ok(OpenStub::new(
            vals[0],
            zin,
            vals[1],
            vals[2],
            Unit::from_str(units[0]).unwrap(),
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        "xfmr" => Ok(Transformer::new(
            vals[0],
            vals[1],
            vals[2],
            vals[3],
            Unit::from_str(units[0]).unwrap(),
            Unit::from_str(units[1]).unwrap(),
            Unit::from_str(units[2]).unwrap(),
            Unit::from_str(units[3]).unwrap(),
            0.0,
            0.0,
            0.0,
            0.0,
        )
        .calc_arc(freq_int, zin, z0, npts, verbose)),
        _ => return Err("element not recognize".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smith_coord() {
        let real: f64 = 1.0;
        let imaginary: f64 = 0.0;
        let rotate = true;
        let ix: f64 = 0.0;
        let iy: f64 = 0.0;
        let exemplar: Complex<f64> = c64(ix, iy);
        let test = find_smith_coord(real, imaginary, rotate, false).unwrap();
        comp_c64(
            &test,
            &exemplar,
            F64Margin::from((1e-5, 1)),
            "find_smith_coord()",
            "",
        );

        let real = 0.7490939362604676;
        let imaginary = 0.43353455562188586;
        let rotate = true;
        let ix = 0.07727;
        let iy = -0.26701;
        let exemplar: Complex<f64> = c64(ix, iy);
        let test = find_smith_coord(real, imaginary, rotate, false).unwrap();
        comp_c64(
            &test,
            &exemplar,
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
                0.0, 0.01201, 0.02398, 0.0359, 0.04777, 0.05956, 0.07128, 0.0829, 0.09443, 0.10585,
                0.11716, 0.12835, 0.1394, 0.15032, 0.16109, 0.17171, 0.18217, 0.19246, 0.20258,
                0.21253, 0.2223, 0.23188, 0.24128, 0.25048, 0.25949, 0.26829, 0.2769, 0.28531,
                0.29351, 0.3015, 0.30929, 0.31687, 0.32424, 0.33141, 0.33836, 0.34512, 0.35166,
                0.358, 0.36414, 0.37007, 0.37581, 0.38135, 0.38669, 0.39184, 0.3968, 0.40157,
                0.40615, 0.41055, 0.41478, 0.41882, 0.4227, 0.4264, 0.42994, 0.43331, 0.43653,
                0.43958, 0.44249, 0.44525, 0.44786, 0.45033, 0.45265, 0.45485, 0.45691, 0.45885,
                0.46066, 0.46235, 0.46392, 0.46537, 0.46672, 0.46796, 0.46909, 0.47012, 0.47105,
                0.47189, 0.47263, 0.47328, 0.47385, 0.47434, 0.47474, 0.47506, 0.47531, 0.47548,
                0.47559, 0.47562, 0.47559, 0.4755, 0.47535, 0.47513, 0.47486, 0.47454, 0.47416,
                0.47373, 0.47326, 0.47273, 0.47217, 0.47155, 0.4709, 0.47021, 0.46948, 0.46871,
                0.46791,
            ],
            end_x_coord: -0.58756,
            end_y_coord: 0.46791,
            real_old: 0.0,
            imag_old: 0.0,
            start_x_coord: 0.0,
            start_y_coord: 0.0,
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
        comp_vec_f64(
            test.x_coord,
            exemplar.x_coord,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "x_coord",
        );
        comp_vec_f64(
            test.y_coord,
            exemplar.y_coord,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "y_coord",
        );
        comp_f64(
            &test.end_x_coord,
            &exemplar.end_x_coord,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "end_x_coord",
        );
        comp_f64(
            &test.end_y_coord,
            &exemplar.end_y_coord,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "end_y_coord",
        );
        comp_f64(
            &test.real_old,
            &exemplar.real_old,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "real_old",
        );
        comp_f64(
            &test.imag_old,
            &exemplar.imag_old,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "imag_old",
        );
        comp_f64(
            &test.start_x_coord,
            &exemplar.start_x_coord,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "start_x_coord",
        );
        comp_f64(
            &test.start_y_coord,
            &exemplar.start_y_coord,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "start_y_coord",
        );
        comp_f64(
            &test.x1,
            &exemplar.x1,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "x1",
        );
        comp_f64(
            &test.y1,
            &exemplar.y1,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "y1",
        );
        comp_f64(
            &test.x2,
            &exemplar.x2,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "x2",
        );
        comp_f64(
            &test.y2,
            &exemplar.y2,
            F64Margin::from((1e-5, 1)),
            "arc_smith_points()",
            "y2",
        );
    }

    #[test]
    fn test_calc_smith_arc_shunt_l_series_c() {
        let testname = "calc_smith_arc_shuntL_seriesC";
        let _freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        let npts = 10;

        let margin = F64Margin::from((1e-15, 1));

        let element = "pi";
        let vals = vec![20.0, 10.0];
        let units = vec!["Q", "pH"];
        let rin = 1.0;
        let xin = 0.0;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.6606893070885922;
        let _end_y_coord = 0.44913494554545125;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = 0.0;
        let _start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.1417482571178263;
        let y2 = -2.8349651423565256;
        let x_coord = vec![
            0.0,
            -0.026326741419400255,
            -0.0854398444401545,
            -0.16550566724301574,
            -0.2542701170487083,
            -0.3423475317367525,
            -0.4240800012974579,
            -0.49688364532064383,
            -0.5601737023676274,
            -0.6144499256284348,
            -0.6606893070885922,
        ];
        let y_coord = vec![
            0.0,
            0.13704519155357964,
            0.25565081047842475,
            0.3474762292461895,
            0.4111671980155597,
            0.45015334288049136,
            0.46983447110827475,
            0.47561491021893504,
            0.4719950960958993,
            0.46236661203209817,
            0.44913494554545125,
        ];

        comp_f64(&test.2 .0, &x1, margin, testname, "x1");
        comp_f64(&test.2 .1, &y1, margin, testname, "y1");
        comp_f64(&test.3 .0, &x2, margin, testname, "x2");
        comp_f64(&test.3 .1, &y2, margin, testname, "y2");
        comp_vec_f64(test.0, x_coord, margin, testname, "x_coord");
        comp_vec_f64(test.1, y_coord, margin, testname, "y_coord");

        let element = "sc";
        let vals = vec![0.0, 20.0];
        let units = vec!["Q", "fF"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.6880997486248435;
        let _end_y_coord = -0.3984722795985554;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.6606893070885922;
        let _start_y_coord = 0.44913494554545125;
        let x1 = 0.12223478657203007;
        let y1 = 0.303509426841484;
        let x2 = 0.12223478657203007;
        let y2 = -0.26490108420099917;
        let x_coord = vec![
            -0.6606893070885922,
            -0.7000257728128035,
            -0.7325853297384076,
            -0.7574787699336148,
            -0.7739911378038796,
            -0.7816310924678345,
            -0.7801666308121229,
            -0.7696424543594921,
            -0.7503765984209411,
            -0.7229367901478022,
            -0.6880997486248435,
        ];
        let y_coord = vec![
            0.44913494554545125,
            0.37366743671535424,
            0.2930688317435207,
            0.20826350708060348,
            0.12036781552616903,
            0.030646806042723088,
            -0.05954360140662473,
            -0.14882375350760965,
            -0.23585988465500587,
            -0.3194289553942221,
            -0.3984722795985554,
        ];

        comp_f64(&test.2 .0, &x1, margin, testname, "x1");
        comp_f64(&test.2 .1, &y1, margin, testname, "y1");
        comp_f64(&test.3 .0, &x2, margin, testname, "x2");
        comp_f64(&test.3 .1, &y2, margin, testname, "y2");
        comp_vec_f64(test.0, x_coord, margin, testname, "x_coord");
        comp_vec_f64(test.1, y_coord, margin, testname, "y_coord");
    }

    #[test]
    fn test_calc_smith_arc_shunt_l_series_tl() {
        let testname = "calc_smith_arc_shuntL_seriesTL";
        let _freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        let npts = 10;

        let margin = F64Margin::from((1e-13, 1));

        let element = "pi";
        let vals = vec![20.0, 10.0];
        let units = vec!["Q", "pH"];
        let rin = 1.0;
        let xin = 0.0;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.6606893070885922;
        let _end_y_coord = 0.44913494554545125;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = 0.0;
        let _start_y_coord = 0.0;
        let x1 = 1.0;
        let y1 = 0.0;
        let x2 = 1.1417482571178263;
        let y2 = -2.8349651423565256;
        let x_coord = vec![
            0.0,
            -0.026326741419400255,
            -0.0854398444401545,
            -0.16550566724301574,
            -0.2542701170487083,
            -0.3423475317367525,
            -0.4240800012974579,
            -0.49688364532064383,
            -0.5601737023676274,
            -0.6144499256284348,
            -0.6606893070885922,
        ];
        let y_coord = vec![
            0.0,
            0.13704519155357964,
            0.25565081047842475,
            0.3474762292461895,
            0.4111671980155597,
            0.45015334288049136,
            0.46983447110827475,
            0.47561491021893504,
            0.4719950960958993,
            0.46236661203209817,
            0.44913494554545125,
        ];

        comp_f64(&test.2 .0, &x1, margin, testname, "x1");
        comp_f64(&test.2 .1, &y1, margin, testname, "y1");
        comp_f64(&test.3 .0, &x2, margin, testname, "x2");
        comp_f64(&test.3 .1, &y2, margin, testname, "y2");
        comp_vec_f64(test.0, x_coord, margin, testname, "x_coord");
        comp_vec_f64(test.1, y_coord, margin, testname, "y_coord");

        let element = "tl";
        let vals = vec![90.0, 9.7, 100.0];
        let units = vec!["um"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = 0.8124103144077774;
        let _end_y_coord = -0.43234225147933797;
        let _real_old = 0.689162382270243;
        let _imag_old = -3.8930512898903054;
        let _start_x_coord = -0.6606893070885922;
        let _start_y_coord = 0.44913494554545125;
        let x1 = 0.12223478657203007;
        let y1 = 0.303509426841484;
        let x2 = 0.689162382270243;
        let y2 = -3.8930512898903054;
        let x_coord = vec![
            -0.6606893070885922,
            -0.3224224749848019,
            0.05707124097140867,
            0.3759710419605837,
            0.610570859366689,
            0.7700761602077888,
            0.8693071122075385,
            0.9196226531870995,
            0.9271493646484471,
            0.8928171981502225,
            0.8124103144077774,
        ];
        let y_coord = vec![
            0.44913494554545125,
            0.7630199424187774,
            0.8582903944845813,
            0.8023369939140537,
            0.6674866714270362,
            0.4979110977085203,
            0.3151129964560999,
            0.12811542713971788,
            -0.05983118729374963,
            -0.24738480230589666,
            -0.43234225147933797,
        ];

        comp_f64(&test.2 .0, &x1, margin, testname, "x1");
        comp_f64(&test.2 .1, &y1, margin, testname, "y1");
        comp_f64(&test.3 .0, &x2, margin, testname, "x2");
        comp_f64(&test.3 .1, &y2, margin, testname, "y2");
        comp_vec_f64(test.0, x_coord, margin, testname, "x_coord");
        comp_vec_f64(test.1, y_coord, margin, testname, "y_coord");
    }

    #[test]
    fn test_calc_smith_arc_multi() {
        let testname = "calc_smith_arc_multi";
        let _freq = Frequency::new(280.0, Unit::Giga);
        let z0 = 50.0;
        let npts = 10;
        let zstart = c64(67.0, -45.0);

        let margin = F64Margin::from((1e-13, 1));

        // "pc" -> "si" -> "pi" -> "sc" -> "tl" -> "sr" -> "so" -> "pr" -> "ss" -> "xfmr"
        // pc = [0.0, 20.0]
        // si = [20.0, 10.0]
        // pi = [14.0, 23.0]
        // sc = [25.0, 45.0]
        // tl = [75.0, 1.0, 76.0]
        // sr = [20.0]
        // so = [30.0, 1.0, 100.0]
        // pr = [50.0]
        // ss = [88.0, 1.0, 90.0]
        // xfmr = [25.0, 14.0, 34.0, 0.65]

        let element = "pc";
        let vals = vec![0.0, 20.0];
        let units = vec!["Q", "fF"];
        let rin = zstart.re / 100.0;
        let xin = zstart.im / 100.0;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.5990238691961544;
        let _end_y_coord = -0.48430376967689825;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.11653406431771074;
        let _start_y_coord = -0.30086247242094;
        let x1 = 1.028553883942278;
        let y1 = 0.6908197727970524;
        let x2 = 1.028553883942278;
        let y2 = 2.4501116588073373;
        let x_coord = vec![
            -0.11653406431771074,
            -0.16628227016882302,
            -0.22011820519090808,
            -0.275520959946788,
            -0.33048408371393156,
            -0.3835488161610348,
            -0.4337544092816007,
            -0.480547969179627,
            -0.5236849306326076,
            -0.5631382156138675,
            -0.5990238691961544,
        ];
        let y_coord = vec![
            -0.30086247242094,
            -0.35622616788763617,
            -0.4008598013917324,
            -0.43521420942247374,
            -0.46026107967563745,
            -0.4772441379784015,
            -0.4874844203544678,
            -0.4922497558040728,
            -0.49268085274416906,
            -0.4897594388881712,
            -0.48430376967689825,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "si";
        let vals = vec![20.0, 10.0];
        let units = vec!["Q", "pH"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.7192753998677598;
        let _end_y_coord = 0.007190005120377053;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.5990238691961544;
        let _start_y_coord = -0.48430376967689825;
        let x1 = 0.145667500362048;
        let y1 = -0.34699362524249144;
        let x2 = 0.16326041922215084;
        let y2 = 0.004864751959565372;
        let x_coord = vec![
            -0.5990238691961544,
            -0.6231670354586387,
            -0.6450451760055558,
            -0.6644586071920592,
            -0.6812254316833684,
            -0.6951857794118175,
            -0.7062056863419697,
            -0.7141804394354806,
            -0.719037233063089,
            -0.7207370102462435,
            -0.7192753998677598,
        ];
        let y_coord = vec![
            -0.48430376967689825,
            -0.44108794175345534,
            -0.39598078186097685,
            -0.3491568090149327,
            -0.30081704030781253,
            -0.2511867069543894,
            -0.2005121742666459,
            -0.14905713613983607,
            -0.09709819637857774,
            -0.044919986069100644,
            0.007190005120377053,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "pi";
        let vals = vec![14.0, 23.0];
        let units = vec!["Q", "pH"];
        let zin = c64(test.3 .0, test.3 .1);
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.7327663256075981;
        let _end_y_coord = 0.05234343800338552;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.7192753998677598;
        let _start_y_coord = 0.007190005120377053;
        let x1 = 6.119749670767431;
        let y1 = -0.18235322648783275;
        let x2 = 6.207564139478474;
        let y2 = -1.4117557884424423;
        let x_coord = vec![
            -0.7192753998677598,
            -0.7199509433336152,
            -0.7207876246925095,
            -0.7217819318855513,
            -0.7229298592198888,
            -0.7242269424735646,
            -0.725668296945297,
            -0.727248657880011,
            -0.7289624226878632,
            -0.7308036943739945,
            -0.7327663256075981,
        ];
        let y_coord = vec![
            0.007190005120377053,
            0.011993656255536448,
            0.01675254633147198,
            0.021458794673041518,
            0.026104847190003187,
            0.030683512827323945,
            0.03518799575966438,
            0.039611923096216986,
            0.043949367944339554,
            0.04819486776154369,
            0.05234343800338552,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "sc";
        let vals = vec![25.0, 45.0];
        let units = vec!["Q", "fF"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.6610576860592663;
        let _end_y_coord = -0.3109878504089656;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.7327663256075981;
        let _start_y_coord = 0.05234343800338552;
        let x1 = 0.15317142625367175;
        let y1 = 0.03483502429920493;
        let x2 = 0.16327650200553812;
        let y2 = -0.21779186949745427;
        let x_coord = vec![
            -0.7327663256075981,
            -0.7327098812673564,
            -0.7309939279184069,
            -0.7276326374602337,
            -0.7226494436744408,
            -0.7160766910599349,
            -0.7079551646250414,
            -0.6983335129527032,
            -0.6872675792235275,
            -0.6748196566659829,
            -0.6610576860592663,
        ];
        let y_coord = vec![
            0.05234343800338552,
            0.014370420135466428,
            -0.02351115478591295,
            -0.06119324916393789,
            -0.09856997025650453,
            -0.13553854316531977,
            -0.17200022083109828,
            -0.2078611147823366,
            -0.24303293298292608,
            -0.27743361406792005,
            -0.3109878504089656,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "tl";
        let vals = vec![75.0, 1.0, 76.0];
        let units = vec!["um"];
        let zin = c64(test.3 .0, test.3 .1);
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.4776979579337571;
        let _end_y_coord = 0.5759630634132183;
        let _real_old = 0.1749560244220851;
        let _imag_old = 0.4579631903587547;
        let _start_x_coord = -0.6610576860592663;
        let _start_y_coord = -0.3109878504089656;
        let x1 = 0.16327650200553812;
        let y1 = -0.21779186949745427;
        let x2 = 0.1749560244220851;
        let y2 = 0.4579631903587547;
        let x_coord = vec![
            -0.6610576860592663,
            -0.6934047506872917,
            -0.7145516247502989,
            -0.7238853726437542,
            -0.7211307100175813,
            -0.7063691851598313,
            -0.6800334653458259,
            -0.6428774667119078,
            -0.5959261164252068,
            -0.5404108322899207,
            -0.4776979579337571,
        ];
        let y_coord = vec![
            -0.3109878504089656,
            -0.2197165106614559,
            -0.12440955744087188,
            -0.026779167776941858,
            0.07135500639917552,
            0.16814969598363008,
            0.26182417977078326,
            0.35074037072388703,
            0.43346943103581587,
            0.5088396482193712,
            0.5759630634132183,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "sr";
        let vals = vec![20.0];
        let units = vec!["Ω"];
        let zin = c64(test.3 .0, test.3 .1);
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.1708766715939743;
        let _end_y_coord = 0.3404656433099943;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.4776979579337571;
        let _start_y_coord = 0.5759630634132183;
        let x1 = 0.1749560244220851;
        let y1 = 0.4579631903587547;
        let x2 = 0.5749560244220852;
        let y2 = 0.4579631903587547;
        let x_coord = vec![
            -0.4776979579337571,
            -0.4413585544262203,
            -0.4063929687413443,
            -0.37276329463949154,
            -0.34042639107244316,
            -0.3093358106224002,
            -0.2794432577552813,
            -0.25069968358290873,
            -0.2230560998625927,
            -0.19646417639451472,
            -0.1708766715939743,
        ];
        let y_coord = vec![
            0.5759630634132183,
            0.5433029251819196,
            0.5132261197435244,
            0.48547985117955084,
            0.4598398263810646,
            0.43610675136729066,
            0.4141032697068614,
            0.39367129154423924,
            0.37466966541529934,
            0.356972149464602,
            0.3404656433099943,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "so";
        let vals = vec![30.0, 1.0, 100.0];
        let units = vec!["um"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.046172180141428695;
        let _end_y_coord = -0.12001976132636427;
        let _real_old = 1.064132904633071;
        let _imag_old = 0.2597289923842747;
        let _start_x_coord = -0.17087667159397438;
        let _start_y_coord = 0.3404656433099943;
        let x1 = 1.064132904633071;
        let y1 = -0.8476016934709594;
        let x2 = 1.064132904633071;
        let y2 = 0.2597289923842747;
        let x_coord = vec![
            -0.17087667159397438,
            -0.14400547887799817,
            -0.11878381964210159,
            -0.09559130425872452,
            -0.07490451659502159,
            -0.05731422834120993,
            -0.04354228071966479,
            -0.034455461497852655,
            -0.031071906698279042,
            -0.03455315454398919,
            -0.046172180141428695,
        ];
        let y_coord = vec![
            0.3404656433099943,
            0.31092125804598975,
            0.2780192638047865,
            0.24156468957874813,
            0.20137274140024192,
            0.15728930602168834,
            0.10922029273243823,
            0.05717233363125816,
            0.0013072271317152353,
            -0.057988468284552945,
            -0.12001976132636427,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "pr";
        let vals = vec![50.0];
        let units = vec!["Ω"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.3519430749726401;
        let _end_y_coord = -0.054932072917106015;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.04617218014142857;
        let _start_y_coord = -0.12001976132636431;
        let x1 = 1.0641329046330708;
        let y1 = 0.2597289923842747;
        let x2 = 2.0641329046330705;
        let y2 = 0.2597289923842747;
        let x_coord = vec![
            -0.04617218014142857,
            -0.08896458913731281,
            -0.12813290172653954,
            -0.16411279430091572,
            -0.19727372630407816,
            -0.22793088544414436,
            -0.2563546756267832,
            -0.28277830809250887,
            -0.3074039174916329,
            -0.3304075225046448,
            -0.3519430749726401,
        ];
        let y_coord = vec![
            -0.12001976132636431,
            -0.10933815976975723,
            -0.10001584379794438,
            -0.09183246054300243,
            -0.08461040629562605,
            -0.07820528055011539,
            -0.0724987294946288,
            -0.06739302116878705,
            -0.06280689082137406,
            -0.05867232849651435,
            -0.054932072917106015,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "ss";
        let vals = vec![88.0, 1.0, 90.0];
        let units = vec!["um"];
        let zin = c64(test.3 .0, test.3 .1);
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.38098629626319114;
        let _end_y_coord = 0.14443147623556063;
        let _real_old = 2.064132904633071;
        let _imag_old = -0.7149393238413289;
        let _start_x_coord = -0.3519430749726401;
        let _start_y_coord = -0.054932072917105994;
        let x1 = 2.064132904633071;
        let y1 = 0.25972899238427477;
        let x2 = 2.064132904633071;
        let y2 = -0.7149393238413289;
        let x_coord = vec![
            -0.3519430749726401,
            -0.3500627176775468,
            -0.34863601136042877,
            -0.3476870897334982,
            -0.34729113435214887,
            -0.34759617902057444,
            -0.3488660154972174,
            -0.3515638686756509,
            -0.3565210759751759,
            -0.3652957486328339,
            -0.38098629626319114,
        ];
        let y_coord = vec![
            -0.05493207291710602,
            -0.04247556697043597,
            -0.029645051770576435,
            -0.016159070650345237,
            -0.0016824013198469435,
            0.014207091842598303,
            0.03206684081462307,
            0.05266314897898353,
            0.07708478029368314,
            0.10691285060883539,
            0.14443147623556063,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );

        let element = "xfmr";
        let vals = vec![25.0, 14.0, 34.0, 0.65];
        let units = vec!["Q", "pH", "pH", "K"];
        let zin = c64(test.3 .0, test.3 .1).inv();
        let rin = zin.re;
        let xin = zin.im;
        let test = calc_smith_arc(
            element, vals, units, rin, xin, z0, 280.0, "ghz", npts, false,
        )
        .unwrap();
        let _end_x_coord = -0.028701359002830346;
        let _end_y_coord = 0.7836278512674113;
        let _real_old = 0.0;
        let _imag_old = 0.0;
        let _start_x_coord = -0.38098629626319114;
        let _start_y_coord = 0.14443147623556063;
        let x1 = 0.43257058304640505;
        let y1 = 0.1498264571349498;
        let x2 = 0.23028393891535823;
        let y2 = 0.9371862407526932;
        let x_coord = vec![
            -0.38098629626319114,
            -0.3799473226333779,
            -0.36991270392532116,
            -0.3508851612135016,
            -0.3232165365047452,
            -0.2875864223346257,
            -0.24495026535542308,
            -0.1964647967226344,
            -0.14340189411894294,
            -0.08706261620951165,
            -0.028701359002830346,
        ];
        let y_coord = vec![
            0.14443147623556063,
            0.22331994594020113,
            0.30239781121591947,
            0.3801253662611437,
            0.4549914134785027,
            0.5256099344377014,
            0.5908035232208507,
            0.6496633617155398,
            0.7015801027812613,
            0.7462453282250527,
            0.7836278512674113,
        ];
        comp_f64(
            &test.2 .0,
            &x1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x1"),
        );
        comp_f64(
            &test.2 .1,
            &y1,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y1"),
        );
        comp_f64(
            &test.3 .0,
            &x2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x2"),
        );
        comp_f64(
            &test.3 .1,
            &y2,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y2"),
        );
        comp_vec_f64(
            test.0,
            x_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_x_coord"),
        );
        comp_vec_f64(
            test.1,
            y_coord,
            margin,
            testname,
            &(element.to_string().as_str().to_owned() + "_y_coord"),
        );
    }
}
