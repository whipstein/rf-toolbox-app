const { invoke } = window.__TAURI__.core;

function digits(val, sd = 2) {
    return val.toFixed(sd);
}

function precision(val, sd = 2) {
    return val.toPrecision(2);
}

function print_unit(unit, parse = false) {
    if (unit == "milli") {
        return "m";
    } else if (unit == "micro") {
        if (parse) {
            return "u";
        }
        return "μ";
    } else if (unit == "nano") {
        return "n";
    } else if (unit == "pico") {
        return "p";
    } else if (unit == "femto") {
        return "f";
    } else {
        return "";
    }
}

function print_val(val, unit, suffix, sd) {
    if (Number.isFinite(val)) {
        return "" + digits(val, sd) + " " + print_unit(unit) + suffix;
    }
    return "" + Number.NaN;
}

function print_cval(val, unit, suffix, sd, form = "ri") {
    if (Number.isFinite(val.re)) {
        if (form == "ri") {
            if (val.im < 0) {
                return "" + digits(val.re, sd) + " - " + Math.abs(digits(val.im, sd)) + print_unit(unit) + "j" + suffix;
            }
            return "" + digits(val.re, sd) + " + " + digits(val.im, sd) + print_unit(unit) + "j" + suffix;
        } else if (form == "ma") {
            return "" + digits(val.mag, sd) + " &angmsd; " + digits(val.ang, sd) + print_unit(unit) + suffix;
        }
        return "" + Number.NaN;
    }
    return "" + Number.NaN;
}

function scalarCopy(el, val, sd = 2) {
    if (Number.isFinite(val)) {
        invoke("copy_scalar", {x: digits(val, sd)});

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function scalarCopyWUnit(el, val, unit, sd = 2) {
    if (Number.isFinite(val)) {
        invoke("copy_scalar_w_unit", {x: digits(val, sd), unit: unit});

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function complexCopy(el, val1, val2, sd = 2) {
    if (Number.isFinite(val1) && Number.isFinite(val2)) {
        invoke("copy_complex", { re: digits(val1, sd), im: digits(val2, sd) });

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function complexCopyWUnit(el, val1, unit1, val2, unit2, sd = 2) {
    if (Number.isFinite(val1) && Number.isFinite(val2)) {
        invoke("copy_complex_w_unit", { re: digits(val1, sd), unit_re: unit1, im: digits(val2, sd), unit_im: unit2 });

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function rcCopy(el, val1, val2, unit, sd = 2) {
    if (Number.isFinite(val1) && Number.isFinite(val2)) {
        invoke("copy_rc", {r: digits(val1, sd), c: digits(val2, sd), unit: unit});

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function copyPiTee(el, val1, unit1, val2, unit2, val3, unit3, sd = 2) {
    if (Number.isFinite(val1) && Number.isFinite(val2) && Number.isFinite(val3)) {
        invoke("copy_pi_tee", { val1: digits(val1, sd), unit1: unit1, val2: digits(val2, sd), unit2: unit2, val3: digits(val3, sd), unit3: unit3 });

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function copyCCLL(el, val1, unit1, val2, unit2, val3, unit3, val4, unit4, sd = 2) {
    if (Number.isFinite(val1) && Number.isFinite(val2) && Number.isFinite(val3) && Number.isFinite(val4)) {
        invoke("copy_ccll", { val1: digits(val1, sd), unit1: unit1, val2: digits(val2, sd), unit2: unit2, val3: digits(val3, sd), unit3: unit3, val4: digits(val4, sd), unit4: unit4 });

        el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";
    
        setTimeout(()=> {
            el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
        },700);
    }
}

function pasteImpedance(el, el1, el2, f) {
    invoke("paste_impedance")
    .then((result) => {
        let val = result.split(/\s+/);
        if (!(/\d/g).test(val[1].charAt(val[1].length - 1))) {
            val[1] = val[1].slice(0, -1);
        }
        if(Number.isFinite(parseFloat(val[0])) && Number.isFinite(parseFloat(val[1]))) {
            el1.value = val[0];
            el2.value = val[1];

            el.innerHTML = "<i class=\"fa-solid fa-check\"></i>";

            setTimeout(()=> {
                el.innerHTML = "<i class=\"fa-regular fa-clipboard\"></i>";
            },700);

            f();
        }
    })
    .catch((error) => {
        console.log("ERROR: " + error);
    });
}


export {digits, precision, print_unit, print_val, print_cval, scalarCopy, scalarCopyWUnit, complexCopy, complexCopyWUnit, rcCopy, copyPiTee, copyCCLL, pasteImpedance};
