const { invoke } = window.__TAURI__.core;
import {complexCopy, rcCopy} from "/util.js";

function digits(val, sd) {
    return parseFloat(val.toFixed(sd));
}

function printUnit(unit, parse = false) {
    switch (unit) {
        case "milli":
            return "m";
        case "micro":
            if (parse) {
                return "u";
            }
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
  
function printVal(val, unit, suffix, sd) {
    if (Number.isFinite(val)) {
        return "" + digits(val, sd) + printUnit(unit) + suffix;
    }
    return "" + val;
}

function calcMatch() {
    getVals();

    invoke("calc_vals", {re: re, im: im, imp: numFormat, z0: z0, freq: freq, f_scale: freqUnit, r_scale: "", c_scale: capUnit})
        .then((result) => {
            current = result;

            var txt = "<div class=\"text_box\">" + printVal(result.z_re, "", "", sd);
            if (result.z_im < 0) txt += " - ";
            else txt += " + ";
            txt += printVal(Math.abs(result.z_im), "", "j Ω</div>", sd);
            zValEl.innerHTML = txt;

            var txt = "<div class=\"text_box\">" + printVal(result.g_re, "", "", sd);
            if (result.g_im < 0) txt += " - ";
            else txt += " + ";
            txt += printVal(Math.abs(result.g_im), "", "j</div>", sd);
            gammaRiValEl.innerHTML = txt;
        
            var txt = "<div class=\"text_box\">" + printVal(result.g_mag, "", " &angmsd; ", sd);
            txt += printVal(result.g_ang, "", "&deg; </div>", sd);
            gammaMaValEl.innerHTML = txt;
        
            var txt = "<div class=\"text_box\">" + printVal(result.r, "", " Ω</div>", sd);
            rValEl.innerHTML = txt;
            var txt = "<div class=\"text_box\">" + printVal(result.c, capUnit, "F</div>", sd);
            cValEl.innerHTML = txt;
        })
        .catch((err) => {
            console.log("ERROR: " + err);
            var txt = "<div class=\"text_box\">ERROR";
            zValEl.innerHTML = txt;
            gammaRiValEl.innerHTML = txt;
            gammaMaValEl.innerHTML = txt;
            rValEl.innerHTML = txt;
            cValEl.innerHTML = txt;
            zRe = Number.NaN;
            zIm = Number.NaN;
            gammaRe = Number.NaN;
            gammaIm = Number.NaN;
            gammaMag = Number.NaN;
            gammaAng = Number.NaN;
            r = Number.NaN;
            c = Number.NaN;
        });
}

function changeImp() {
    getVals();

    let reTxt, imTxt;

    switch (numFormat) {
        case "z":
            reTxt = "Z Real";
            imTxt = "Z Imag";
            break;
        case "ri":
            reTxt = "Γ Real";
            imTxt = "Γ Imag";
            break;
        case "ma":
            reTxt = "Γ Mag";
            imTxt = "Γ Angle (&deg;)";
            break;
        case "db":
            reTxt = "Γ Mag (dB)";
            imTxt = "Γ Angle (&deg;)";
            break;
        case "rc":
            reTxt = "Resistance";
            imTxt = "Capacitance";
            break;
        default:
            reTxt = "ERROR";
            imTxt = "ERROR";
    }

    s11ReLabelEl.innerHTML = "<div>" + reTxt + "</div>";
    s11ImLabelEl.innerHTML = "<div>" + imTxt + "</div>";

    calcMatch();
}

function getVals() {
    sd = sigDigitsEl.value;
    numFormat = numFormatEl.value;
    freqUnit = freqUnitEl.value;
    capUnit = capUnitEl.value;
    z0 = parseFloat(z0El.value);
    freq = parseFloat(freqEl.value);
    re = parseFloat(s11ReEl.value);
    im = parseFloat(s11ImEl.value);
}

let sd = 2;
let numFormat, freqUnit, capUnit, z0, freq, re, im;
let numFormatEl, capUnitEl, freqUnitEl, freqEl, z0El, sigDigitsEl, s11ReLabelEl, s11ReEl, s11ImLabelEl, s11ImEl, gCopyEl, gMaCopyEl, zCopyEl, rcCopyEl, points;
let zValEl, gammaRiValEl, gammaMaValEl, rValEl, cValEl;
let current;

window.addEventListener("DOMContentLoaded", () => {
    numFormatEl = document.getElementById("numFormat");
    capUnitEl = document.getElementById("capUnit");
    freqUnitEl = document.getElementById("freqUnit");
    freqEl = document.getElementById("freq");
    z0El = document.getElementById("z0");
    sigDigitsEl = document.getElementById("sigDigits");
    s11ReLabelEl = document.getElementById("s11ReLabel");
    s11ImLabelEl = document.getElementById("s11ImLabel");
    s11ReEl = document.getElementById("s11Re");
    s11ImEl = document.getElementById("s11Im");
    zValEl = document.getElementById("zVal");
    gammaRiValEl = document.getElementById("gammaRiVal");
    gammaMaValEl = document.getElementById("gammaMaVal");
    rValEl = document.getElementById("rVal");
    cValEl = document.getElementById("cVal");
    gCopyEl = document.getElementById("gCopy");
    gMaCopyEl = document.getElementById("gMaCopy");
    zCopyEl = document.getElementById("zCopy");
    rcCopyEl = document.getElementById("rcCopy");

    getVals();

    numFormatEl.addEventListener("change", (e) => {
        e.preventDefault();
        changeImp();
    });

    capUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        calcMatch();
    });

    freqUnitEl.addEventListener("change", (e) => {
        e.preventDefault();
        calcMatch();
    });

    freqEl.addEventListener("change", (e) => {
        e.preventDefault();
        calcMatch();
    });

    z0El.addEventListener("change", (e) => {
        e.preventDefault();
        calcMatch();
    });

    sigDigitsEl.addEventListener("change", (e) => {
        e.preventDefault();
        sd = parseInt(sigDigitsEl.value, 10);
        calcMatch();
    });

    s11ReEl.addEventListener("change", (e) => {
        e.preventDefault();
        calcMatch();
    });

    s11ImEl.addEventListener("change", (e) => {
        e.preventDefault();
        calcMatch();
    });

    gCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(gCopyEl, current.g_re, current.g_im, sd);
    });

    gMaCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(gMaCopyEl, current.g_mag, current.g_ang, sd);
    });

    zCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        complexCopy(zCopyEl, current.z_re, current.z_im, sd);
    });

    rcCopyEl.addEventListener("click", (e) => {
        e.preventDefault();
        rcCopy(rcCopyEl, current.r, current.c, capUnit, sd);
    });
});
