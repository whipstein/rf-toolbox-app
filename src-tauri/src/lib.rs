use crate::conjugate::calc_match;
use crate::copy::{
    copy_ccll, copy_complex, copy_complex_ri, copy_complex_w_unit, copy_pi_tee, copy_rc,
    copy_scalar, copy_scalar_w_unit, paste_impedance,
};
use crate::matching::{calc_networks, change_impedance};
use crate::rf_utils::{calc_impedance, get_c64_inv};
use crate::smith::{arc_smith_points, calc_ri, calc_smith_arc, find_smith_coord_js};
use crate::unit::get_unit_scale;
use regex::Regex;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

mod conjugate;
mod copy;
mod element;
mod frequency;
mod matching;
mod rf_utils;
mod smith;
mod unit;

#[tauri::command]
async fn start_impedance_calculator(app: AppHandle) -> tauri::Result<()> {
    let re = Regex::new(r"Impedance-Calculator-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key) {
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            }
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Impedance-Calculator-{}", i),
        WebviewUrl::App("impCalc.html".into()),
    )
    .inner_size(500.0, 600.0)
    .build()?;

    Ok(())
}

#[tauri::command]
async fn start_matching_calculator(app: AppHandle) -> tauri::Result<()> {
    let re = Regex::new(r"Matching-Network-Calculator-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key) {
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            }
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Matching-Network-Calculator-{}", i),
        WebviewUrl::App("matchCalc.html".into()),
    )
    .inner_size(1200.0, 1150.0)
    .build()?;

    Ok(())
}

#[tauri::command]
async fn start_conjugate_match_calculator(app: AppHandle) -> tauri::Result<()> {
    let re = Regex::new(r"Conjugate-Match-Calculator-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key) {
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            }
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Conjugate-Match-Calculator-{}", i),
        WebviewUrl::App("conjCalc.html".into()),
    )
    .inner_size(750.0, 750.0)
    .build()?;

    Ok(())
}

#[tauri::command]
async fn start_smith_chart_tool(app: AppHandle) -> tauri::Result<()> {
    let re = Regex::new(r"Smith-Chart-Tool-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key) {
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            }
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Smith-Chart-Tool-{}", i),
        WebviewUrl::App("smithChart.html".into()),
    )
    // .setup(|app| {
    //     let handle = app.handle();
    //     handle.manage(Mutex::new(SmithState::default()));
    //     Ok(())
    // })
    .inner_size(1800.0, 1600.0)
    .build()?;
    // .open_devtools();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_impedance_calculator,
            calc_impedance,
            start_matching_calculator,
            calc_networks,
            change_impedance,
            copy_complex,
            copy_complex_w_unit,
            copy_complex_ri,
            copy_rc,
            copy_scalar,
            copy_scalar_w_unit,
            calc_match,
            copy_pi_tee,
            copy_ccll,
            start_conjugate_match_calculator,
            paste_impedance,
            start_smith_chart_tool,
            get_unit_scale,
            get_c64_inv,
            arc_smith_points,
            calc_ri,
            calc_smith_arc,
            find_smith_coord_js
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
