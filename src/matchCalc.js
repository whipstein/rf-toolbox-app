const { invoke } = window.__TAURI__.core;
import {scalarCopyWUnit, print_cval, complexCopyWUnit, rcCopy, copyPiTee, copyCCLL, pasteImpedance} from "/util.js";

function digits(val, sd) {
    return val.toFixed(sd);
}

function print_unit(unit) {
    switch (unit) {
        case "milli":
            return "m";
        case "micro":
            return "μ";
        case "nano":
            return "n";
        case "pico":
            return "p";
        case "femto":
            return "f";
        default:
            return "";
    }
}
  
function print_val(val, unit, suffix, sd) {
    if (Number.isFinite(val)) {
        return "" + digits(val, sd) + " " + print_unit(unit) + suffix;
    }
    return "" + Number.NaN;
}

function calc_nets() {
    invoke("calc_networks", { rs: rs, xs: xs, rl: rl, xl: xl, imp: imp_unit, q_net: q_net, q: q, z0: z0, freq: freq, f_scale: freq_unit, c_scale: cap_unit, l_scale: ind_unit, z_scale: mode_unit })
    .then((result) => {
        current = result;

        zsEl.innerHTML = "Z<sub>S</sub> = " + print_cval(result.zs, "", " Ω", sd);
        zlEl.innerHTML = "Z<sub>L</sub> = " + print_cval(result.zl, "", " Ω", sd);

        document.getElementById("hp1_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp1.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("hp1_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp1.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("hp1_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp1.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("hp1_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp1.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("hp2_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp2.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("hp2_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp2.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("hp2_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp2.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("hp2_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp2.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("lp1_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp1.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("lp1_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp1.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("lp1_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp1.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("lp1_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp1.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("lp2_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp2.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("lp2_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp2.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("lp2_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp2.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("lp2_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp2.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("bp1_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp1.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp1_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp1.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp1_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp1.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("bp1_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp1.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("bp2_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp2.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp2_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp2.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp2_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp2.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("bp2_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp2.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("bp3_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp3.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp3_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp3.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp3_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp3.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("bp3_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp3.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("bp4_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp4.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp4_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp4.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("bp4_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp4.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("bp4_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.bp4.ll, ind_unit, "H", sd) + "</div>";

        document.getElementById("tee_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("tee_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("tee_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.l, ind_unit, "H", sd) + "</div>";
        if (Number.isFinite(result.tee.cs) || Number.isFinite(result.tee.cl) || Number.isFinite(result.tee.l)) {
            document.getElementById("teehp_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.q, "", "", sd) + "</div>";
        } else {
            document.getElementById("teehp_q_val").innerHTML = "<div class=\"text_box\">" + Number.NaN + "</div>";
        }

        document.getElementById("tee_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("tee_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.ll, ind_unit, "H", sd) + "</div>";
        document.getElementById("tee_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.c, cap_unit, "F", sd) + "</div>";
        if (Number.isFinite(result.tee.ls) || Number.isFinite(result.tee.ll) || Number.isFinite(result.tee.c)) {
            document.getElementById("teelp_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.tee.q, "", "", sd) + "</div>";
        } else {
            document.getElementById("teelp_q_val").innerHTML = "<div class=\"text_box\">" + Number.NaN + "</div>";
        }

        document.getElementById("pi_cs_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.cs, cap_unit, "F", sd) + "</div>";
        document.getElementById("pi_cl_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.cl, cap_unit, "F", sd) + "</div>";
        document.getElementById("pi_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.l, ind_unit, "H", sd) + "</div>";
        if (Number.isFinite(result.pi.cs) || Number.isFinite(result.pi.cl) || Number.isFinite(result.pi.l)) {
            document.getElementById("pilp_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.q, "", "", sd) + "</div>";
        } else {
            document.getElementById("pilp_q_val").innerHTML = "<div class=\"text_box\">" + Number.NaN + "</div>";
        }

        document.getElementById("pi_ls_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.ls, ind_unit, "H", sd) + "</div>";
        document.getElementById("pi_ll_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.ll, ind_unit, "H", sd) + "</div>";
        document.getElementById("pi_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.c, cap_unit, "F", sd) + "</div>";
        if (Number.isFinite(result.pi.ls) || Number.isFinite(result.pi.ll) || Number.isFinite(result.pi.c)) {
            document.getElementById("pihp_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.pi.q, "", "", sd) + "</div>";
        } else {
            document.getElementById("pihp_q_val").innerHTML = "<div class=\"text_box\">" + Number.NaN + "</div>";
        }

        document.getElementById("hplc_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_lc.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("hplc_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_lc.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("hplc_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_lc.q, "", "", sd) + "</div>";

        document.getElementById("hplcq_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_lc_w_q.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("hplcq_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_lc_w_q.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("hplcq_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_lc_w_q.q_net, "", "", sd) + "</div>";

        document.getElementById("hpcl_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_cl.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("hpcl_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_cl.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("hpcl_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_cl.q, "", "", sd) + "</div>";

        document.getElementById("hpclq_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_cl_w_q.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("hpclq_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_cl_w_q.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("hpclq_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.hp_ell_cl_w_q.q_net, "", "", sd) + "</div>";

        document.getElementById("lplc_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_lc.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("lplc_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_lc.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("lplc_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_lc.q, "", "", sd) + "</div>";

        document.getElementById("lplcq_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_lc_w_q.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("lplcq_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_lc_w_q.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("lplcq_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_lc_w_q.q_net, "", "", sd) + "</div>";

        document.getElementById("lpcl_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_cl.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("lpcl_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_cl.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("lpcl_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_cl.q, "", "", sd) + "</div>";

        document.getElementById("lpclq_c_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_cl_w_q.c, cap_unit, "F", sd) + "</div>";
        document.getElementById("lpclq_l_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_cl_w_q.l, ind_unit, "H", sd) + "</div>";
        document.getElementById("lpclq_q_val").innerHTML = "<div class=\"text_box\">" + print_val(result.lp_ell_cl_w_q.q_net, "", "", sd) + "</div>";
    })
    .catch((err) => {
        console.log("ERROR: " + err);
        var txt = "<div class=\"text_box\">ERROR";
        document.getElementById("hp1_cs_val").innerHTML = txt;
        document.getElementById("hp1_cl_val").innerHTML = txt;
        document.getElementById("hp1_ls_val").innerHTML = txt;
        document.getElementById("hp1_ll_val").innerHTML = txt;

        document.getElementById("hp2_cs_val").innerHTML = txt;
        document.getElementById("hp2_cl_val").innerHTML = txt;
        document.getElementById("hp2_ls_val").innerHTML = txt;
        document.getElementById("hp2_ll_val").innerHTML = txt;

        document.getElementById("lp1_cs_val").innerHTML = txt;
        document.getElementById("lp1_cl_val").innerHTML = txt;
        document.getElementById("lp1_ls_val").innerHTML = txt;
        document.getElementById("lp1_ll_val").innerHTML = txt;

        document.getElementById("lp2_cs_val").innerHTML = txt;
        document.getElementById("lp2_cl_val").innerHTML = txt;
        document.getElementById("lp2_ls_val").innerHTML = txt;
        document.getElementById("lp2_ll_val").innerHTML = txt;

        document.getElementById("bp1_cs_val").innerHTML = txt;
        document.getElementById("bp1_cl_val").innerHTML = txt;
        document.getElementById("bp1_ls_val").innerHTML = txt;
        document.getElementById("bp1_ll_val").innerHTML = txt;

        document.getElementById("bp2_cs_val").innerHTML = txt;
        document.getElementById("bp2_cl_val").innerHTML = txt;
        document.getElementById("bp2_ls_val").innerHTML = txt;
        document.getElementById("bp2_ll_val").innerHTML = txt;

        document.getElementById("bp3_cs_val").innerHTML = txt;
        document.getElementById("bp3_cl_val").innerHTML = txt;
        document.getElementById("bp3_ls_val").innerHTML = txt;
        document.getElementById("bp3_ll_val").innerHTML = txt;

        document.getElementById("bp4_cs_val").innerHTML = txt;
        document.getElementById("bp4_cl_val").innerHTML = txt;
        document.getElementById("bp4_ls_val").innerHTML = txt;
        document.getElementById("bp4_ll_val").innerHTML = txt;

        document.getElementById("tee_cs_val").innerHTML = txt;
        document.getElementById("tee_cl_val").innerHTML = txt;
        document.getElementById("tee_l_val").innerHTML = txt;
        document.getElementById("teehp_q_val").innerHTML = txt;

        document.getElementById("tee_ls_val").innerHTML = txt;
        document.getElementById("tee_ll_val").innerHTML = txt;
        document.getElementById("tee_c_val").innerHTML = txt;
        document.getElementById("teelp_q_val").innerHTML = txt;

        document.getElementById("pi_cs_val").innerHTML = txt;
        document.getElementById("pi_cl_val").innerHTML = txt;
        document.getElementById("pi_l_val").innerHTML = txt;
        document.getElementById("pilp_q_val").innerHTML = txt;

        document.getElementById("pi_ls_val").innerHTML = txt;
        document.getElementById("pi_ll_val").innerHTML = txt;
        document.getElementById("pi_c_val").innerHTML = txt;
        document.getElementById("pihp_q_val").innerHTML = txt;

        document.getElementById("hplc_c_val").innerHTML = txt;
        document.getElementById("hplc_l_val").innerHTML = txt;
        document.getElementById("hplc_q_val").innerHTML = txt;

        document.getElementById("hplcq_c_val").innerHTML = txt;
        document.getElementById("hplcq_l_val").innerHTML = txt;
        document.getElementById("hplcq_q_val").innerHTML = txt;

        document.getElementById("hpcl_c_val").innerHTML = txt;
        document.getElementById("hpcl_l_val").innerHTML = txt;
        document.getElementById("hpcl_q_val").innerHTML = txt;

        document.getElementById("hpclq_c_val").innerHTML = txt;
        document.getElementById("hpclq_l_val").innerHTML = txt;
        document.getElementById("hpclq_q_val").innerHTML = txt;

        document.getElementById("lplc_c_val").innerHTML = txt;
        document.getElementById("lplc_l_val").innerHTML = txt;
        document.getElementById("lplc_q_val").innerHTML = txt;

        document.getElementById("lplcq_c_val").innerHTML = txt;
        document.getElementById("lplcq_l_val").innerHTML = txt;
        document.getElementById("lplcq_q_val").innerHTML = txt;

        document.getElementById("lpcl_c_val").innerHTML = txt;
        document.getElementById("lpcl_l_val").innerHTML = txt;
        document.getElementById("lpcl_q_val").innerHTML = txt;

        document.getElementById("lpclq_c_val").innerHTML = txt;
        document.getElementById("lpclq_l_val").innerHTML = txt;
        document.getElementById("lpclq_q_val").innerHTML = txt;
    });
}

function change_imp() {
    invoke("change_impedance", { rs: rs, xs: xs, rl: rl, xl: xl, imp_in: imp_unit, imp_out: impUnitEl.value, z0: z0, freq: freq, f_scale: freq_unit, c_scale: cap_unit })
    .then((result) => {
        switch (imp_unit) {
            case "zri":
                z_label = "Z";
                r_label = "+";
                x_label = "jΩ";
                break;
            case "yri":
                z_label = "Y";
                r_label = "+";
                x_label = "jS";
                break;
            case "gma":
                z_label = "Γ";
                r_label = "&ang;";
                x_label = "&deg;";
                break;
            case "gri":
                z_label = "Γ";
                r_label = "+";
                x_label = "j";
                break;
            case "rc":
                var unit
                switch (cap_unit) {
                    case "milli":
                        unit = "mF"
                        break;
                    case "micro":
                        unit = "uF"
                        break;
                    case "nano":
                        unit = "nF"
                        break;
                    case "pico":
                        unit = "pF"
                        break;
                    case "femto":
                        unit = "fF"
                        break;
                }
                z_label = "RC";
                r_label = "Ω";
                x_label = unit;
                break;
        }

        zsLabelEl.innerHTML = z_label;
        zlLabelEl.innerHTML = z_label;
        rsLabelEl.innerHTML = r_label;
        xsLabelEl.innerHTML = x_label;
        rlLabelEl.innerHTML = r_label;
        xlLabelEl.innerHTML = x_label;

        rsEl.innerText = print_val(result.src.re, "", "", result.sd);
        xsEl.innerText = print_val(result.src.im, "", "", result.sd);
        rlEl.innerText = print_val(result.load.re, "", "", result.sd);
        xlEl.innerText = print_val(result.load.im, "", "", result.sd);

        update_imp()
    })
    .catch((err) => {
        console.log("ERROR: " + err);
        var txt = "<div class=\"text_box\">ERROR";
        rsEl.innerText = txt;
        xsEl.innerText = txt;
        rlEl.innerText = txt;
        xlEl.innerText = txt;
    });

    imp_unit = impUnitEl.value;
}

function change_unit() {
    freq_unit = freqUnitEl.value;
    cap_unit = capUnitEl.value;
    ind_unit = indUnitEl.value;
    mode_unit = modeUnitEl.value;

    update_imp();
}

function update_imp() {
    sd = parseInt(sigDigitsEl.value);
    freq = parseFloat(freqEl.value);
    q_net = parseFloat(qNetEl.value);
    q = parseFloat(qEl.value);
    z0 = parseFloat(z0El.value);
    rs = parseFloat(rsEl.value);
    xs = parseFloat(xsEl.value);
    rl = parseFloat(rlEl.value);
    xl = parseFloat(xlEl.value);

    calc_nets();
}

let sigDigitsEl, modeUnitEl, capUnitEl, indUnitEl, freqUnitEl, impUnitEl, z0El, qNetEl, qEl, freqEl, rsLabelEl, rsEl, xsLabelEl, xsEl, rlLabelEl, rlEl, xlLabelEl, xlEl, calcEl, zsLabelEl, zsEl, zsPasteEl, zlLabelEl, zlEl, zlPasteEl;
let mode_unit, cap_unit, ind_unit, freq_unit, imp_unit, z0, q_net, q, freq, rs, xs, rl, xl;
let z_label, r_label, x_label;
let sd = 2;
let hpclCCopyEl, hpclLCopyEl, hpclAllCopyEl, hplcCCopyEl, hplcLCopyEl, hplcAllCopyEl, lpclCCopyEl, lpclLCopyEl, lpclAllCopyEl, lplcCCopyEl, lplcLCopyEl, lplcAllCopyEl;
let hpclqCCopyEl, hpclqLCopyEl, hpclqAllCopyEl, hplcqCCopyEl, hplcqLCopyEl, hplcqAllCopyEl, lpclqCCopyEl, lpclqLCopyEl, lpclqAllCopyEl, lplcqCCopyEl, lplcqLCopyEl, lplcqAllCopyEl;
let piCCopyEl, piCsCopyEl, piClCopyEl, piLCopyEl, piLsCopyEl, piLlCopyEl, pihpAllCopyEl, pilpAllCopyEl;
let teeCCopyEl, teeCsCopyEl, teeClCopyEl, teeLCopyEl, teeLsCopyEl, teeLlCopyEl, teehpAllCopyEl, teelpAllCopyEl;
let hp1CsCopyEl, hp1ClCopyEl, hp1LsCopyEl, hp1LlCopyEl, hp1AllCopyEl, hp2CsCopyEl, hp2ClCopyEl, hp2LsCopyEl, hp2LlCopyEl, hp2AllCopyEl;
let lp1CsCopyEl, lp1ClCopyEl, lp1LsCopyEl, lp1LlCopyEl, lp1AllCopyEl, lp2CsCopyEl, lp2ClCopyEl, lp2LsCopyEl, lp2LlCopyEl, lp2AllCopyEl;
let bp1CsCopyEl, bp1ClCopyEl, bp1LsCopyEl, bp1LlCopyEl, bp1AllCopyEl, bp2CsCopyEl, bp2ClCopyEl, bp2LsCopyEl, bp2LlCopyEl, bp2AllCopyEl;
let bp3CsCopyEl, bp3ClCopyEl, bp3LsCopyEl, bp3LlCopyEl, bp3AllCopyEl, bp4CsCopyEl, bp4ClCopyEl, bp4LsCopyEl, bp4LlCopyEl, bp4AllCopyEl;

let current = {};

window.addEventListener("DOMContentLoaded", () => {
    z_label = "Z";
    r_label = "+";
    x_label = "jΩ";

    sigDigitsEl = document.getElementById("sig_digits");
    sd = parseInt(sigDigitsEl.value);
    modeUnitEl = document.getElementById("mode_unit");
    mode_unit = modeUnitEl.value;
    capUnitEl = document.getElementById("cap_unit");
    cap_unit = capUnitEl.value;
    indUnitEl = document.getElementById("ind_unit");
    ind_unit = indUnitEl.value;
    freqUnitEl = document.getElementById("freq_unit");
    freq_unit = freqUnitEl.value;
    impUnitEl = document.getElementById("imp_unit");
    imp_unit = impUnitEl.value;
    z0El = document.getElementById("z0");
    z0 = parseFloat(z0El.value);
    qNetEl = document.getElementById("q_net");
    q_net = parseFloat(qNetEl.value);
    qEl = document.getElementById("q");
    q = parseFloat(qEl.value);
    freqEl = document.getElementById("freq");
    freq = parseFloat(freqEl.value);
    rsLabelEl = document.getElementById("rs_label");
    rsEl = document.getElementById("rs");
    rs = parseFloat(rsEl.value);
    xsLabelEl = document.getElementById("xs_label");
    xsEl = document.getElementById("xs");
    xs = parseFloat(xsEl.value);
    rlLabelEl = document.getElementById("rl_label");
    rlEl = document.getElementById("rl");
    rl = parseFloat(rlEl.value);
    xlLabelEl = document.getElementById("xl_label");
    xlEl = document.getElementById("xl");
    xl = parseFloat(xlEl.value);
    calcEl = document.getElementById("calc");
    zsLabelEl = document.getElementById("zs_label");
    zsEl = document.getElementById("zs");
    zsPasteEl = document.getElementById("zs_paste");
    zlLabelEl = document.getElementById("zl_label");
    zlEl = document.getElementById("zl");
    zlPasteEl = document.getElementById("zl_paste");

    hpclCCopyEl = document.getElementById("hpcl_c_copy");
    hpclLCopyEl = document.getElementById("hpcl_l_copy");
    hpclAllCopyEl = document.getElementById("hpcl_all_copy");

    hplcCCopyEl = document.getElementById("hplc_c_copy");
    hplcLCopyEl = document.getElementById("hplc_l_copy");
    hplcAllCopyEl = document.getElementById("hplc_all_copy");

    lpclCCopyEl = document.getElementById("lpcl_c_copy");
    lpclLCopyEl = document.getElementById("lpcl_l_copy");
    lpclAllCopyEl = document.getElementById("lpcl_all_copy");

    lplcCCopyEl = document.getElementById("lplc_c_copy");
    lplcLCopyEl = document.getElementById("lplc_l_copy");
    lplcAllCopyEl = document.getElementById("lplc_all_copy");

    hpclqCCopyEl = document.getElementById("hpclq_c_copy");
    hpclqLCopyEl = document.getElementById("hpclq_l_copy");
    hpclqAllCopyEl = document.getElementById("hpclq_all_copy");

    hplcqCCopyEl = document.getElementById("hplcq_c_copy");
    hplcqLCopyEl = document.getElementById("hplcq_l_copy");
    hplcqAllCopyEl = document.getElementById("hplcq_all_copy");

    lpclqCCopyEl = document.getElementById("lpclq_c_copy");
    lpclqLCopyEl = document.getElementById("lpclq_l_copy");
    lpclqAllCopyEl = document.getElementById("lpclq_all_copy");

    lplcqCCopyEl = document.getElementById("lplcq_c_copy");
    lplcqLCopyEl = document.getElementById("lplcq_l_copy");
    lplcqAllCopyEl = document.getElementById("lplcq_all_copy");

    piCCopyEl = document.getElementById("pi_c_copy");
    piCsCopyEl = document.getElementById("pi_cs_copy");
    piClCopyEl = document.getElementById("pi_cl_copy");
    piLCopyEl = document.getElementById("pi_l_copy");
    piLsCopyEl = document.getElementById("pi_ls_copy");
    piLlCopyEl = document.getElementById("pi_ll_copy");
    pihpAllCopyEl = document.getElementById("pihp_all_copy");
    pilpAllCopyEl = document.getElementById("pilp_all_copy");

    teeCCopyEl = document.getElementById("tee_c_copy");
    teeCsCopyEl = document.getElementById("tee_cs_copy");
    teeClCopyEl = document.getElementById("tee_cl_copy");
    teeLCopyEl = document.getElementById("tee_l_copy");
    teeLsCopyEl = document.getElementById("tee_ls_copy");
    teeLlCopyEl = document.getElementById("tee_ll_copy");
    teehpAllCopyEl = document.getElementById("teehp_all_copy");
    teelpAllCopyEl = document.getElementById("teelp_all_copy");

    hp1CsCopyEl = document.getElementById("hp1_cs_copy");
    hp1ClCopyEl = document.getElementById("hp1_cl_copy");
    hp1LsCopyEl = document.getElementById("hp1_ls_copy");
    hp1LlCopyEl = document.getElementById("hp1_ll_copy");
    hp1AllCopyEl = document.getElementById("hp1_all_copy");

    hp2CsCopyEl = document.getElementById("hp2_cs_copy");
    hp2ClCopyEl = document.getElementById("hp2_cl_copy");
    hp2LsCopyEl = document.getElementById("hp2_ls_copy");
    hp2LlCopyEl = document.getElementById("hp2_ll_copy");
    hp2AllCopyEl = document.getElementById("hp2_all_copy");

    lp1CsCopyEl = document.getElementById("lp1_cs_copy");
    lp1ClCopyEl = document.getElementById("lp1_cl_copy");
    lp1LsCopyEl = document.getElementById("lp1_ls_copy");
    lp1LlCopyEl = document.getElementById("lp1_ll_copy");

    lp1AllCopyEl = document.getElementById("lp1_all_copy");
    lp2CsCopyEl = document.getElementById("lp2_cs_copy");
    lp2ClCopyEl = document.getElementById("lp2_cl_copy");
    lp2LsCopyEl = document.getElementById("lp2_ls_copy");
    lp2LlCopyEl = document.getElementById("lp2_ll_copy");
    lp2AllCopyEl = document.getElementById("lp2_all_copy");

    bp1CsCopyEl = document.getElementById("bp1_cs_copy");
    bp1ClCopyEl = document.getElementById("bp1_cl_copy");
    bp1LsCopyEl = document.getElementById("bp1_ls_copy");
    bp1LlCopyEl = document.getElementById("bp1_ll_copy");
    bp1AllCopyEl = document.getElementById("bp1_all_copy");

    bp2CsCopyEl = document.getElementById("bp2_cs_copy");
    bp2ClCopyEl = document.getElementById("bp2_cl_copy");
    bp2LsCopyEl = document.getElementById("bp2_ls_copy");
    bp2LlCopyEl = document.getElementById("bp2_ll_copy");
    bp2AllCopyEl = document.getElementById("bp2_all_copy");

    bp3CsCopyEl = document.getElementById("bp3_cs_copy");
    bp3ClCopyEl = document.getElementById("bp3_cl_copy");
    bp3LsCopyEl = document.getElementById("bp3_ls_copy");
    bp3LlCopyEl = document.getElementById("bp3_ll_copy");
    bp3AllCopyEl = document.getElementById("bp3_all_copy");

    bp4CsCopyEl = document.getElementById("bp4_cs_copy");
    bp4ClCopyEl = document.getElementById("bp4_cl_copy");
    bp4LsCopyEl = document.getElementById("bp4_ls_copy");
    bp4LlCopyEl = document.getElementById("bp4_ll_copy");
    bp4AllCopyEl = document.getElementById("bp4_all_copy");

    sigDigitsEl.addEventListener("change", (e) => {
        e.preventDefault();
        sig_digits = parseInt(sigDigitsEl.value, 10);
        update_imp();
    });

    modeUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        change_unit();
    });

    capUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        change_unit();
    });

    indUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        change_unit();
    });

    freqUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        change_unit();
    });

    impUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        change_imp();
    });

    z0El.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    qNetEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    qEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    freqEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    rsEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    xsEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    zsPasteEl.addEventListener("click", (e) => {
        e.preventDefault();
        pasteImpedance(zsPasteEl, rsEl, xsEl, update_imp);
    });

    rlEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    xlEl.addEventListener("change", (e) => {
        e.preventDefault();
        update_imp();
    });

    zlPasteEl.addEventListener("click", (e) => {
        e.preventDefault();
        pasteImpedance(zlPasteEl, rlEl, xlEl, update_imp);
    });

    calcEl.addEventListener("click", (e) => {
        e.preventDefault();
        update_imp();
    });

    hpclCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hpclCCopyEl, current.hp_ell_cl.c, current.hp_ell_cl.c_scale, sd);
    });
    hpclLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hpclLCopyEl, current.hp_ell_cl.l, current.hp_ell_cl.l_scale, sd);
    });
    hpclAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.hp_ell_cl;
        complexCopyWUnit(hpclAllCopyEl, net.c, net.c_scale, net.l, net.l_scale, sd);
    });

    hplcCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hplcCCopyEl, current.hp_ell_lc.c, current.hp_ell_lc.c_scale, sd);
    });
    hplcLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hplcLCopyEl, current.hp_ell_lc.l, current.hp_ell_lc.l_scale, sd);
    });
    hplcAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.hp_ell_lc;
        complexCopyWUnit(hplcAllCopyEl, net.l, net.l_scale, net.c, net.c_scale, sd);
    });

    lpclCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lpclCCopyEl, current.lp_ell_cl.c, current.lp_ell_cl.c_scale, sd);
    });
    lpclLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lpclLCopyEl, current.lp_ell_cl.l, current.lp_ell_cl.l_scale, sd);
    });
    lpclAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.lp_ell_cl;
        complexCopyWUnit(lpclAllCopyEl, net.c, net.c_scale, net.l, net.l_scale, sd);
    });

    lplcCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lplcCCopyEl, current.lp_ell_lc.c, current.lp_ell_lc.c_scale, sd);
    });
    lplcLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lplcLCopyEl, current.lp_ell_lc.l, current.lp_ell_lc.l_scale, sd);
    });
    lplcAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.lp_ell_lc;
        complexCopyWUnit(lplcAllCopyEl, net.l, net.l_scale, net.c, net.c_scale, sd);
    });

    hpclqCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hpclqCCopyEl, current.hp_ell_cl_w_q.c, current.hp_ell_cl_w_q.c_scale, sd);
    });
    hpclqLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hpclqLCopyEl, current.hp_ell_cl_w_q.l, current.hp_ell_cl_w_q.l_scale, sd);
    });
    hpclqAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.hp_ell_cl_w_q;
        complexCopyWUnit(hpclqAllCopyEl, net.c, net.c_scale, net.l, net.l_scale, sd);
    });

    hplcqCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hplcqCCopyEl, current.hp_ell_lc_w_q.c, current.hp_ell_lc_w_q.c_scale, sd);
    });
    hplcqLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hplcqLCopyEl, current.hp_ell_lc_w_q.l, current.hp_ell_lc_w_q.l_scale, sd);
    });
    hplcqAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.hp_ell_lc_w_q;
        complexCopyWUnit(hplcqAllCopyEl, net.l, net.l_scale, net.c, net.c_scale, sd);
    });

    lpclqCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lpclqCCopyEl, current.lp_ell_cl_w_q.c, current.lp_ell_cl_w_q.c_scale, sd);
    });
    lpclqLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lpclqLCopyEl, current.lp_ell_cl_w_q.l, current.lp_ell_cl_w_q.l_scale, sd);
    });
    lpclqAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.lp_ell_cl_w_q;
        complexCopyWUnit(lpclqAllCopyEl, net.c, net.c_scale, net.l, net.l_scale, sd);
    });

    lplcqCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lplcqCCopyEl, current.lp_ell_lc_w_q.c, current.lp_ell_lc_w_q.c_scale, sd);
    });
    lplcqLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lplcqLCopyEl, current.lp_ell_lc_w_q.l, current.lp_ell_lc_w_q.l_scale, sd);
    });
    lplcqAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.lp_ell_lc_w_q;
        complexCopyWUnit(lplcqAllCopyEl, net.l, net.l_scale, net.c, net.c_scale, sd);
    });

    piCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(piCCopyEl, current.pi.c, current.pi.c_scale, sd);
    });
    piCsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(piCsCopyEl, current.pi.cs, current.pi.c_scale, sd);
    });
    piClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(piClCopyEl, current.pi.cl, current.pi.c_scale, sd);
    });
    piLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(piLCopyEl, current.pi.l, current.pi.l_scale, sd);
    });
    piLsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(piLsCopyEl, current.pi.ls, current.pi.l_scale, sd);
    });
    piLlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(piLlCopyEl, current.pi.ll, current.pi.l_scale, sd);
    });
    pihpAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.pi;
        copyPiTee(pihpAllCopyEl, net.ls, net.l_scale, net.c, net.c_scale, net.ll, net.l_scale, sd);
    });
    pilpAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.pi;
        copyPiTee(pilpAllCopyEl, net.cs, net.c_scale, net.l, net.l_scale, net.cl, net.c_scale, sd);
    });

    teeCCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(teeCCopyEl, current.tee.c, current.tee.c_scale, sd);
    });
    teeCsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(teeCsCopyEl, current.tee.cs, current.tee.c_scale, sd);
    });
    teeClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(teeClCopyEl, current.tee.cl, current.tee.c_scale, sd);
    });
    teeLCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(teeLCopyEl, current.tee.l, current.tee.l_scale, sd);
    });
    teeLsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(teeLsCopyEl, current.tee.ls, current.tee.l_scale, sd);
    });
    teeLlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(teeLlCopyEl, current.tee.ll, current.tee.l_scale, sd);
    });
    teehpAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.tee;
        copyPiTee(teehpAllCopyEl, net.cs, net.c_scale, net.l, net.l_scale, net.cl, net.c_scale, sd);
    });
    teelpAllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.tee;
        copyPiTee(teelpAllCopyEl, net.ls, net.l_scale, net.c, net.c_scale, net.ll, net.l_scale, sd);
    });

    hp1CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp1CsCopyEl, current.hp1.cs, current.hp1.c_scale, sd);
    });
    hp1ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp1ClCopyEl, current.hp1.cl, current.hp1.c_scale, sd);
    });
    hp1LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp1LsCopyEl, current.hp1.ls, current.hp1.l_scale, sd);
    });
    hp1LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp1LlCopyEl, current.hp1.ll, current.hp1.l_scale, sd);
    });
    hp1AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.hp1;
        copyCCLL(hp1AllCopyEl, net.ls, net.l_scale, net.cs, net.c_scale, net.ll, net.l_scale, net.cl, net.c_scale, sd);
    });

    hp2CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp2CsCopyEl, current.hp2.cs, current.hp2.c_scale, sd);
    });
    hp2ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp2ClCopyEl, current.hp2.cl, current.hp2.c_scale, sd);
    });
    hp2LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp2LsCopyEl, current.hp2.ls, current.hp2.l_scale, sd);
    });
    hp2LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(hp2LlCopyEl, current.hp2.ll, current.hp2.l_scale, sd);
    });
    hp2AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.hp2;
        copyCCLL(hp2AllCopyEl, net.cs, net.c_scale, net.ls, net.l_scale, net.cl, net.c_scale, net.ll, net.l_scale, sd);
    });

    lp1CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp1CsCopyEl, current.lp1.cs, current.lp1.c_scale, sd);
    });
    lp1ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp1ClCopyEl, current.lp1.cl, current.lp1.c_scale, sd);
    });
    lp1LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp1LsCopyEl, current.lp1.ls, current.lp1.l_scale, sd);
    });
    lp1LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp1LlCopyEl, current.lp1.ll, current.lp1.l_scale, sd);
    });
    lp1AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.lp1;
        copyCCLL(lp1AllCopyEl, net.cs, net.c_scale, net.ls, net.l_scale, net.cl, net.c_scale, net.ll, net.l_scale, sd);
    });

    lp2CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp2CsCopyEl, current.lp2.cs, current.lp2.c_scale, sd);
    });
    lp2ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp2ClCopyEl, current.lp2.cl, current.lp2.c_scale, sd);
    });
    lp2LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp2LsCopyEl, current.lp2.ls, current.lp2.l_scale, sd);
    });
    lp2LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(lp2LlCopyEl, current.lp2.ll, current.lp2.l_scale, sd);
    });
    lp2AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.lp2;
        copyCCLL(lp2AllCopyEl, net.ls, net.l_scale, net.cs, net.c_scale, net.ll, net.l_scale, net.cl, net.c_scale, sd);
    });

    bp1CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp1CsCopyEl, current.bp1.cs, current.bp1.c_scale, sd);
    });
    bp1ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp1ClCopyEl, current.bp1.cl, current.bp1.c_scale, sd);
    });
    bp1LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp1LsCopyEl, current.bp1.ls, current.bp1.l_scale, sd);
    });
    bp1LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp1LlCopyEl, current.bp1.ll, current.bp1.l_scale, sd);
    });
    bp1AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.bp1;
        copyCCLL(bp1AllCopyEl, net.ls, net.l_scale, net.cs, net.c_scale, net.cl, net.c_scale, net.ll, net.l_scale, sd);
    });

    bp2CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp2CsCopyEl, current.bp2.cs, current.bp2.c_scale, sd);
    });
    bp2ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp2ClCopyEl, current.bp2.cl, current.bp2.c_scale, sd);
    });
    bp2LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp2LsCopyEl, current.bp2.ls, current.bp2.l_scale, sd);
    });
    bp2LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp2LlCopyEl, current.bp2.ll, current.bp2.l_scale, sd);
    });
    bp2AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.bp2;
        copyCCLL(bp2AllCopyEl, net.ls, net.l_scale, net.cs, net.c_scale, net.cl, net.c_scale, net.ll, net.l_scale, sd);
    });

    bp3CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp3CsCopyEl, current.bp3.cs, current.bp3.c_scale, sd);
    });
    bp3ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp3ClCopyEl, current.bp3.cl, current.bp3.c_scale, sd);
    });
    bp3LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp3LsCopyEl, current.bp3.ls, current.bp3.l_scale, sd);
    });
    bp3LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp3LlCopyEl, current.bp3.ll, current.bp3.l_scale, sd);
    });
    bp3AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.bp3;
        copyCCLL(bp3AllCopyEl, net.cs, net.c_scale, net.ls, net.l_scale, net.ll, net.l_scale, net.cl, net.c_scale, sd);
    });

    bp4CsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp4CsCopyEl, current.bp4.cs, current.bp4.c_scale, sd);
    });
    bp4ClCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp4ClCopyEl, current.bp4.cl, current.bp4.c_scale, sd);
    });
    bp4LsCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp4LsCopyEl, current.bp4.ls, current.bp4.l_scale, sd);
    });
    bp4LlCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        scalarCopyWUnit(bp4LlCopyEl, current.bp4.ll, current.bp4.l_scale, sd);
    });
    bp4AllCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        let net = current.bp4;
        copyCCLL(bp4AllCopyEl, net.cs, net.c_scale, net.ls, net.l_scale, net.ll, net.l_scale, net.cl, net.c_scale, sd);
    });
});
