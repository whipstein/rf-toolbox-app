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
//parameters
export var resolution = 100; // 100; //number of points per arc
export var span_resolution = 20;
export var precision = 3;

export let onchangeEl = [];
export let clickEl = [];

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
  schematic.push({ type: 'bb', real: 1, imaginary: 0, abs: [100, 0], unit: ['null'], tol: 0 });
} else {
  schematic.push({ type: 'bb', real: 1, imaginary: 0, abs: [50, 0], unit: ['null'], tol: 0 });
}

export function update_constQ(q_new) {
  constQ = q_new;
  update_smith_chart();
}
export function update_vswr(vswr_new) {
  vswr = vswr_new;
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
  } else {
    color_of_smith_curves = 'bland';
  }

  update_smith_chart();
}

window.schematic = schematic;
window.toggle_color_scheme_fn = toggle_color_scheme_fn;
