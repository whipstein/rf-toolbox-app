export const verbose = 2;

//get dom elements
export var vmin_distanceEl = document.getElementById('vmin_distance');
export var vmax_distanceEl = document.getElementById('vmax_distance');

export var schematic = [];
export var dataPoints = [];
export var vswr = 0.0;
export var constQ = 0.0;
export var z0 = 50;
export var fontsize = 12;
export var color_of_smith_curves = 'bland';
export var trace_intensity = 'light';
//parameters
export var resolution = 100; // 100; //number of points per arc
export var span_resolution = 20;
export var precision = 3;

export let onchangeEl = [];
export let clickEl = [];

var colorList = {
  bland: {
    resistance_real: 'rgba(255, 0, 0, 0.1)',
    resistance_imaginary: 'rgba(255, 0, 0, 0.1)',
    admittance_real: 'rgba(0, 0, 255, 0.1)',
    admittance_imaginary: 'rgba(0, 0, 255, 0.1)',
    im: 'rgba(255, 0, 0, 0.1)',
    real: 'rgba(255, 0, 0, 0.1)',
    adm: 'rgba(0, 0, 255, 0.1)',
    sus: 'rgba(0, 0, 255, 0.1)',
    vswr: 'limegreen',
    constQ: 'mediumblue',
    markers: 'red',
  },
  colorful: {
    resistance_real: 'rgba(150, 0, 0, 0.1)',
    resistance_imaginary: 'rgba(252, 114, 2, 0.1)',
    admittance_real: 'rgba(255, 0, 250, 0.1)',
    admittance_imaginary: 'rgba(0, 10, 163, 0.1)',
    im: 'rgba(252, 114, 2, 0.1)',
    real: 'rgba(150, 0, 0, 0.1)',
    adm: 'rgba(255, 0, 250, 0.1)',
    sus: 'rgba(0, 10, 163, 0.1)',
    vswr: 'orangered',
    constQ: 'mediumblue',
    markers: 'red',
  },
};

export var colors = colorList.bland;

schematic.push({ type: 'raw', imp: 'diff', z0: 50, freq: 280, er: 1, freq_unit: { multiplier: 1e9 }, span: 0.0, span_unit: { multiplier: 1e9 } });

// schematic {
//    type: type of element
//    real: real part of normalized impedance
//    imaginary: imaginary part of normalized impedance
//    abs: array of the element values, e.g. for an inductor [Q, inductance]
//    unit: units for abs
//    tol: tolerance to use for the element values
// }
if (schematic[0].imp == 'diff') {
  schematic.push({ type: 'bb', real: 1, imaginary: 0, abs: [100, 0], unit: ['diff'], tol: 0 });
} else {
  schematic.push({ type: 'bb', real: 1, imaginary: 0, abs: [50, 0], unit: ['null'], tol: 0 });
}

export function update_constQ(q_new) {
  if (verbose >= 5) console.log('update_constQ(q_new: ' + q_new + ')');
  constQ = q_new;
  update_smith_chart();
}
export function update_vswr(vswr_new) {
  if (verbose >= 5) console.log('update_vswr(vswr_new: ' + vswr_new + ')');
  vswr = vswr_new;
  update_smith_chart();
}

export function toggle_color_scheme_fn() {
  if (verbose >= 5) console.log('toggle_color_scheme_fn(' + ')');
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
    colors = colorList.colorful;
  } else {
    color_of_smith_curves = 'bland';
    colors = colorList.bland;
  }

  update_smith_chart();
}

export function toggle_trace_intensity_fn() {
  if (verbose >= 5) console.log('toggle_color_scheme_fn(' + ')');

  if (trace_intensity == 'light') {
    trace_intensity = 'dark';
    colorList = {
      bland: {
        resistance_real: 'rgba(255, 0, 0, 0.3)',
        resistance_imaginary: 'rgba(255, 0, 0, 0.3)',
        admittance_real: 'rgba(0, 0, 255, 0.3)',
        admittance_imaginary: 'rgba(0, 0, 255, 0.3)',
        im: 'rgba(255, 0, 0, 0.3)',
        real: 'rgba(255, 0, 0, 0.3)',
        adm: 'rgba(0, 0, 255, 0.3)',
        sus: 'rgba(0, 0, 255, 0.3)',
        vswr: 'limegreen',
        constQ: 'mediumblue',
        markers: 'red',
      },
      colorful: {
        resistance_real: 'rgba(150, 0, 0, 0.3)',
        resistance_imaginary: 'rgba(252, 114, 2, 0.3)',
        admittance_real: 'rgba(255, 0, 250, 0.3)',
        admittance_imaginary: 'rgba(0, 10, 163, 0.3)',
        im: 'rgba(252, 114, 2, 0.3)',
        real: 'rgba(150, 0, 0, 0.3)',
        adm: 'rgba(255, 0, 250, 0.3)',
        sus: 'rgba(0, 10, 163, 0.3)',
        vswr: 'orangered',
        constQ: 'mediumblue',
        markers: 'red',
      },
    };
  } else {
    trace_intensity = 'light';
    colorList = {
      bland: {
        resistance_real: 'rgba(255, 0, 0, 0.1)',
        resistance_imaginary: 'rgba(255, 0, 0, 0.1)',
        admittance_real: 'rgba(0, 0, 255, 0.1)',
        admittance_imaginary: 'rgba(0, 0, 255, 0.1)',
        im: 'rgba(255, 0, 0, 0.1)',
        real: 'rgba(255, 0, 0, 0.1)',
        adm: 'rgba(0, 0, 255, 0.1)',
        sus: 'rgba(0, 0, 255, 0.1)',
        vswr: 'limegreen',
        constQ: 'mediumblue',
        markers: 'red',
      },
      colorful: {
        resistance_real: 'rgba(150, 0, 0, 0.1)',
        resistance_imaginary: 'rgba(252, 114, 2, 0.1)',
        admittance_real: 'rgba(255, 0, 250, 0.1)',
        admittance_imaginary: 'rgba(0, 10, 163, 0.1)',
        im: 'rgba(252, 114, 2, 0.1)',
        real: 'rgba(150, 0, 0, 0.1)',
        adm: 'rgba(255, 0, 250, 0.1)',
        sus: 'rgba(0, 10, 163, 0.1)',
        vswr: 'orangered',
        constQ: 'mediumblue',
        markers: 'red',
      },
    };
  }

  if (color_of_smith_curves == 'bland') {
    colors = colorList.bland;
  } else {
    colors = colorList.colorful;
  }

  update_smith_chart();
}

window.schematic = schematic;
window.toggle_color_scheme_fn = toggle_color_scheme_fn;
window.toggle_trace_intensity_fn = toggle_trace_intensity_fn;
