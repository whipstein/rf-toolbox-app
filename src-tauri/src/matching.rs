#![allow(unused)]
use crate::matching::bp::{calc_bp1, calc_bp2, calc_bp3, calc_bp4};
use crate::matching::ell::{calc_hp_ell_cl, calc_hp_ell_lc, calc_lp_ell_cl, calc_lp_ell_lc};
use crate::matching::ell_w_q::{
    calc_hp_ell_cl_w_q, calc_hp_ell_lc_w_q, calc_lp_ell_cl_w_q, calc_lp_ell_lc_w_q,
};
use crate::matching::hp::{calc_hp1, calc_hp2};
use crate::matching::lp::{calc_lp1, calc_lp2};
use crate::matching::pi::calc_pi;
use crate::matching::tee::calc_tee;
use crate::rf_utils::{calc_gamma, calc_rc, calc_z, scale, unscale, Complex2Return, ComplexReturn};
use crate::unit::{get_unit, Unit, UnitType};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

pub mod bp;
pub mod ell;
pub mod ell_w_q;
pub mod hp;
pub mod lp;
pub mod pi;
pub mod tee;

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct MatchingReturn {
    pub zs: ComplexReturn,
    pub zl: ComplexReturn,
    pub hp1: CCLL,
    pub hp2: CCLL,
    pub lp1: CCLL,
    pub lp2: CCLL,
    pub bp1: CCLL,
    pub bp2: CCLL,
    pub bp3: CCLL,
    pub bp4: CCLL,
    pub pi: PiTee,
    pub tee: PiTee,
    pub hp_ell_cl: CL,
    pub hp_ell_cl_w_q: CLQ,
    pub hp_ell_lc: CL,
    pub hp_ell_lc_w_q: CLQ,
    pub lp_ell_cl: CL,
    pub lp_ell_cl_w_q: CLQ,
    pub lp_ell_lc: CL,
    pub lp_ell_lc_w_q: CLQ,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct CCLL {
    cs: f64,
    cl: f64,
    ls: f64,
    ll: f64,
    c_scale: String,
    l_scale: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct PiTee {
    c: f64,
    cs: f64,
    cl: f64,
    l: f64,
    ls: f64,
    ll: f64,
    q: f64,
    c_scale: String,
    l_scale: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct CL {
    c: f64,
    l: f64,
    q: f64,
    c_scale: String,
    l_scale: String,
}

#[derive(serde::Serialize, Default, Debug, PartialEq)]
pub struct CLQ {
    c: f64,
    l: f64,
    q: f64,
    q_net: f64,
    sol: usize,
    c_scale: String,
    l_scale: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn calc_networks(
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

    let (zs, zl) = match z_scale {
        "diff" => (zs_init / 2.0, zl_init / 2.0),
        "se" => (zs_init, zl_init),
        _ => (
            Complex::new(INFINITY, INFINITY),
            Complex::new(INFINITY, INFINITY),
        ),
    };

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
    out.zs = ComplexReturn {
        re: zs.re,
        im: zs.im,
    };
    out.zl = ComplexReturn {
        re: zl.re,
        im: zl.im,
    };

    Ok(out)
}

#[tauri::command(rename_all = "snake_case")]
pub fn change_impedance(
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
                let (src_r, src_c) = calc_rc(
                    Complex::new(rs, xs),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
                let (load_r, load_c) = calc_rc(
                    Complex::new(rl, xl),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
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
        },
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
                let (src_r, src_c) = calc_rc(
                    Complex::new(rs, xs).inv(),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
                let (load_r, load_c) = calc_rc(
                    Complex::new(rl, xl).inv(),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
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
        },
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
                let (src_r, src_c) = calc_rc(
                    calc_z(Complex::new(rs, xs), z0),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
                let (load_r, load_c) = calc_rc(
                    calc_z(Complex::new(rl, xl), z0),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
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
        },
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
                let (src_r, src_c) = calc_rc(
                    calc_z(Complex::new(rs, xs), z0),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
                let (load_r, load_c) = calc_rc(
                    calc_z(Complex::new(rl, xl), z0),
                    freq,
                    &freq_unit,
                    &Unit::Base,
                    &cap_unit,
                );
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
        },
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
                        re: gs.re,
                        im: gs.im,
                    },
                    load: ComplexReturn {
                        re: gl.re,
                        im: gl.im,
                    },
                })
            }
            _ => Err("impedance unit(s) not recognized".to_string()),
        },
        _ => Err("impedance unit(s) not recognized".to_string()),
    }
}
