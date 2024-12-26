use crate::rf_utils::{
    calc_gamma, calc_rc, calc_z, get_unit, scale, unscale, Complex2Return, ComplexReturn, Element,
    Unit,
};
use float_cmp::F64Margin;
use num_complex::Complex;
use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};
use std::str::FromStr;

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

// ---CAP---------
//          |
//         IND
//          |
//         GND
pub fn calc_hp_ell_cl(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = zl.im / zl.re;
        let c1 = -1.0 / (w * zs.im);
        let l1 = (1.0 + qs.powi(2)) * zl.im / (w * qs.powi(2));
        let rp = (1.0 + qs.powi(2)) * zl.re;

        if zs.re > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / zs.re - 1.0).sqrt();
            l = rp / (w * q);
            c = 1.0 / (q * w * zs.re);

            if zs.im != 0.0 {
                if c1 == c {
                    c = INFINITY;
                } else {
                    c *= c1 / (c1 - c);
                }
            }

            if zl.im != 0.0 {
                if l1 == l {
                    l = INFINITY;
                } else {
                    l *= l1 / (l1 - l);
                }
            }

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---CAP---------
//          |
//         RES
//          |
//         IND
//          |
//         GND
pub fn calc_hp_ell_cl_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = zs.im / zs.re;
        let rp = (1.0 + qs.powi(2)) * zl.re;
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / rs - 1.0).sqrt();
        let xp1 = (q
            * (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            + q * xl.powi(2)
            - 2.0 * q.powi(2) * rs * xl
            - 2.0 * q * rl * rs
            + q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl);
        let xc1 = -(((2.0 * xl - 2.0 * q * rl) * xs
            + (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            + xl.powi(2)
            + rl.powi(2))
            / (2.0 * xl - 2.0 * q * rl));
        let xp2 = -((q
            * (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            - q * xl.powi(2)
            + 2.0 * q.powi(2) * rs * xl
            + 2.0 * q * rl * rs
            - q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl));
        let xc2 = ((2.0 * q * rl - 2.0 * xl) * xs
            + (xl.powi(4) - 4.0 * q * rs * xl.powi(3)
                + (-(4.0 * rs.powi(2)) + 4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2))
                    * xl.powi(2)
                + (8.0 * q * rl * rs.powi(2) - 4.0 * q * rl.powi(2) * rs) * xl
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            - xl.powi(2)
            - rl.powi(2))
            / (2.0 * xl - 2.0 * q * rl);

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------CAP----
//     |
//    IND
//     |
//    GND
pub fn calc_hp_ell_lc(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = zs.im / zs.re;
        let c1 = -1.0 / (w * zl.im);
        let l1 = (1.0 + qs.powi(2)) * zs.im / (w * qs.powi(2));
        let rp = (1.0 + qs.powi(2)) * zs.re;
        let rs = zl.re + 0.0;

        if rs > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / rs - 1.0).sqrt();
            l = rp / (w * q);
            c = 1.0 / (q * w * rs);

            if zl.im != 0.0 {
                if c1 == c {
                    c = INFINITY;
                } else {
                    c *= c1 / (c1 - c);
                }
            }

            if zs.im != 0.0 {
                if l1 == l {
                    l = INFINITY;
                } else {
                    l *= l1 / (l1 - l);
                }
            }

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------CAP----
//     |
//    RES
//     |
//    IND
//     |
//    GND
pub fn calc_hp_ell_lc_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = zs.im / zs.re;
        let rp = (1.0 + qs.powi(2)) * zs.re;
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / rs - 1.0).sqrt();
        let xp1 = -((q
            * (xs.powi(4) - 4.0 * q * rl * xs.powi(3)
                + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
                + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            + q * xs.powi(2)
            - 2.0 * q.powi(2) * rl * xs
            + q * rs.powi(2)
            - 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl));
        let xc1 = -(((xs.powi(4) - 4.0 * q * rl * xs.powi(3)
            + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
            + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            + xs.powi(2)
            + 2.0 * xl * xs
            - 2.0 * q * rs * xl
            + rs.powi(2))
            / (2.0 * xs - 2.0 * q * rs));
        let xp2 = (q
            * (xs.powi(4) - 4.0 * q * rl * xs.powi(3)
                + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
                + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            - q * xs.powi(2)
            + 2.0 * q.powi(2) * rl * xs
            - q * rs.powi(2)
            + 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs + (-(2.0 * q.powi(2)) - 2.0) * rl);
        let xc2 = ((xs.powi(4) - 4.0 * q * rl * xs.powi(3)
            + (2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs - 4.0 * rl.powi(2)) * xs.powi(2)
            + (8.0 * q * rl.powi(2) * rs - 4.0 * q * rl * rs.powi(2)) * xs
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            - xs.powi(2)
            - 2.0 * xl * xs
            + 2.0 * q * rs * xl
            - rs.powi(2))
            / (2.0 * xs - 2.0 * q * rs);

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------IND----
//     |
//    CAP
//     |
//    GND
pub fn calc_lp_ell_cl(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = -zs.im / zs.re;
        let rp = zs.re * (1.0 + qs.powi(2));

        if zl.re > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / zl.re - 1.0).sqrt();
            let cp = q / (rp * w);
            let c1 = qs / (rp * w);
            c = cp - c1;
            let ls = q * zl.re / w;
            let l1 = zl.im / w;
            l = ls - l1;

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------RES--IND----
//     |
//    CAP
//     |
//    GND
pub fn calc_lp_ell_cl_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = -zs.im / zs.re;
        let rp = zs.re * (1.0 + qs.powi(2));
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / zl.re - 1.0).sqrt();
        let xp1 = -((q
            * (xs.powi(4)
                + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs)
                    * xs.powi(2)
                - 4.0 * rs.powi(2) * xl.powi(2)
                + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            + q * xs.powi(2)
            + 2.0 * q.powi(2) * rs * xl
            + q * rs.powi(2)
            - 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs));
        let xc1 = -(((xs.powi(4)
            + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs) * xs.powi(2)
            - 4.0 * rs.powi(2) * xl.powi(2)
            + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            + xs.powi(2)
            + (2.0 * xl + 2.0 * q * rl) * xs
            + rs.powi(2))
            / (2.0 * xs + 2.0 * xl - 2.0 * q * rs + 2.0 * q * rl));
        let xp2 = (q
            * (xs.powi(4)
                + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs) * xs.powi(2)
                - 4.0 * rs.powi(2) * xl.powi(2)
                + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
                + rs.powi(4)
                + 4.0 * q.powi(2) * rl * rs.powi(3)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
            .sqrt()
            - q * xs.powi(2)
            - 2.0 * q.powi(2) * rs * xl
            - q * rs.powi(2)
            + 2.0 * q * rl * rs)
            / ((2.0 * q.powi(2) + 2.0) * rs);
        let xc2 = ((xs.powi(4)
            + (4.0 * q * rs * xl + 2.0 * rs.powi(2) + 4.0 * q.powi(2) * rl * rs) * xs.powi(2)
            - 4.0 * rs.powi(2) * xl.powi(2)
            + (4.0 * q * rs.powi(3) - 8.0 * q * rl * rs.powi(2)) * xl
            + rs.powi(4)
            + 4.0 * q.powi(2) * rl * rs.powi(3)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2))
        .sqrt()
            - xs.powi(2)
            + (-(2.0 * xl) - 2.0 * q * rl) * xs
            - rs.powi(2))
            / (2.0 * xs + 2.0 * xl - 2.0 * q * rs + 2.0 * q * rl);

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---IND---------
//          |
//         CAP
//          |
//         GND
pub fn calc_lp_ell_lc(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CL, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q: f64;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q = zs.im / zs.re;
    } else {
        let qs = -zl.im / zl.re;
        let rp = zl.re * (1.0 + qs.powi(2));

        if zs.re > rp {
            c = NAN;
            l = NAN;
            q = NAN;
        } else {
            q = (rp / zs.re - 1.0).sqrt();
            let cp = q / (rp * w);
            let c1 = qs / (rp * w);
            c = cp - c1;
            let ls = q * zs.re / w;
            let l1 = zs.im / w;
            l = ls - l1;

            c = scale(c, c_scale);
            l = scale(l, l_scale);

            if (c < 0.0) || (l < 0.0) {
                c = NAN;
                l = NAN;
                q = NAN;
            }
        }
    }

    Ok(CL {
        c: c,
        l: l,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---RES--IND---------
//          |
//         CAP
//          |
//         GND
pub fn calc_lp_ell_lc_w_q(
    zs: Complex<f64>,
    zl: Complex<f64>,
    q: f64,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CLQ, String> {
    let mut c: f64;
    let mut l: f64;
    let mut q_net: f64;
    let mut sol: usize = 0;

    if (zs.re == zl.re) && (zs.im == (-zl.im)) {
        c = 0.0;
        l = 0.0;
        q_net = zs.im / zs.re;
    } else {
        let qs = -zl.im / zl.re;
        let rp = zl.re * (1.0 + qs.powi(2));
        let rs = zs.re;
        let xs = zs.im;
        let rl = zl.re;
        let xl = zl.im;
        q_net = (rp / zs.re - 1.0).sqrt();

        let xp1 = -((q
            * (-(4.0 * rl.powi(2) * xs.powi(2))
                + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3))
                    * xs
                + xl.powi(4)
                + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            + 2.0 * q.powi(2) * rl * xs
            + q * xl.powi(2)
            - 2.0 * q * rl * rs
            + q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rl));
        let xc1 = -(((-(4.0 * rl.powi(2) * xs.powi(2))
            + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3))
                * xs
            + xl.powi(4)
            + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
            + 4.0 * q.powi(2) * rl.powi(3) * rs
            + rl.powi(4))
        .sqrt()
            + 2.0 * xl * xs
            + xl.powi(2)
            + 2.0 * q * rs * xl
            + rl.powi(2))
            / (2.0 * xs + 2.0 * xl + 2.0 * q * rs - 2.0 * q * rl));
        let xp2 = (q
            * (-(4.0 * rl.powi(2) * xs.powi(2))
                + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3))
                    * xs
                + xl.powi(4)
                + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
                - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
                + 4.0 * q.powi(2) * rl.powi(3) * rs
                + rl.powi(4))
            .sqrt()
            - 2.0 * q.powi(2) * rl * xs
            - q * xl.powi(2)
            + 2.0 * q * rl * rs
            - q * rl.powi(2))
            / ((2.0 * q.powi(2) + 2.0) * rl);
        let xc2 = ((-(4.0 * rl.powi(2) * xs.powi(2))
            + (4.0 * q * rl * xl.powi(2) - 8.0 * q * rl.powi(2) * rs + 4.0 * q * rl.powi(3)) * xs
            + xl.powi(4)
            + (4.0 * q.powi(2) * rl * rs + 2.0 * rl.powi(2)) * xl.powi(2)
            - 4.0 * q.powi(2) * rl.powi(2) * rs.powi(2)
            + 4.0 * q.powi(2) * rl.powi(3) * rs
            + rl.powi(4))
        .sqrt()
            - 2.0 * xl * xs
            - xl.powi(2)
            - 2.0 * q * rs * xl
            - rl.powi(2))
            / (2.0 * xs + 2.0 * xl + 2.0 * q * rs - 2.0 * q * rl);

        if xc1 >= 0.0 || xp1 < 0.0 {
            l = xp2 / w;
            c = -1.0 / (w * xc2);
            sol = 2;
        } else {
            l = xp1 / w;
            c = -1.0 / (w * xc1);
            sol = 1;
        }

        c = scale(c, c_scale);
        l = scale(l, l_scale);

        if l < 0.0 || c < 0.0 {
            l = NAN;
            c = NAN;
            q_net = NAN;
        }
    }

    Ok(CLQ {
        c: c,
        l: l,
        q: q,
        q_net: q_net,
        sol: sol,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_tee(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    q_tgt: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<PiTee, String> {
    let mut c: f64;
    let mut cs: f64;
    let mut cl: f64;
    let mut l: f64;
    let mut ls: f64;
    let mut ll: f64;
    let mut q = q_tgt;

    if q_tgt < 0.0 {
        c = NAN;
        cs = NAN;
        cl = NAN;
        l = NAN;
        ls = NAN;
        ll = NAN;
        q = NAN;
    } else {
        if (q_tgt == 0.0) && (zs.re == zl.re) {
            cs = 0.0;
            cl = 0.0;
            l = 0.0;
            ls = 0.0;
            ll = 0.0;
            c = 0.0;
            q = 0.0;
        } else {
            if q_tgt < (zs.re.max(zl.re) / zs.re.min(zl.re) - 1.0).sqrt() {
                c = NAN;
                cs = NAN;
                cl = NAN;
                l = NAN;
                ls = NAN;
                ll = NAN;
                q = NAN;
            } else {
                let rv = zs.re.min(zl.re) * (q_tgt.powi(2) + 1.0);

                let mut qx = (rv / zs.re - 1.0).sqrt();
                cs = 1.0 / (w * zs.re * qx);
                if zs.im != 0.0 {
                    if cs == -1.0 / (w * zs.im) {
                        cs = INFINITY;
                    } else {
                        cs *= -1.0 / (w * zs.im) / (cs + 1.0 / (w * zs.im));
                    }
                }

                let l5 = rv / (w * qx);
                qx = (rv / zl.re - 1.0).sqrt();
                cl = 1.0 / (w * zl.re * qx);
                if zl.im != 0.0 {
                    if cl == -1.0 / (w * zs.im) {
                        cl = INFINITY;
                    } else {
                        cl *= -1.0 / (w * zs.im) / (cl + 1.0 / (w * zs.im));
                    }
                }

                let l1 = rv / (w * qx);
                l = l1 * l5 / (l1 + l5);

                qx = (rv / zs.re - 1.0).sqrt();
                ls = qx * zs.re / w - zs.im / w;
                let c5 = qx / (w * rv);
                qx = (rv / zl.re - 1.0).sqrt();
                ll = qx * zl.re / w - zl.im / w;
                c = c5 + (qx / (w * rv));

                c = scale(c, c_scale);
                cs = scale(cs, c_scale);
                cl = scale(cl, c_scale);
                l = scale(l, l_scale);
                ls = scale(ls, l_scale);
                ll = scale(ll, l_scale);

                if (c < 0.0) || (ls < 0.0) || (ll < 0.0) {
                    c = NAN;
                    ls = NAN;
                    ll = NAN;
                }
                if (l < 0.0) || (cs < 0.0) || (cl < 0.0) || (cs == INFINITY) || (cl == INFINITY) {
                    l = NAN;
                    cs = NAN;
                    cl = NAN;
                }
            }
        }
    }

    Ok(PiTee {
        c: c,
        cs: cs,
        cl: cl,
        l: l,
        ls: ls,
        ll: ll,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_pi(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    q_tgt: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<PiTee, String> {
    let mut c: f64;
    let mut cs: f64;
    let mut cl: f64;
    let mut l: f64;
    let mut ls: f64;
    let mut ll: f64;
    let mut q = q_tgt;

    if q_tgt < 0.0 {
        c = NAN;
        cs = NAN;
        cl = NAN;
        l = NAN;
        ls = NAN;
        ll = NAN;
        q = NAN;
    } else {
        if (q_tgt == 0.0) && (zs.re == zl.re) {
            cs = 0.0;
            cl = 0.0;
            l = 0.0;
            ls = 0.0;
            ll = 0.0;
            c = 0.0;
            q = 0.0;
        } else {
            if q_tgt < (zs.re.max(zl.re) / zs.re.min(zl.re) - 1.0).sqrt() {
                c = NAN;
                cs = NAN;
                cl = NAN;
                l = NAN;
                ls = NAN;
                ll = NAN;
                q = NAN;
            } else {
                let rv = zs.re.max(zl.re) / (q_tgt.powi(2) + 1.0);

                let qs = -zs.im / zs.re;
                let ql = -zl.im / zl.re;
                let rps = zs.re * (1.0 + qs.powi(2));
                let rpl = zl.re * (1.0 + ql.powi(2));

                //cs-l-cl pi network matching
                let cps = qs / (rps * w);
                let cpl = ql / (rpl * w);
                let mut qx = (rps / rv - 1.0).sqrt();
                cs = qx / (w * rps) - cps;
                let l5 = qx * rv / w;
                qx = (rpl / rv - 1.0).sqrt();
                cl = qx / (w * rpl) - cpl;
                l = l5 + (qx * rv / w);

                //ls-c-ll pi network matching
                qx = (rps / rv - 1.0).sqrt();
                ls = rps / (w * qx);
                if qs != 0.0 {
                    let lps = rps / (qs * w);
                    ls *= lps / (ls - lps);
                }
                let c5 = 1.0 / (w * qx * rv);
                qx = (rpl / rv - 1.0).sqrt();
                ll = rpl / (w * qx);
                if ql != 0.0 {
                    let lpl = rpl / (ql * w);
                    ll *= lpl / (ll - lpl);
                }
                let c1 = 1.0 / (w * qx * rv);
                c = c1 * c5 / (c1 + c5);

                c = scale(c, c_scale);
                cs = scale(cs, c_scale);
                cl = scale(cl, c_scale);
                l = scale(l, l_scale);
                ls = scale(ls, l_scale);
                ll = scale(ll, l_scale);

                if (c < 0.0) || (ls < 0.0) || (ll < 0.0) {
                    c = NAN;
                    ls = NAN;
                    ll = NAN;
                }
                if (l < 0.0) || (cs < 0.0) || (cl < 0.0) {
                    l = NAN;
                    cs = NAN;
                    cl = NAN;
                }
            }
        }
    }

    Ok(PiTee {
        c: c,
        cs: cs,
        cl: cl,
        l: l,
        ls: ls,
        ll: ll,
        q: q,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_lp1(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();

    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = zl.re * ql / w - zl.im / w;
        cl = ql / (w * rv);
        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_lp2(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = zs.re * ql / w - zs.im / w;
        cl = ql / (w * rv);

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_hp1(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();
    if rp <= rv {
        ls = NAN;
        ll = NAN;
        cs = NAN;
        cl = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zs.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        let c5 = -1.0 / (w * zl.im);
        ll = rv / (w * ql);
        cl = 1.0 / (w * zl.re * ql);
        if zl.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

pub fn calc_hp2(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zl.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        let c5 = -1.0 / (w * zs.im);
        ll = rv / (w * ql);
        cl = 1.0 / (w * zs.re * ql);
        if zs.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------CAP-------IND--
//     |         |
//    IND       CAP
//     |         |
//    GND       GND
pub fn calc_bp1(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zs.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        ll = ql * zl.re / w - zl.im / w;
        cl = ql / (w * rv);

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---IND-------CAP-------
//          |         |
//         CAP       IND
//          |         |
//         GND       GND
pub fn calc_bp2(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let lp = rp / (w * q);
        cs = 1.0 / (w * rv * qs);
        ls = rp / (w * qs);
        if zl.im != 0.0 {
            if lp == ls {
                ls = INFINITY;
            } else {
                ls *= lp / (lp - ls);
            }
        }

        ll = ql * zs.re / w - zs.im / w;
        cl = ql / (w * rv);

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// --------IND-------CAP--
//     |         |
//    CAP       IND
//     |         |
//    GND       GND
pub fn calc_bp3(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zs.im / zs.re;
    let rp = (1.0 + q.powi(2)) * zs.re;
    let rv = (rp * zl.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zl.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = rv / (w * ql);
        let c5 = -1.0 / (w * zl.im);
        cl = 1.0 / (w * zl.re * ql);
        if zl.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

// ---CAP-------IND-------
//          |         |
//         IND       CAP
//          |         |
//         GND       GND
pub fn calc_bp4(
    zs: Complex<f64>,
    zl: Complex<f64>,
    w: f64,
    c_scale: &Unit,
    l_scale: &Unit,
) -> Result<CCLL, String> {
    let mut cs: f64;
    let mut cl: f64;
    let mut ls: f64;
    let mut ll: f64;

    let q = -zl.im / zl.re;
    let rp = (1.0 + q.powi(2)) * zl.re;
    let rv = (rp * zs.re).sqrt();
    if rp <= rv {
        cs = NAN;
        cl = NAN;
        ls = NAN;
        ll = NAN;
    } else {
        let qs = (rp / rv - 1.0).sqrt();
        let ql = (rv / zs.re - 1.0).sqrt();
        let cp = q / (w * rp);
        cs = qs / (w * rp) - cp;
        ls = qs * rv / w;
        ll = rv / (w * ql);
        let c5 = -1.0 / (w * zs.im);
        cl = 1.0 / (w * zs.re * ql);
        if zs.im != 0.0 {
            if c5 == cl {
                cl = INFINITY;
            } else {
                cl *= c5 / (c5 - cl);
            }
        }

        cs = scale(cs, c_scale);
        cl = scale(cl, c_scale);
        ls = scale(ls, l_scale);
        ll = scale(ll, l_scale);

        if (cs < 0.0) || (cl < 0.0) || (ls < 0.0) || (ll < 0.0) {
            cs = NAN;
            cl = NAN;
            ls = NAN;
            ll = NAN;
        }
    }

    Ok(CCLL {
        cs: cs,
        cl: cl,
        ls: ls,
        ll: ll,
        c_scale: get_unit(c_scale, &Element::Capacitor),
        l_scale: get_unit(l_scale, &Element::Inductor),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rf_utils::comp_f64;
    use std::f64::consts::PI;

    #[test]
    fn test_calc_hp_ell_cl() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 8.58125245724517,
            l: 69.18681390709257,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(62.4, -14.6);
        let zl = Complex::new(202.3, 23.2);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 11.408503434826747,
            l: 133.4483264614267,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 175.0e9;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_hp_ell_lc() {
        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 8.58125245724517,
            l: 69.18681390709257,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(202.3, 23.2);
        let zl = Complex::new(62.4, -14.6);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 11.408503434826747,
            l: 133.4483264614267,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_hp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_lp_ell_cl() {
        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 5.906505625073422,
            l: 61.719118523742445,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(202.3, 23.2);
        let zl = Complex::new(62.4, -14.6);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 7.2157251698188345,
            l: 99.0557187033109,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_cl()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_cl(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_lp_ell_lc() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: 5.906505625073422,
            l: 61.719118523742445,
            q: 2.0529004985170953,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(62.4, -14.6);
        let zl = Complex::new(202.3, 23.2);
        let w = 2.0 * PI * 175.0e6;
        let c_scale = Unit::Pico;
        let l_scale = Unit::Nano;
        let exemplar = CL {
            c: 7.2157251698188345,
            l: 99.0557187033109,
            q: 1.5114976179652644,
            c_scale: "pF".to_string(),
            l_scale: "nH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.c,
            exemplar.c,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "c",
        );
        comp_f64(
            test.l,
            exemplar.l,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "l",
        );
        comp_f64(
            test.q,
            exemplar.q,
            F64Margin::default(),
            "calc_lp_ell_lc()",
            "q",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CL {
            c: NAN,
            l: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp_ell_lc(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.l.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_tee() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: 4.186603177852454,
            cs: -3.5382547173462546,
            cl: -1.6843173326757916,
            l: 80.00425342422247,
            ls: 117.35101636675431,
            ll: 185.20322518485523,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(test.c, exemplar.c, F64Margin::default(), "calc_tee()", "c");
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_tee()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_tee()",
            "ll",
        );
        comp_f64(test.q, exemplar.q, F64Margin::default(), "calc_tee()", "q");
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 1.99;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: NAN,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: NAN,
            ll: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, -363.20820041403255);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: 4.186603177852454,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: 420.4100397629459,
            ll: 117.35101636675431,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(test.c, exemplar.c, F64Margin::default(), "calc_tee()", "c");
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_tee()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_tee()",
            "ll",
        );
        comp_f64(test.q, exemplar.q, F64Margin::default(), "calc_tee()", "q");
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, -183.168);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: 4.186603177852454,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: 316.2126293951322,
            ll: 117.35101636675431,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_tee(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(test.c, exemplar.c, F64Margin::default(), "calc_tee()", "c");
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_tee()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_tee()",
            "ll",
        );
        comp_f64(test.q, exemplar.q, F64Margin::default(), "calc_tee()", "q");
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_pi() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let q = 4.32;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: 8.435997609374349,
            cs: 16.62637373190316,
            cl: 12.08508737222243,
            l: 39.704380813877926,
            ls: -20.145466896660622,
            ll: -27.71565081088584,
            q: 4.32,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_pi(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_pi()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_pi()",
            "cl",
        );
        comp_f64(test.l, exemplar.l, F64Margin::default(), "calc_pi()", "l");
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        comp_f64(test.q, exemplar.q, F64Margin::default(), "calc_pi()", "q");
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let q = 1.99;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: NAN,
            cs: NAN,
            cl: NAN,
            l: NAN,
            ls: NAN,
            ll: NAN,
            q: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_pi(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        assert!(test.c.is_nan());
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.l.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert!(test.q.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(42.4, 0.0);
        let zl = Complex::new(212.3, 0.0);
        let w = 2.0 * PI * 275.0e9;
        let q = 3.88;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = PiTee {
            c: 8.157016433395613,
            cs: 20.27486954435912,
            cl: 10.57716232084193,
            l: 41.06232522178837,
            ls: 16.520257301519933,
            ll: 31.666911357459586,
            q: 3.88,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_pi(zs, zl, w, q, &c_scale, &l_scale).unwrap();
        comp_f64(test.c, exemplar.c, F64Margin::default(), "calc_pi()", "c");
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_pi()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_pi()",
            "cl",
        );
        comp_f64(test.l, exemplar.l, F64Margin::default(), "calc_pi()", "l");
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_pi()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_pi()",
            "ll",
        );
        comp_f64(test.q, exemplar.q, F64Margin::default(), "calc_pi()", "q");
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_lp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 6.772022183008002,
            ls: 63.48256505664435,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_lp1()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_lp1()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_lp1()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_lp1()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_lp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 6.772022183008002,
            ls: 63.48256505664435,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_lp2()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_lp2()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_lp2()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_lp2()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_lp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_hp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 20.352712959723295,
            ls: 137.66998607438342,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_hp1()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_hp1()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_hp1()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_hp1()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_hp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 20.352712959723295,
            ls: 137.66998607438342,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_hp2()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_hp2()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_hp2()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_hp2()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_hp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp1() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 6.772022183008002,
            ls: 137.66998607438342,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp1(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_bp1()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_bp1()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_bp1()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_bp1()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp2() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 5.276189790514869,
            cl: 6.772022183008002,
            ls: 137.66998607438342,
            ll: 39.14388565971301,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_bp2()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_bp2()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_bp2()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_bp2()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp2(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp3() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp3(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 20.352712959723295,
            ls: 63.48256505664435,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp3(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_bp3()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_bp3()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_bp3()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_bp3()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }

    #[test]
    fn test_calc_bp4() {
        let zs = Complex::new(42.4, -19.6);
        let zl = Complex::new(212.3, 43.2);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: 3.498285705078592,
            cl: 20.352712959723295,
            ls: 63.48256505664435,
            ll: 49.4602723641384,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp4(zs, zl, w, &c_scale, &l_scale).unwrap();
        comp_f64(
            test.cs,
            exemplar.cs,
            F64Margin::default(),
            "calc_bp4()",
            "cs",
        );
        comp_f64(
            test.cl,
            exemplar.cl,
            F64Margin::default(),
            "calc_bp4()",
            "cl",
        );
        comp_f64(
            test.ls,
            exemplar.ls,
            F64Margin::default(),
            "calc_bp4()",
            "ls",
        );
        comp_f64(
            test.ll,
            exemplar.ll,
            F64Margin::default(),
            "calc_bp4()",
            "ll",
        );
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);

        let zs = Complex::new(212.3, 43.2);
        let zl = Complex::new(42.4, -19.6);
        let w = 2.0 * PI * 275.0e9;
        let c_scale = Unit::Femto;
        let l_scale = Unit::Pico;
        let exemplar = CCLL {
            cs: NAN,
            cl: NAN,
            ls: NAN,
            ll: NAN,
            c_scale: "fF".to_string(),
            l_scale: "pH".to_string(),
        };
        let test = calc_bp4(zs, zl, w, &c_scale, &l_scale).unwrap();
        assert!(test.cs.is_nan());
        assert!(test.cl.is_nan());
        assert!(test.ls.is_nan());
        assert!(test.ll.is_nan());
        assert_eq!(test.c_scale, exemplar.c_scale);
        assert_eq!(test.l_scale, exemplar.l_scale);
    }
}
