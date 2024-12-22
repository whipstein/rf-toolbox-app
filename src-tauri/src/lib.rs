use crate::conjugate::calc_match;
use crate::copy::{copy_complex, copy_complex_w_unit, copy_complex_ri, copy_rc, copy_scalar, copy_scalar_w_unit, copy_pi_tee, copy_ccll, paste_impedance};
use crate::matching::{
    calc_bp1, calc_bp2, calc_bp3, calc_bp4, calc_hp1, calc_hp2, calc_hp_ell_cl, calc_hp_ell_lc, calc_hp_ell_cl_w_q, calc_hp_ell_lc_w_q, calc_lp_ell_cl_w_q, calc_lp_ell_lc_w_q,
    calc_lp1, calc_lp2, calc_lp_ell_cl, calc_lp_ell_lc, calc_pi, calc_tee, PiTee, CCLL, CL, CLQ
};
use crate::rf_utils::{calc_gamma, calc_gamma_from_rc, calc_rc, calc_z, calc_z_from_rc, unscale, ComplexReturn, Unit};
use std::str::FromStr;
use regex::Regex;
use num_complex::Complex;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::f64::consts::PI;
use std::f64::INFINITY;
use tauri::ipc::Response;
use tauri::{
    utils::{config::WindowEffectsConfig, WindowEffect},
    window::Effect,
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_positioner::{Position, WindowExt};
use std::collections::HashMap;

mod matching;
mod rf_utils;
mod copy;
mod conjugate;

#[derive(Serialize, Default)]
struct ResultsReturn {
    zre: f64,
    zim: f64,
    gre: f64,
    gim: f64,
    gmag: f64,
    gang: f64,
    r: f64,
    c: f64,
}

#[derive(Default)]
struct ResponseReturn {
    z: Complex<f64>,
    g: Complex<f64>,
    g_mag: f64,
    g_ang: f64,
    r: f64,
    c: f64,
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

#[tauri::command(rename_all = "snake_case")]
fn calc_vals(
    re: f64,
    im: f64,
    imp: &str,
    z0: f64,
    freq: f64,
    f_scale: &str,
    r_scale: &str,
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

#[tauri::command(rename_all = "snake_case")]
fn copy_point(app: AppHandle, x: f64, y: f64, unit: &str) {
    let val = format!("{}, {}{}", x, y, unit.to_string());
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn copy_val(app: AppHandle, x: f64, unit: &str) {
    let val = format!("{}{}", x, unit.to_string());
    app.clipboard().write_text(val.to_string()).unwrap();
}

#[tauri::command]
async fn start_impedance_calculator(app: AppHandle) -> tauri::Result<()> {
    let re = Regex::new(r"Impedance-Calculator-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key){
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            },
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Impedance-Calculator-{}", i),
        WebviewUrl::App("impCalc.html".into()),
    ).inner_size(500.0, 600.0).build()?;

    Ok(())
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
struct MatchingReturn {
    zs: ComplexReturn,
    zl: ComplexReturn,
    hp1: CCLL,
    hp2: CCLL,
    lp1: CCLL,
    lp2: CCLL,
    bp1: CCLL,
    bp2: CCLL,
    bp3: CCLL,
    bp4: CCLL,
    pi: PiTee,
    tee: PiTee,
    hp_ell_cl: CL,
    hp_ell_cl_w_q: CLQ,
    hp_ell_lc: CL,
    hp_ell_lc_w_q: CLQ,
    lp_ell_cl: CL,
    lp_ell_cl_w_q: CLQ,
    lp_ell_lc: CL,
    lp_ell_lc_w_q: CLQ,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
struct Complex2Return {
    src: ComplexReturn,
    load: ComplexReturn,
}

#[tauri::command(rename_all = "snake_case")]
fn calc_networks(
    rs: f64,
    xs: f64,
    rl: f64,
    xl: f64,
    imp: &str,
    q_net: f64,
    q: f64,
    z0: f64,
    freq: f64,
    f_scale: &str,
    c_scale: &str,
    l_scale: &str,
    z_scale: &str,
) -> Result<MatchingReturn, String> {
    let mut out = MatchingReturn::default();

    let freq_unit = Unit::from_str(f_scale).unwrap();
    let cap_unit = Unit::from_str(c_scale).unwrap();
    let ind_unit = Unit::from_str(l_scale).unwrap();
    let w = 2.0 * PI * unscale(freq, &freq_unit);

    println!("rs = {:}, xs = {:}, rl = {:}, xl = {:}", rs, xs, rl, xl);

    let (zs_init, zl_init) = match imp {
        "zri" => (Complex::new(rs, xs), Complex::new(rl, xl)),
        "yri" => (1.0 / Complex::new(rs, xs), 1.0 / Complex::new(rl, xl)),
        "gma" => (
            calc_z(Complex::from_polar(rs, xs * PI / 180.0), z0),
            calc_z(Complex::from_polar(rl, xl * PI / 180.0), z0),
        ),
        "gri" => (
            calc_z(Complex::new(rs, xs), z0),
            calc_z(Complex::new(rl, xl), z0),
        ),
        "rc" => (
            1.0 / Complex::new(1.0 / rs, unscale(xs, &cap_unit) * w),
            1.0 / Complex::new(1.0 / rl, unscale(xl, &cap_unit) * w),
        ),
        _ => (
            Complex::new(INFINITY, INFINITY),
            Complex::new(INFINITY, INFINITY),
        ),
    };

    println!("zs_init = {:}, zl_init = {:}", zs_init, zl_init);

    let (zs, zl) = match z_scale {
        "diff" => (zs_init / 2.0, zl_init / 2.0),
        "se" => (zs_init, zl_init),
        _ => (
            Complex::new(INFINITY, INFINITY),
            Complex::new(INFINITY, INFINITY),
        ),
    };

    println!("zs = {:}, zl = {:}", zs, zl);

    if zs == Complex::new(INFINITY, INFINITY) || zl == Complex::new(INFINITY, INFINITY) {
        return Err("Impedance type not recognized".to_string());
    }

    out.hp_ell_cl = calc_hp_ell_cl(zs, zl, w, &cap_unit, &ind_unit)?;
    out.hp_ell_cl_w_q = calc_hp_ell_cl_w_q(zs, zl, q, w, &cap_unit, &ind_unit)?;
    out.hp_ell_lc = calc_hp_ell_lc(zs, zl, w, &cap_unit, &ind_unit)?;
    out.hp_ell_lc_w_q = calc_hp_ell_lc_w_q(zs, zl, q, w, &cap_unit, &ind_unit)?;
    out.lp_ell_cl = calc_lp_ell_cl(zs, zl, w, &cap_unit, &ind_unit)?;
    out.lp_ell_cl_w_q = calc_lp_ell_cl_w_q(zs, zl, q, w, &cap_unit, &ind_unit)?;
    out.lp_ell_lc = calc_lp_ell_lc(zs, zl, w, &cap_unit, &ind_unit)?;
    out.lp_ell_lc_w_q = calc_lp_ell_lc_w_q(zs, zl, q, w, &cap_unit, &ind_unit)?;
    out.tee = calc_tee(zs, zl, w, q_net, &cap_unit, &ind_unit)?;
    out.pi = calc_pi(zs, zl, w, q_net, &cap_unit, &ind_unit)?;
    out.lp1 = calc_lp1(zs, zl, w, &cap_unit, &ind_unit)?;
    out.lp2 = calc_lp2(zs, zl, w, &cap_unit, &ind_unit)?;
    out.hp1 = calc_hp1(zs, zl, w, &cap_unit, &ind_unit)?;
    out.hp2 = calc_hp2(zs, zl, w, &cap_unit, &ind_unit)?;
    out.bp1 = calc_bp1(zs, zl, w, &cap_unit, &ind_unit)?;
    out.bp2 = calc_bp2(zs, zl, w, &cap_unit, &ind_unit)?;
    out.bp3 = calc_bp3(zs, zl, w, &cap_unit, &ind_unit)?;
    out.bp4 = calc_bp4(zs, zl, w, &cap_unit, &ind_unit)?;
    out.zs = ComplexReturn {re: zs.re, im: zs.im};
    out.zl = ComplexReturn {re: zl.re, im: zl.im};
    
    Ok(out)
}

#[tauri::command(rename_all = "snake_case")]
fn change_impedance(
    rs: f64,
    xs: f64,
    rl: f64,
    xl: f64,
    imp_in: &str,
    imp_out: &str,
    z0: f64,
    freq: f64,
    f_scale: &str,
    c_scale: &str,
) -> Result<Complex2Return, String> {
    if imp_in == imp_out {
        return Ok(Complex2Return {
            src: ComplexReturn { re: rs, im: xs },
            load: ComplexReturn { re: rl, im: xl },
        });
    }
    
    let freq_unit = Unit::from_str(f_scale).unwrap();
    let cap_unit = Unit::from_str(c_scale).unwrap();

    match imp_in {
        "zri" => match imp_out {
            "yri" => {
                let ys = Complex::new(rs, xs).inv();
                let yl = Complex::new(rl, xl).inv();
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: ys.re,
                        im: ys.im,
                    },
                    load: ComplexReturn {
                        re: yl.re,
                        im: yl.im,
                    },
                })
            }
            "gma" => {
                let gs = calc_gamma(Complex::new(rs, xs), z0);
                let gl = calc_gamma(Complex::new(rl, xl), z0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.norm(),
                        im: gs.arg() * 180.0 / PI,
                    },
                    load: ComplexReturn {
                        re: gl.norm(),
                        im: gl.arg() * 180.0 / PI,
                    },
                })
            }
            "gri" => {
                let gs = calc_gamma(Complex::new(rs, xs), z0);
                let gl = calc_gamma(Complex::new(rl, xl), z0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.re,
                        im: gs.im,
                    },
                    load: ComplexReturn {
                        re: gl.re,
                        im: gl.im,
                    },
                })
            }
            "rc" => {
                let (src_r, src_c) = calc_rc(Complex::new(rs, xs), freq, &freq_unit, &Unit::Base, &cap_unit);
                let (load_r, load_c) = calc_rc(Complex::new(rl, xl), freq, &freq_unit, &Unit::Base, &cap_unit);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: src_r,
                        im: src_c,
                    },
                    load: ComplexReturn {
                        re: load_r,
                        im: load_c,
                    },
                })
            }
            _ => Err("impedance unit(s) not recognized".to_string()),
        }
        "yri" => match imp_out {
            "zri" => {
                let zs = Complex::new(rs, xs).inv();
                let zl = Complex::new(rl, xl).inv();
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: zs.re,
                        im: zs.im,
                    },
                    load: ComplexReturn {
                        re: zl.re,
                        im: zl.im,
                    },
                })
            }
            "gma" => {
                let gs = calc_gamma(Complex::new(rs, xs).inv(), z0);
                let gl = calc_gamma(Complex::new(rl, xl).inv(), z0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.norm(),
                        im: gs.arg() * 180.0 / PI,
                    },
                    load: ComplexReturn {
                        re: gl.norm(),
                        im: gl.arg() * 180.0 / PI,
                    },
                })
            }
            "gri" => {
                let gs = calc_gamma(Complex::new(rs, xs).inv(), z0);
                let gl = calc_gamma(Complex::new(rl, xl).inv(), z0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.re,
                        im: gs.im,
                    },
                    load: ComplexReturn {
                        re: gl.re,
                        im: gl.im,
                    },
                })
            }
            "rc" => {
                let (src_r, src_c) =
                    calc_rc(Complex::new(rs, xs).inv(), freq, &freq_unit, &Unit::Base, &cap_unit);
                let (load_r, load_c) =
                    calc_rc(Complex::new(rl, xl).inv(), freq, &freq_unit, &Unit::Base, &cap_unit);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: src_r,
                        im: src_c,
                    },
                    load: ComplexReturn {
                        re: load_r,
                        im: load_c,
                    },
                })
            }
            _ => Err("impedance unit(s) not recognized".to_string()),
        }
        "gma" => match imp_out {
            "zri" => {
                let zs = calc_z(Complex::new(rs, xs), z0);
                let zl = calc_z(Complex::new(rl, xl), z0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: zs.re,
                        im: zs.im,
                    },
                    load: ComplexReturn {
                        re: zl.re,
                        im: zl.im,
                    },
                })
            }
            "yri" => {
                let ys = calc_z(Complex::from_polar(rs, xs * PI / 180.0), z0).inv();
                let yl = calc_z(Complex::from_polar(rl, xl * PI / 180.0), z0).inv();
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: ys.re,
                        im: ys.im,
                    },
                    load: ComplexReturn {
                        re: yl.re,
                        im: yl.im,
                    },
                })
            }
            "gri" => {
                let gs = Complex::from_polar(rs, xs * PI / 180.0);
                let gl = Complex::from_polar(rl, xl * PI / 180.0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.re,
                        im: gs.im,
                    },
                    load: ComplexReturn {
                        re: gl.re,
                        im: gl.im,
                    },
                })
            }
            "rc" => {
                let (src_r, src_c) =
                    calc_rc(calc_z(Complex::new(rs, xs), z0), freq, &freq_unit, &Unit::Base, &cap_unit);
                let (load_r, load_c) =
                    calc_rc(calc_z(Complex::new(rl, xl), z0), freq, &freq_unit, &Unit::Base, &cap_unit);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: src_r,
                        im: src_c,
                    },
                    load: ComplexReturn {
                        re: load_r,
                        im: load_c,
                    },
                })
            }
            _ => Err("impedance unit(s) not recognized".to_string()),
        }
        "gri" => match imp_out {
            "zri" => {
                let zs = calc_z(Complex::new(rs, xs), z0);
                let zl = calc_z(Complex::new(rl, xl), z0);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: zs.re,
                        im: zs.im,
                    },
                    load: ComplexReturn {
                        re: zl.re,
                        im: zl.im,
                    },
                })
            }
            "yri" => {
                let ys = calc_z(Complex::new(rs, xs), z0).inv();
                let yl = calc_z(Complex::new(rl, xl), z0).inv();
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: ys.re,
                        im: ys.im,
                    },
                    load: ComplexReturn {
                        re: yl.re,
                        im: yl.im,
                    },
                })
            }
            "gma" => {
                let gs = Complex::new(rs, xs);
                let gl = Complex::new(rl, xl);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.norm(),
                        im: gs.arg() * 180.0 / PI,
                    },
                    load: ComplexReturn {
                        re: gl.norm(),
                        im: gl.arg() * 180.0 / PI,
                    },
                })
            }
            "rc" => {
                let (src_r, src_c) =
                    calc_rc(calc_z(Complex::new(rs, xs), z0), freq, &freq_unit, &Unit::Base, &cap_unit);
                let (load_r, load_c) =
                    calc_rc(calc_z(Complex::new(rl, xl), z0), freq, &freq_unit, &Unit::Base, &cap_unit);
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: src_r,
                        im: src_c,
                    },
                    load: ComplexReturn {
                        re: load_r,
                        im: load_c,
                    },
                })
            }
            _ => Err("impedance unit(s) not recognized".to_string()),
        }
        "rc" => match imp_out {
            "zri" => {
                let zs = Complex::new(
                    1.0 / rs,
                    unscale(xs, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                )
                .inv();
                let zl = Complex::new(
                    1.0 / rl,
                    unscale(xl, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                )
                .inv();
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: zs.re,
                        im: zs.im,
                    },
                    load: ComplexReturn {
                        re: zl.re,
                        im: zl.im,
                    },
                })
            }
            "yri" => {
                let ys = Complex::new(
                    1.0 / rs,
                    unscale(xs, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                );
                let yl = Complex::new(
                    1.0 / rl,
                    unscale(xl, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                );
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: ys.re,
                        im: ys.im,
                    },
                    load: ComplexReturn {
                        re: yl.re,
                        im: yl.im,
                    },
                })
            }
            "gma" => {
                let gs = calc_gamma(
                    Complex::new(
                        1.0 / rs,
                        unscale(xs, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                    )
                    .inv(),
                    z0,
                );
                let gl = calc_gamma(
                    Complex::new(
                        1.0 / rl,
                        unscale(xl, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                    )
                    .inv(),
                    z0,
                );
                Ok(Complex2Return {
                    src: ComplexReturn {
                        re: gs.norm(),
                        im: gs.arg() * 180.0 / PI,
                    },
                    load: ComplexReturn {
                        re: gl.norm(),
                        im: gl.arg() * 180.0 / PI,
                    },
                })
            }
            "gri" => {
                let gs = calc_gamma(
                    Complex::new(
                        1.0 / rs,
                        unscale(xs, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit),
                    )
                    .inv(),
                    z0,
                );
                let gl = calc_gamma(Complex::new(1.0 / rl, unscale(xl, &cap_unit) * 2.0 * PI * unscale(freq, &freq_unit)).inv(), z0);
                Ok(Complex2Return {src: ComplexReturn {re: gs.re, im: gs.im}, load: ComplexReturn {re: gl.re, im: gl.im}})
            }
            _ => Err("impedance unit(s) not recognized".to_string()),
        }
        _ => Err("impedance unit(s) not recognized".to_string()),
    }
}

#[tauri::command]
async fn start_matching_calculator(app: AppHandle) -> tauri::Result<()> {
    let re = Regex::new(r"Matching-Network-Calculator-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key){
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            },
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Matching-Network-Calculator-{}", i),
        WebviewUrl::App("matchCalc.html".into()),
    ).inner_size(1200.0, 1150.0).build()?;

    Ok(())
}


#[tauri::command]
async fn start_conjugate_match_calculator(app: AppHandle) -> tauri::Result<()> {    
    let re = Regex::new(r"Conjugate-Match-Calculator-(?<i>\d+)").unwrap();
    let mut i: usize = 0;

    for key in app.webview_windows().keys() {
        match re.captures(key){
            Some(val) => {
                if val["i"].parse::<usize>().unwrap() > i {
                    i = val["i"].parse::<usize>().unwrap();
                }
            },
            None => (),
        };
    }

    i += 1;

    WebviewWindowBuilder::new(
        &app,
        format!("Conjugate-Match-Calculator-{}", i),
        WebviewUrl::App("conjCalc.html".into()),
    ).inner_size(750.0, 750.0).build()?;

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
            calc_vals,
            copy_point,
            start_matching_calculator,
            calc_networks,
            change_impedance,
            copy_val,
            copy_complex, copy_complex_w_unit, copy_complex_ri, copy_rc, copy_scalar, copy_scalar_w_unit, calc_match, copy_pi_tee, copy_ccll,
            start_conjugate_match_calculator,
            paste_impedance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
