#![allow(dead_code, unused_variables, unused_imports)]
use crate::rf_utils::{ComplexReturn, Unit};
use num_complex::Complex;
use std::str::FromStr;
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command(rename_all = "snake_case")]
pub fn copy_rc(app: AppHandle, r: &str, c: &str, unit: &str) {
    let cunit = Unit::from_str(unit).unwrap();
    let val = format!("{} {}{}", r, c, cunit.to_string());
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_scalar(app: AppHandle, x: &str) {
    let val = format!("{}", x);
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_scalar_w_unit(app: AppHandle, x: &str, unit: &str) {
    let val = format!("{}{}", x, Unit::from_str(unit).unwrap().to_string());
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_complex(app: AppHandle, re: &str, im: &str) {
    let val = format!("{} {}", re, im);
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_complex_w_unit(app: AppHandle, re: &str, unit_re: &str, im: &str, unit_im: &str) {
    let val = format!(
        "{}{} {}{}",
        re,
        Unit::from_str(unit_re).unwrap().to_string(),
        im,
        Unit::from_str(unit_im).unwrap().to_string()
    );
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_complex_ri(app: AppHandle, re: &str, im: &str) {
    let mut val = "".to_string();
    let im_val: String = im.to_string();

    if &im[0..0] == "-" {
        val = format!("{} - {}", re, &im[1..]);
    } else {
        val = format!("{} + {}", re, &im);
    }
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_pi_tee(
    app: AppHandle,
    val1: &str,
    unit1: &str,
    val2: &str,
    unit2: &str,
    val3: &str,
    unit3: &str,
) {
    let val = format!(
        "{}{} {}{} {}{}",
        val1,
        Unit::from_str(unit1).unwrap().to_string(),
        val2,
        Unit::from_str(unit2).unwrap().to_string(),
        val3,
        Unit::from_str(unit3).unwrap().to_string()
    );
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn copy_ccll(
    app: AppHandle,
    val1: &str,
    unit1: &str,
    val2: &str,
    unit2: &str,
    val3: &str,
    unit3: &str,
    val4: &str,
    unit4: &str,
) {
    let val = format!(
        "{}{} {}{} {}{} {}{}",
        val1,
        Unit::from_str(unit1).unwrap().to_string(),
        val2,
        Unit::from_str(unit2).unwrap().to_string(),
        val3,
        Unit::from_str(unit3).unwrap().to_string(),
        val4,
        Unit::from_str(unit4).unwrap().to_string()
    );
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub fn paste_impedance(app: AppHandle) -> Result<String, String> {
    let val = app.clipboard().read_text().unwrap();

    Ok(val)
}
