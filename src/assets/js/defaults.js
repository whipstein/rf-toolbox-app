const { invoke } = window.__TAURI__.core;

//get dom elements
export var vmin_distanceEl = document.getElementById('vmin_distance');
export var vmax_distanceEl = document.getElementById('vmax_distance');

// export var schematic = [];
export var dataPoints = [];
export var vswr = 0.0;
export var constQ = 0.0;
export var const_gamma = 0.0;
export var z0 = 50;
export var fontsize = 10;
export var color_of_smith_curves = 'bland';
//parameters
export var resolution = 100; // 100; //number of points per arc
export var span_resolution = 20;
export var precision = 3;

export let onchangeEl = [];
export let clickEl = [];

let color = {
  bland: {
    resistance_real: 'rgba(255, 0, 0, 0.2)',
    resistance_imaginary: 'rgba(255, 0, 0, 0.2)',
    admittance_real: 'rgba(0, 0, 255, 0.2)',
    admittance_imaginary: 'rgba(0, 0, 255, 0.2)',
    im: 'rgba(255, 0, 0, 0.3)',
    real: 'rgba(255, 0, 0, 0.3)',
    adm: 'rgba(0, 0, 255, 0.3)',
    sus: 'rgba(0, 0, 255, 0.3)',
  },
  colorful: {
    resistance_real: 'rgba(150, 0, 0, 0.2)',
    resistance_imaginary: 'rgba(252, 114, 2, 0.2)',
    admittance_real: 'rgba(255, 0, 250, 0.2)',
    admittance_imaginary: 'rgba(0, 10, 163, 0.2)',
    im: 'rgba(252, 114, 2, 0.3)',
    real: 'rgba(150, 0, 0, 0.3)',
    adm: 'rgba(255, 0, 250, 0.3)',
    sus: 'rgba(0, 10, 163, 0.3)',
  },
};
export let current_color = color.bland;

// export let system = {
//   global: {
//     type: 'raw',
//     imp: 'diff',
//     z0: 50,
//     freq: 280,
//     er: 1,
//     freq_unit: { unit: 'ghz', multiplier: 1e9 },
//     span: 0.0,
//     span_unit: { multiplier: 1e9 },
//   },
//   schematic: [],
// };

function unscale(unit) {
  if (unit[0] == 'f') return 1e-15;
  else if (unit[0] == 'p') return 1e-12;
  else if (unit[0] == 'n') return 1e-9;
  else if (unit[0] == 'u' && unit[0] == 'μ') return 1e-6;
  else if (unit == 'm') return 1; //tl can have unit of meters
  else if (unit[0] == 'm') return 1e-3; //milli...
  else if (unit[0] == 'K' || unit[0] == 'k') return 1e3;
  else if (unit[0] == 'M') return 1e6;
  else return 1;
}

export class Settings {
  #imp = 'diff';
  #z0 = 50.0;
  #er = 1.0;
  #freq = { val: 280, unit: 'ghz' };
  #span = { val: 0.0, unit: 'ghz' };

  // constructor(imp = 'se', z0 = 50.0, er = 1.0, freq = { val: 280, unit: 'ghz' }, span = { val: 0.0, unit: 'ghz' }) {
  //   this.imp = imp;
  //   this.#z0 = z0;
  //   this.#er = er;
  //   this.#freq = freq;
  //   this.#span = span;
  // }

  get imp() {
    return this.#imp;
  }
  set imp(val) {
    if (val != 'se' && val != 'diff') {
      throw new Error('element type not recognized');
    }
    this.#imp = val;
  }
  get z0() {
    return this.#z0;
  }
  set z0(val) {
    this.#z0 = val;
  }
  get er() {
    return this.#er;
  }
  set er(val) {
    this.#er = val;
  }
  get freq() {
    return this.#freq.val * unscale(this.#freq.unit);
  }
  set freq(val) {
    this.#freq = val / unscale(this.#freq.unit);
  }
  get freq_scaled() {
    return this.#freq.val;
  }
  set freq_scaled(val) {
    this.#freq = val;
  }
  get freq_unit() {
    return this.#freq.unit;
  }
  set freq_unit(val) {
    this.#freq.unit = val;
  }
  get freq_multiplier() {
    return unscale(this.#freq.unit);
  }
  get span() {
    return this.#span.val * unscale(this.#span.unit);
  }
  set span(val) {
    this.#span = val / unscale(this.#span.unit);
  }
  get span_scaled() {
    return this.#span.val;
  }
  set span_scaled(val) {
    this.#span = val;
  }
  get span_unit() {
    return this.#span.unit;
  }
  set span_unit(val) {
    this.#span.unit = val;
  }
  get span_multiplier() {
    return unscale(this.#span.unit);
  }
}
export var settings = new Settings();

export class BlackBox {
  #type = 'bb';
  #r;
  #x;
  #abs;
  #tol;

  constructor(abs = [50.0, 0.0], tol = 0.0) {
    this.abs = abs;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_bb', {
      vals: this.#abs,
      z0: settings.z0,
      diff: settings.imp,
      verbose: true,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed BlackBox: calc_ri_bb: ' + error);
      });
  }
  get unit() {
    return [''];
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class ParallelResistor {
  #type = 'pr';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [50.0], unit = ['Ω'], tol = 0.0) {
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Resistor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val != 'mΩ' && type != 'Ω' && type != 'kΩ' && type != 'MΩ') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Resistor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class SeriesResistor {
  #type = 'sr';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [50.0], unit = ['Ω'], tol = 0.0) {
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Resistor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val != 'mΩ' && type != 'Ω' && type != 'kΩ' && type != 'MΩ') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Resistor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class ParallelCapacitor {
  #type = 'pc';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [0.0, 20.0], unit = ['Q', 'fF'], tol = 0.0) {
    this.type = mode;
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Capacitor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != 'mΩ' && val[0] != 'Ω' && val[0] != 'Q' && val[1] != 'mF' && val[1] != 'μF' && val[1] != 'nF' && val[1] != 'pF' && val[1] != 'fF') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Capacitor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class SeriesCapacitor {
  #type = 'sc';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [0.0, 20.0], unit = ['Q', 'fF'], tol = 0.0) {
    this.type = mode;
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Capacitor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != 'mΩ' && val[0] != 'Ω' && val[0] != 'Q' && val[1] != 'mF' && val[1] != 'μF' && val[1] != 'nF' && val[1] != 'pF' && val[1] != 'fF') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Capacitor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class ParallelInductor {
  #type = 'pi';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [20.0, 10.0], unit = ['Q', 'pH'], tol = 0.0) {
    this.#unit = unit;
    this.abs = abs;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Inductor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != 'mΩ' && val[0] != 'Ω' && val[0] != 'Q' && val[1] != 'H' && val[1] != 'mH' && val[1] != 'μH' && val[1] != 'nH' && val[1] != 'pH') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Inductor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class SeriesInductor {
  #type = 'si';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [20.0, 10.0], unit = ['Q', 'pH'], tol = 0.0) {
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Inductor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != 'mΩ' && val[0] != 'Ω' && val[0] != 'Q' && val[1] != 'H' && val[1] != 'mH' && val[1] != 'μH' && val[1] != 'nH' && val[1] != 'pH') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Inductor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class RLC {
  #type = 'rlc';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [1.0, 1.0, 1.0], unit = ['Ω', 'pH', 'fF'], tol = 0.0) {
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed RLC: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (
      val[0] != 'mΩ' &&
      val[0] != 'Ω' &&
      val[1] != 'H' &&
      val[1] != 'mH' &&
      val[1] != 'μH' &&
      val[1] != 'nH' &&
      val[1] != 'pH' &&
      val[2] != 'mF' &&
      val[2] != 'μF' &&
      val[2] != 'nF' &&
      val[2] != 'pF' &&
      val[2] != 'fF'
    ) {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed RLC: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}
export class CustomZ {
  #type = 'customZ';
  #r;
  #x;
  #lut;
  #interp;
  #raw;

  constructor(lut = [[280e9, 50.0, 0.0]], interp = 'linear', raw = '280e9,50.0,0.0') {
    this.lut = lut;
    this.interp = interp;
    this.raw = raw;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get lut() {
    return this.#lut;
  }
  set lut(val) {
    this.#lut = val;
    invoke('calc_ri_custom', {
      lut: this.#lut,
      interp: this.#interp,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed CustomZ: calc_ri_custom: ' + error);
      });
  }
  get interp() {
    return this.#interp;
  }
  set interp(val) {
    if (val != 'linear') {
      throw new Error('element type not recognized');
    }
    this.#interp = val;
  }
  get raw() {
    return this.#raw;
  }
  set raw(val) {
    this.#raw = val;
  }
}
export class TLine {
  #type;
  #r;
  #x;
  #abs;
  #unit;
  #line_z0;

  constructor(type, abs = [100.0], unit = ['μm'], line_z0 = 50.0) {
    this.type = type;
    this.abs = abs;
    this.unit = unit;
    this.line_z0 = line_z0;
  }

  get type() {
    return this.#type;
  }
  set type(val) {
    if (val != 'tl' || type != 'so' || type != 'ss') {
      this.#type = val;
    } else {
      throw new Error('element type not recognized');
    }
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != ' m' && val[1] != 'mm' && val[1] != 'μm' && val[1] != 'λ') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
  get line_z0() {
    return this.#line_z0;
  }
  set line_z0(val) {
    this.#line_z0 = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
}
export class ShortStub {
  #type = 'ss';
  #r;
  #x;
  #abs;
  #unit;
  #line_z0;

  constructor(abs = [100.0], unit = ['μm'], line_z0 = 50.0) {
    this.abs = abs;
    this.unit = unit;
    this.line_z0 = line_z0;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != ' m' && val[1] != 'mm' && val[1] != 'μm' && val[1] != 'λ') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
  get line_z0() {
    return this.#line_z0;
  }
  set line_z0(val) {
    this.#line_z0 = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
}
export class OpenStub {
  #type = 'so';
  #r;
  #x;
  #abs;
  #unit;
  #line_z0;

  constructor(abs = [100.0], unit = ['μm'], line_z0 = 50.0) {
    this.abs = abs;
    this.unit = unit;
    this.line_z0 = line_z0;
  }

  get type() {
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (val[0] != ' m' && val[1] != 'mm' && val[1] != 'μm' && val[1] != 'λ') {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
  get line_z0() {
    return this.#line_z0;
  }
  set line_z0(val) {
    this.#line_z0 = val;
    invoke('calc_ri_tline', {
      vals: this.#abs,
      units: this.#unit,
      line_z0: this.#line_z0,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      er: settings.er,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed TLine: calc_ri_tline: ' + error);
      });
  }
}
export class Transformer {
  #type = 'xfmr';
  #r;
  #x;
  #abs;
  #unit;
  #tol;

  constructor(abs = [20.0, 10.0, 10.0, 0.4], unit = ['Q', 'pH', 'pH', 'k'], tol = 0.0) {
    this.abs = abs;
    this.unit = unit;
    this.tol = tol;
  }

  get type() {
    console.log(this.#type);
    return this.#type;
  }
  get r() {
    return this.#r;
  }
  get x() {
    return this.#x;
  }
  get real() {
    return this.#r;
  }
  get imaginary() {
    return this.#x;
  }
  get abs() {
    return this.#abs;
  }
  set abs(val) {
    this.#abs = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Inductor: calc_ri_lumped: ' + error);
      });
  }
  get unit() {
    return this.#unit;
  }
  set unit(val) {
    if (
      val[0] != 'mΩ' &&
      val[0] != 'Ω' &&
      val[0] != 'Q' &&
      val[1] != 'H' &&
      val[1] != 'mH' &&
      val[1] != 'μH' &&
      val[1] != 'nH' &&
      val[1] != 'pH' &&
      val[2] != 'H' &&
      val[2] != 'mH' &&
      val[2] != 'μH' &&
      val[2] != 'nH' &&
      val[2] != 'pH' &&
      val[2] != 'n' &&
      val[3] != 'H' &&
      val[3] != 'mH' &&
      val[3] != 'μH' &&
      val[3] != 'nH' &&
      val[3] != 'pH' &&
      val[3] != 'k'
    ) {
      throw new Error('element unit not recognized');
    }
    this.#unit = val;
    invoke('calc_ri_lumped', {
      vals: this.#abs,
      units: this.#unit,
      type: this.#type,
      freq: settings.freq_scaled,
      freq_unit: settings.freq_unit,
      z0: settings.z0,
      verbose: false,
    })
      .then((result) => {
        [this.#r, this.#x] = result;
      })
      .catch((error) => {
        console.log('ERROR: failed Inductor: calc_ri_lumped: ' + error);
      });
  }
  get tol() {
    return this.#tol;
  }
  set tol(val) {
    this.#tol = val;
  }
}

export var schematic = [];
schematic.push(new BlackBox([50.0, 0.0]));

// schematic.push({ type: 'raw', imp: 'diff', z0: 50, freq: 280, er: 1, freq_unit: { multiplier: 1e9 }, span: 0.0, span_unit: { multiplier: 1e9 } });

// schematic {
//    type: type of element
//    real: real part of normalized impedance
//    imaginary: imaginary part of normalized impedance
//    abs: array of the element values, e.g. for an inductor [Q, inductance]
//    unit: units for abs
//    tol: tolerance to use for the element values
// }

// schematic.push({
//   type: 'bb',
//   r: parseFloat(1.0),
//   x: parseFloat(0.0),
//   abs: [parseFloat(100), parseFloat(0)],
//   unit: ['null'],
//   tol: parseFloat(0),
//   rin: parseFloat(0),
//   xin: parseFloat(0),
//   line_zo: 50.0,
//   line_length: 0.0,
// });
// if (settings.imp == 'diff') {
//   schematic.push({
//     type: 'bb',
//     r: parseFloat(1.0),
//     x: parseFloat(0.0),
//     abs: [parseFloat(100), parseFloat(0)],
//     unit: ['null'],
//     tol: parseFloat(0),
//     rin: parseFloat(0),
//     xin: parseFloat(0),
//     line_zo: 50.0,
//     line_length: 0.0,
//   });
// } else {
//   schematic.push({
//     type: 'bb',
//     r: parseFloat(1.0),
//     x: parseFloat(0.0),
//     abs: [parseFloat(50), parseFloat(0)],
//     unit: ['null'],
//     tol: parseFloat(0),
//     rin: parseFloat(0),
//     xin: parseFloat(0),
//     line_zo: 50.0,
//     line_length: 0.0,
//   });
// }

export function update_constQ(q_new) {
  constQ = q_new;
  update_smith_chart();
}
export function update_vswr(vswr_new) {
  vswr = vswr_new;
  update_smith_chart();
}
export function update_const_gamma(gamma_new) {
  const_gamma = gamma_new;
  update_smith_chart();
}

export function toggle_color_scheme_fn() {
  var element = document.getElementById('mainSection');
  var x = document.getElementsByClassName('bg-white');
  if (x.length > 0) {
    element.classList.remove('bg-white');
    element.classList.add('bg-green');
    document.getElementById('hollowed_circle').style['boxShadow'] = '0px 0px 0px 2000px rgb(184, 255, 241)';
  } else {
    element.classList.add('bg-white');
    element.classList.remove('bg-green');
    document.getElementById('hollowed_circle').style['boxShadow'] = '0px 0px 0px 2000px white';
  }

  if (color_of_smith_curves == 'bland') {
    color_of_smith_curves = 'colorful';
    current_color = color.colorful;
  } else {
    color_of_smith_curves = 'bland';
    current_color = color.bland;
  }

  update_smith_chart();
}

window.schematic = schematic;
window.toggle_color_scheme_fn = toggle_color_scheme_fn;
