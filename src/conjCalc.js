const { invoke } = window.__TAURI__.core;
import {print_val, print_cval, complexCopy, rcCopy, pasteImpedance} from "/util.js";

function calcMatch() {
    invoke("calc_match", { s11re: s11re, s11im: s11im, s12re: s12re, s12im: s12im, s21re: s21re, s21im: s21im, s22re: s22re, s22im: s22im, imp: imp_unit, z0: z0, freq: freq, fscale: freq_unit, cscale: cap_unit })
        .then((result) => {
            kEl.innerHTML = "<div class=\"text_box\">" + print_val(result.k, "", " </div>", sd);
            b1El.innerHTML = "<div class=\"text_box\">" + print_val(result.b1, "", " </div>", sd);
            magEl.innerHTML = "<div class=\"text_box\">" + print_val(result.mag, "", "dB </div>", sd);

            current.src_gamma_mag = result.src.gamma.mag;
            current.src_gamma_ang = result.src.gamma.ang;
            current.src_gamma_re = result.src.gamma.re;
            current.src_gamma_im = result.src.gamma.im;
            current.src_z_re = result.src.z.re;
            current.src_z_im = result.src.z.im;
            current.src_r = result.src.r;
            current.src_c = result.src.c;
            srcGammaEl.innerHTML = "<div class=\"text_box\">" + print_cval(result.src.gamma, "", "&deg; </div>", sd, "ma")
            srcGammaRiEl.innerHTML = "<div class=\"text_box\">" + print_cval(result.src.gamma, "", "&deg; </div>", sd, "ri")
            srcZEl.innerHTML = "<div class=\"text_box\">" + print_cval(result.src.z, "", "j Ω</div>", sd)
            srcREl.innerHTML = "<div class=\"text_box\">" + print_val(result.src.r, "", " Ω</div>", sd)
            srcCEl.innerHTML = "<div class=\"text_box\">" + print_val(result.src.c, cap_unit, "F</div>", sd)

            current.load_gamma_mag = result.load.gamma.mag;
            current.load_gamma_ang = result.load.gamma.ang;
            current.load_gamma_re = result.load.gamma.re;
            current.load_gamma_im = result.load.gamma.im;
            current.load_z_re = result.load.z.re;
            current.load_z_im = result.load.z.im;
            current.load_r = result.load.r;
            current.load_c = result.load.c;
            loadGammaEl.innerHTML = "<div class=\"text_box\">" + print_cval(result.load.gamma, "", "&deg; </div>", sd, "ma")
            loadGammaRiEl.innerHTML = "<div class=\"text_box\">" + print_cval(result.load.gamma, "", "&deg; </div>", sd, "ri")
            loadZEl.innerHTML = "<div class=\"text_box\">" + print_cval(result.load.z, "", "j Ω</div>", sd)
            loadREl.innerHTML = "<div class=\"text_box\">" + print_val(result.load.r, "", " Ω</div>", sd)
            loadCEl.innerHTML = "<div class=\"text_box\">" + print_val(result.load.c, cap_unit, "F</div>", sd)
        })
        .catch((err) => {
            console.log("ERROR: " + err);
            var txt = "<div class=\"text_box\">ERROR";    
            kEl.innerHTML = txt;
            b1El.innerHTML = txt;
            magEl.innerHTML = txt;
            srcGammaEl.innerHTML = txt;
            srcZEl.innerHTML = txt;
            srcREl.innerHTML = txt;
            srcCEl.innerHTML = txt;
            loadGammaEl.innerHTML = txt;
            loadZEl.innerHTML = txt;
            loadREl.innerHTML = txt;
            loadCEl.innerHTML = txt;
        });
}

function updateVals() {
    sd = parseInt(sigDigitsEl.value);
    cap_unit = capUnitEl.value;
    freq_unit = freqUnitEl.value;
    imp_unit = impUnitEl.value;
    z0 = parseFloat(z0El.value);
    freq = parseFloat(freqEl.value);
    s11re = parseFloat(s11reEl.value);
    s11im = parseFloat(s11imEl.value);
    s12re = parseFloat(s12reEl.value);
    s12im = parseFloat(s12imEl.value);
    s21re = parseFloat(s21reEl.value);
    s21im = parseFloat(s21imEl.value);
    s22re = parseFloat(s22reEl.value);
    s22im = parseFloat(s22imEl.value);

    calcMatch();
}

function updateLabels() {
    imp_unit = impUnitEl.value;

    let reLabel, imLabel;

    if (imp_unit == "ma" || imp_unit == "db") {
        reLabel = "&ang;";
        imLabel = "&deg;";
    } else {
        reLabel = "+";
        imLabel = "j";
    }

    s11reLabelEl.innerHTML = reLabel;
    s12reLabelEl.innerHTML = reLabel;
    s21reLabelEl.innerHTML = reLabel;
    s22reLabelEl.innerHTML = reLabel;

    s11imLabelEl.innerHTML = imLabel;
    s12imLabelEl.innerHTML = imLabel;
    s21imLabelEl.innerHTML = imLabel;
    s22imLabelEl.innerHTML = imLabel;

    updateVals();
}

let sigDigitsEl, capUnitEl, freqUnitEl, impUnitEl, z0El, freqEl, s11reLabelEl, s11imLabelEl, s11PasteEl, s12reLabelEl, s12imLabelEl, s12PasteEl, s21reLabelEl, s21imLabelEl, s21PasteEl, s22reLabelEl, s22imLabelEl, s22PasteEl, s11reEl, s11imEl, s12reEl, s12imEl, s21reEl, s21imEl, s22reEl, s22imEl, calcEl;
let sd, cap_unit, freq_unit, imp_unit, z0, freq, s11re, s11im, s12re, s12im, s21re, s21im, s22re, s22im;
let kEl, b1El, magEl, srcGammaEl, srcGammaRiEl, srcZEl, srcREl, srcCEl, loadGammaEl, loadGammaRiEl, loadZEl, loadREl, loadCEl;
let srcGammaCopyEl, srcGammaRiCopyEl, srcZCopyEl, srcRcCopyEl, loadGammaCopyEl, loadGammaRiCopyEl, loadZCopyEl, loadRcCopyEl;

let current = {
    src_gamma_mag: 0.0,
    src_gamma_mag: 0.0,
    src_gamma_re: 0.0,
    src_gamma_im: 0.0,
    src_z_re: 0.0,
    src_z_im: 0.0,
    src_r: 0.0,
    src_c: 0.0,
    load_gamma_mag: 0.0,
    load_gamma_mag: 0.0,
    load_gamma_re: 0.0,
    load_gamma_im: 0.0,
    load_z_re: 0.0,
    load_z_im: 0.0,
    load_r: 0.0,
    load_c: 0.0,
}

window.addEventListener("DOMContentLoaded", () => {
    sigDigitsEl = document.getElementById("sig_digits");
    capUnitEl = document.getElementById("cap_unit");
    freqUnitEl = document.getElementById("freq_unit");
    impUnitEl = document.getElementById("imp_unit");
    z0El = document.getElementById("z0");
    freqEl = document.getElementById("freq");
    s11reLabelEl = document.getElementById("s11_re_label");
    s11imLabelEl = document.getElementById("s11_im_label");
    s12reLabelEl = document.getElementById("s12_re_label");
    s12imLabelEl = document.getElementById("s12_im_label");
    s21reLabelEl = document.getElementById("s21_re_label");
    s21imLabelEl = document.getElementById("s21_im_label");
    s22reLabelEl = document.getElementById("s22_re_label");
    s22imLabelEl = document.getElementById("s22_im_label");
    s11reEl = document.getElementById("s11_re");
    s11imEl = document.getElementById("s11_im");
    s11PasteEl = document.getElementById("s11_paste");
    s12reEl = document.getElementById("s12_re");
    s12imEl = document.getElementById("s12_im");
    s12PasteEl = document.getElementById("s12_paste");
    s21reEl = document.getElementById("s21_re");
    s21imEl = document.getElementById("s21_im");
    s21PasteEl = document.getElementById("s21_paste");
    s22reEl = document.getElementById("s22_re");
    s22imEl = document.getElementById("s22_im");
    s22PasteEl = document.getElementById("s22_paste");
    calcEl = document.getElementById("calc");
    srcGammaCopyEl = document.getElementById("src_gamma_copy");
    srcGammaRiCopyEl = document.getElementById("src_gamma_ri_copy");
    srcZCopyEl = document.getElementById("src_z_copy");
    srcRcCopyEl = document.getElementById("src_rc_copy");
    loadGammaCopyEl = document.getElementById("load_gamma_copy");
    loadGammaRiCopyEl = document.getElementById("load_gamma_ri_copy");
    loadZCopyEl = document.getElementById("load_z_copy");
    loadRcCopyEl = document.getElementById("load_rc_copy");

    kEl = document.getElementById("k_val");
    b1El = document.getElementById("b1_val");
    magEl = document.getElementById("mag_val");
    srcGammaEl = document.getElementById("src_gamma_val");
    srcGammaRiEl = document.getElementById("src_gamma_ri_val");
    srcZEl = document.getElementById("src_z_val");
    srcREl = document.getElementById("src_r_val");
    srcCEl = document.getElementById("src_c_val");
    loadGammaEl = document.getElementById("load_gamma_val");
    loadGammaRiEl = document.getElementById("load_gamma_ri_val");
    loadZEl = document.getElementById("load_z_val");
    loadREl = document.getElementById("load_r_val");
    loadCEl = document.getElementById("load_c_val");

    sigDigitsEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });

    capUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });

    freqUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });

    impUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateLabels();
    });

    z0El.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });

    freqEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });

    s11reEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s11imEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s11PasteEl.addEventListener("click", (e) => {
        e.preventDefault();
        pasteImpedance(s11PasteEl, s11reEl, s11imEl, updateVals);
    });

    s12reEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s12imEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s12PasteEl.addEventListener("click", (e) => {
        e.preventDefault();
        pasteImpedance(s12PasteEl, s12reEl, s12imEl, updateVals);
    });

    s21reEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s21imEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s21PasteEl.addEventListener("click", (e) => {
        e.preventDefault();
        pasteImpedance(s21PasteEl, s21reEl, s21imEl, updateVals);
    });

    s22reEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s22imEl.addEventListener("change", (e) => {
        e.preventDefault();
        updateVals();
    });
    s22PasteEl.addEventListener("click", (e) => {
        e.preventDefault();
        pasteImpedance(s22PasteEl, s22reEl, s22imEl, updateVals);
    });

    calcEl.addEventListener("click", (e) => {
        e.preventDefault();
        updateVals();
    });

    srcGammaCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(srcGammaCopyEl, current.src_gamma_mag, current.src_gamma_ang, sd);
    });

    srcGammaRiCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(srcGammaRiCopyEl, current.src_gamma_re, current.src_gamma_im, sd);
    });

    srcZCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(srcZCopyEl, current.src_z_re, current.src_z_im, sd);
    });

    srcRcCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        rcCopy(srcRcCopyEl, current.src_r, current.src_c, cap_unit, sd);
    });

    loadGammaCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(loadGammaCopyEl, current.load_gamma_mag, current.load_gamma_ang, sd);
    });

    loadGammaRiCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(loadGammaRiCopyEl, current.load_gamma_re, current.load_gamma_im, sd);
    });

    loadZCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(loadZCopyEl, current.load_z_re, current.load_z_im, sd);
    });

    loadRcCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        rcCopy(loadRcCopyEl, current.load_r, current.load_c, cap_unit, sd);
    });
});
