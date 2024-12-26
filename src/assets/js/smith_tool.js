const { invoke } = window.__TAURI__.core;
import {
  vmin_distanceEl,
  vmax_distanceEl,
  schematic,
  dataPoints,
  vswr,
  constQ,
  z0,
  fontsize,
  color_of_smith_curves,
  resolution,
  span_resolution,
  precision,
  onchangeEl,
  clickEl,
} from './defaults.js';
import { layout, configure_layout_shapes, draw_schematic, show_labels_DP, show_labels_res, show_labels_adm } from './draw.js';
import { customMarkers, drawMakerTable } from './marker.js';
import { one_over_complex, expo, unitTextToNum, domFreqSel } from './util.js';

export function clicked_cell(type) {
  if (type == 'pr') {
    schematic.push({ type: 'pr', real: 0, imaginary: 0, abs: [50], unit: ['Ω'], tol: 0 });
  } else if (type == 'sr') {
    schematic.push({ type: 'sr', real: 0, imaginary: 0, abs: [50], unit: ['Ω'], tol: 0 });
  } else if (type == 'si') {
    schematic.push({ type: 'si', real: 0, imaginary: 0, abs: [20, 1], unit: ['Q', 'pH'], tol: 0 });
  } else if (type == 'pi') {
    schematic.push({ type: 'pi', real: 0, imaginary: 0, abs: [20, 1], unit: ['Q', 'pH'], tol: 0 });
  } else if (type == 'sc') {
    schematic.push({ type: 'sc', real: 0, imaginary: 0, abs: [0, 1], unit: ['Q', 'fF'], tol: 0 });
  } else if (type == 'pc') {
    schematic.push({ type: 'pc', real: 0, imaginary: 0, abs: [0, 1], unit: ['Q', 'fF'], tol: 0 });
  } else if (type == 'tl') {
    schematic.push({ type: 'tl', line_length: 1e-6, abs: [1], line_zo: 50, unit: ['um'], real: 0, imaginary: 0, tol: 0 });
  } else if (type == 'ss') {
    schematic.push({ type: 'ss', line_length: 1e-6, abs: [1], line_zo: 50, unit: ['um'], real: 0, imaginary: 0, tol: 0 });
  } else if (type == 'so') {
    schematic.push({ type: 'so', line_length: 1e-6, abs: [1], line_zo: 50, unit: ['um'], real: 0, imaginary: 0, tol: 0 });
  } else if (type == 'xfmr') {
    schematic.push({ type: 'xfmr', real: 0, imaginary: 0, abs: [20, 1, 1, 0.4], unit: ['Q', 'pH', 'pH', 'k'], tol: 0 });
  } else if (type == 'rlc') {
    schematic.push({ type: 'rlc', real: 0, imaginary: 0, abs: [50, 1, 1], unit: ['Ω', 'pH', 'fF'], tol: 0 });
  } else if (type == 'customZ') {
    schematic.push({
      type: 'customZ',
      real: 0,
      imaginary: 0,
      abs: [50, 1, 1],
      unit: ['Ω', 'nH', 'pF'],
      lut: [[2440e6, 50, 50]],
      interp: 'linear',
      raw: '2440e6,50,50',
      tol: 0,
    });
  }
  update_smith_chart();
}

export function update_schem_abs(target_num, obj, absCounter) {
  var complex = obj.name;
  // console.log('dbg0',target_num, obj.value, complex)
  switch (schematic[target_num].type) {
    case 'bb':
      // console.log('dbg1',target_num, obj.value, complex)
      if (complex == 'abs') schematic[target_num].abs[absCounter] = Number(obj.value);
      else schematic[target_num].abs_bb_i = Number(obj.value);
      break;
    case 'tl':
    case 'ss':
    case 'so':
      if (complex == 'abs') schematic[target_num].abs[absCounter] = Number(obj.value);
      else if (complex == 'line_zo') schematic[target_num].line_zo = Number(obj.value);
      break;
    case 'rc':
    case 'rl':
    case 'rlc':
    case 'bb':
    case 'sr':
    case 'pr':
    case 'pc':
    case 'sc':
    case 'pi':
    case 'si':
    case 'xfmr':
      schematic[target_num].abs[absCounter] = Number(obj.value);
      break;
  }
  update_smith_chart();
}

export async function update_schem_component(freq_here, save_impedance, sch_index) {
  var re_here = 0;
  var im_here = 0;
  var ln_here = 0;
  var scaler = [];
  var i = 0;
  for (i = 0; i < schematic[sch_index].unit.length; i++) {
    scaler[i] = unitTextToNum(schematic[sch_index].unit[i], freq_here);
  }

  let temp_diff = false;
  if (schematic[0].imp == 'diff') {
    temp_diff = true;
  }
  let lut = [[0.0, 0.0, 0.0]];
  if (schematic[sch_index].type == 'customZ') {
    lut = schematic[sch_index].lut;
  }
  await invoke('calc_ri', {
    vals: schematic[sch_index].abs,
    units: schematic[sch_index].unit,
    lut: lut,
    type: schematic[sch_index].type,
    freq: freq_here,
    z0: z0,
    diff: temp_diff,
    verbose: false,
  })
    .then((result) => {
      re_here = result[0];
      im_here = result[1];
      ln_here = result[2];

      if (save_impedance) {
        var re_out, im_out;

        if (schematic[sch_index].type == 'xfmr') {
          var out = one_over_complex(re_here[0], im_here[0]);
          var m = one_over_complex(re_here[1], im_here[1]);
          out = one_over_complex(out[0] + m[0], out[1] + m[1]);
          re_out = out[0];
          im_out = out[1];
        } else {
          re_out = re_here;
          im_out = im_here;
        }

        if (Math.abs(re_out) < 0.1 && re_out != 0) {
          schematic[sch_index].real = expo(re_out, 2);
        } else {
          schematic[sch_index].real = Number(re_out).toFixed(precision);
        }

        if (Math.abs(im_out) < 0.1 && im_out != 0) {
          schematic[sch_index].imaginary = expo(im_out, 2);
        } else {
          schematic[sch_index].imaginary = Number(im_out).toFixed(precision);
        }

        schematic[sch_index].line_length = ln_here;
      }
    })
    .catch((error) => {
      console.log('ERROR (smith_tool.js (update_schem_component): ' + error);
    });
  return [re_here, im_here, ln_here];
}

export function impedanceToReflectionCoefficient(real_old, imag_old, z0) {
  //Calculate the reflection coefficient -current_admittance (z0-zimp) / (z0+zimp)
  var bot_real, bot_imag;
  let temp_array = one_over_complex(real_old * z0 + z0, imag_old * z0);
  bot_real = temp_array[0];
  bot_imag = temp_array[1];
  var reflectio_coeff_real = (real_old * z0 - z0) * bot_real - imag_old * z0 * bot_imag;
  var reflectio_coeff_imag = imag_old * z0 * bot_real + (real_old * z0 - z0) * bot_imag;
  var reflection_mag = Math.sqrt(reflectio_coeff_real * reflectio_coeff_real + reflectio_coeff_imag * reflectio_coeff_imag);
  if (reflectio_coeff_real == 0) var reflection_phase = 0;
  else var reflection_phase = (360 * Math.atan(reflectio_coeff_imag / reflectio_coeff_real)) / (2 * Math.PI);
  if (reflectio_coeff_real < 0) reflection_phase += 180;
  if (reflection_phase < 0) reflection_phase = 360 + reflection_phase;
  return [reflectio_coeff_real, reflectio_coeff_imag, reflection_mag, reflection_phase];
}

export function calcOutputValues(real_old, imag_old, temp_array) {
  //Update the impedance box
  var txt = '<div class="text_box">';
  txt += (real_old * z0).toFixed(precision);
  if (imag_old < 0) txt += ' - ';
  else txt += ' + ';
  txt += Math.abs(imag_old * z0).toFixed(precision) + 'j</div>';
  document.getElementById('current_impedance').innerHTML = txt;

  //Calculate the admittance
  var admittance_real, admittance_imaginary;
  temp_array = one_over_complex(real_old * z0, imag_old * z0);
  admittance_real = temp_array[0];
  admittance_imaginary = temp_array[1];
  txt = '<div class="text_box">' + admittance_real.toFixed(precision);
  if (admittance_imaginary < 0) txt += ' - ';
  else txt += ' + ';
  txt += Math.abs(admittance_imaginary).toFixed(precision) + 'j</div>';
  document.getElementById('current_admittance').innerHTML = txt;

  //Calculate the reflection coefficient -current_admittance (z0-zimp) / (z0+zimp)
  var reflectio_coeff_real, reflectio_coeff_imag, reflection_mag, reflection_phase;
  [reflectio_coeff_real, reflectio_coeff_imag, reflection_mag, reflection_phase] = impedanceToReflectionCoefficient(real_old, imag_old, z0);
  txt = '<div class="text_box">' + reflectio_coeff_real.toFixed(precision);
  if (reflectio_coeff_imag < 0) txt += ' - ';
  else txt += ' + ';
  txt += Math.abs(reflectio_coeff_imag).toFixed(precision) + 'j</div>';
  document.getElementById('current_reflection').innerHTML = txt;
  txt = '<div class="text_box">' + reflection_mag.toFixed(precision);
  txt += ' &ang; ';
  txt += reflection_phase.toFixed(precision) + '&deg; </div>';
  document.getElementById('current_reflection_mag').innerHTML = txt;

  //calculate VSWR (1+r) / (1-r)
  var vswr_live = (1 + reflection_mag) / (1 - reflection_mag);
  document.getElementById('vswr_live').innerHTML = '<div class="text_box">' + vswr_live.toFixed(precision) + ':1</div>';

  //calculate RL 20*log10(r)
  var rl_live = 20 * Math.log10(reflection_mag);
  document.getElementById('rl_live').innerHTML = '<div class="text_box">' + rl_live.toFixed(precision) + '</div>';

  //populate vmin_distanceEl and vmax_distanceEl
  vmax_distanceEl.value = ((0.5 * reflection_phase) / 360).toFixed(precision);

  if (reflection_phase > 180) vmin_distanceEl.value = ((0.5 * (reflection_phase - 180)) / 360).toFixed(precision);
  else vmin_distanceEl.value = ((0.5 * (reflection_phase + 180)) / 360).toFixed(precision);

  return [reflection_mag, reflection_phase];
}

export function addEvents() {
  console.log(onchangeEl);
  for (let i = 0; i < onchangeEl.length; i++) {
    let cell = onchangeEl[i];
    let el = document.getElementById(cell.el);
    let args = '' + cell.f + '(';
    for (let k = 0; k < cell.args.length; k++) {
      if (k > 0) {
        args += ', ';
      }
      args += cell.args[k];
    }
    args += ')';
    console.log(document.getElementById(cell.el));
    console.log(cell.el);
    console.log(args);

    el.addEventListener('onchange', (e) => {
      e.preventDefault();
      eval(args);
    });
  }
  console.log(clickEl);
  for (let i = 0; i < clickEl.length; i++) {
    let el = document.getElementById(clickEl[i].el);
    let args = '' + clickEl[i].f + '(';
    for (let k = 0; k < clickEl[i].args.length; k++) {
      if (k > 0) {
        args += ', ';
      }
      if (clickEl[i].args[k] == 'this') {
        args += el.value;
      } else {
        args += clickEl[i].args[k];
      }
    }
    args += ')';

    el.addEventListener('click', (e) => {
      e.preventDefault();
      eval(args);
    });
  }
}

//TODO - A big improvement here would be to separate out the impedance calculation and arc drawing. It should calculate impedances, then calculate points along the arc
export async function update_smith_chart() {
  //Update the layout variable
  layout.shapes = configure_layout_shapes();
  //Calculate and verify freqeuencies...
  freq = schematic[0].freq * schematic[0].freq_unit.multiplier;
  let span_freq = schematic[0].span * schematic[0].span_unit.multiplier;
  //console.log(schematic[0].freq * schematic[0].freq_unit.multiplier,schematic[0].span * schematic[0].span_unit.multiplier)
  if (freq < span_freq) {
    swal({
      type: 'error',
      title: 'Oops...',
      text: 'Span is larger than frequency, this will result in -ve frequencies and is not allowed..."',
      footer: '<a href>Reduce your span frequency</a>',
    });
  }

  //Save first point, which must come from a black box
  console.log(schematic);
  var trace = [];
  var layout_shapes = [];
  var textbox_trace = [];
  var span_impedance_re = [];
  var span_impedance_im = [];
  var end_x_coord = 0;
  var end_y_coord = 0;
  let span_res;
  if (span_freq == 0) {
    span_res = 0;
  } else {
    span_res = span_resolution;
  }

  var real_old = 0.0;
  var imag_old = 0.0;
  var x;
  var y;
  var x0, x1, y0, y1;

  //update black box
  await update_schem_component(0, true, 1);
  var schemEl = document.getElementById('schematic');
  schemEl.innerHTML = '';
  var newDiv = draw_schematic(1);
  schemEl.appendChild(newDiv);
  // addEvents();

  //Create an array of all different arcs to draw. There will be 1 + 2 ^ (number of tolerances) arcs (every max and min combination, plus the ideal case)
  var originalSchematic = JSON.parse(JSON.stringify(schematic));
  var tolElements = []; //always 1 arc
  var numTolElements = 0;
  var i, j, x;
  for (i = 1; i < schematic.length; i++) if (schematic[i].tol > 0) numTolElements++;
  var arrLen = Math.pow(2, numTolElements);
  var tolJumper = 2;
  for (i = 1; i < schematic.length; i++) {
    tolElements[i] = Array(arrLen);
    tolElements[i].fill(1);
    if (schematic[i].tol > 0) {
      tolElements[i] = Array(arrLen);
      tolElements[i].fill(1);
      for (x = 0; x < tolJumper / 2; x++) {
        for (j = x; j < arrLen; j += tolJumper) {
          tolElements[i][j] = 1 + schematic[i].tol / 100;
        }
      }
      for (x = 0; x < tolJumper / 2; x++) {
        for (j = x + tolJumper / 2; j < arrLen; j += tolJumper) {
          tolElements[i][j] = 1 - schematic[i].tol / 100;
        }
      }
      tolJumper = tolJumper * 2;
    }
    if (arrLen > 1) tolElements[i].push(1); //this setting uses ideal components
  }
  // console.log(tolElements);

  var idealArc = false;
  for (let xx = 0; xx < tolElements[1].length; xx++) {
    if (xx == tolElements[1].length - 1) idealArc = true;
    if (idealArc) var arcColor = 'rgb(0, 0, 0)';
    else var arcColor = 'rgb(100, 100, 100)';

    //for each 'corner' set every component to min, max or ideal
    for (i = 1; i < schematic.length; i++) {
      for (j = 0; j < schematic[i].abs.length; j++) {
        // console.log("overwrite vals",tolElements[i][xx],originalSchematic[i].abs[j]);
        schematic[i].abs[j] = tolElements[i][xx] * originalSchematic[i].abs[j];
      }
    }

    dataPoints.length = 0;
    await update_schem_component(0, true, 1);
    for (i = 0; i <= span_res * 2; i++) {
      span_impedance_re[i] = Number(schematic[1].real);
      span_impedance_im[i] = Number(schematic[1].imaginary);
    }
    for (i = 2; i < schematic.length; i++) {
      //If tol is defined, loop this 3 times with min, typ and max value
      // Create some values to be fed into update_schem_component

      for (let sp = 0; sp <= 2 * span_res; sp++) {
        if (!idealArc || span_freq == 0) sp = span_res; //if trying different tolerances, don't compute all the spans
        //frequency at this point in the frequency span
        let frequency_at_sp;
        if (span_res == 0) frequency_at_sp = freq;
        else frequency_at_sp = freq + (span_freq * (sp - span_res)) / span_res;

        //calcuate re and im values of component at this frequency
        //if sp offset is at the original frequency, calculate a lot more points
        let num_points;
        if (sp == span_res) {
          num_points = resolution;
          var temp_array = await update_schem_component(frequency_at_sp, true, i);
        } else {
          num_points = 1;
          var temp_array = await update_schem_component(frequency_at_sp, false, i);
        }

        var re, im, ln_length;
        if (schematic[i].type == 'xfmr') {
          re = temp_array[0];
          im = temp_array[1];
          ln_length = temp_array[2];
        } else {
          re = Number(temp_array[0]);
          im = Number(temp_array[1]);
          ln_length = Number(temp_array[2]);
        }

        var temp_trace = {};
        var x_points, y_points;
        var start = [];
        var start_impedance = [];

        if (schematic[i].type == 'ss' || schematic[i].type == 'so') {
          //if the stub is longer than 0.5 waves then there is a full circle. Trim to 1 wave so user can see if there are multiple loops
          var wave_length = 3e8 / (frequency_at_sp * Math.sqrt(schematic[0].er));
          //if (ln_length>wave_length) ln_length = wave_length + ln_length % wave_length;
          //for "ss" matching, can't assume that we start at 0 length
          if (ln_length < 0.5 * wave_length) var start_at_qtr_wl = wave_length / 4;
          else start_at_qtr_wl = 0;
          start_impedance[0] = span_impedance_re[sp];
          start_impedance[1] = span_impedance_im[sp];
          start = one_over_complex(span_impedance_re[sp], span_impedance_im[sp]);
          await invoke('arc_smith_points', {
            x1: parseFloat(start[0]),
            y1: parseFloat(start[1]),
            x2: parseFloat(ln_length),
            y2: parseFloat(schematic[i].line_zo),
            type: schematic[i].type,
            rotate: true,
            beta: (2 * Math.PI * frequency_at_sp * Math.sqrt(schematic[0].er)) / 3e8,
            start_at_qtr_wl: parseFloat(start_at_qtr_wl),
            z0: schematic[0].z0,
            resolution: parseInt(resolution),
            verbose: false,
          })
            .then((result) => {
              temp_array = [
                result.x_coord,
                result.y_coord,
                result.end_x_coord,
                result.end_y_coord,
                result.real_old,
                result.imag_old,
                result.start_x_coord,
                result.start_y_coord,
                result.x1,
                result.y1,
                result.x2,
                result.y2,
              ];
              let schem_inv = one_over_complex(temp_array[4], temp_array[5]);
              span_impedance_re[sp] = schem_inv[0];
              span_impedance_im[sp] = schem_inv[1];
            })
            .catch((error) => {
              console.log('ERROR (smith_tool.js: arc_smith_points: transmission_line): ' + error);
            });
        } else if (schematic[i].type[0] == 'p' || schematic[i].type == 'rlc' || schematic[i].type == 'rc' || schematic[i].type == 'rl') {
          //For parallel elements plotted on rotated graph....
          start_impedance[0] = span_impedance_re[sp];
          start_impedance[1] = span_impedance_im[sp];
          start = one_over_complex(span_impedance_re[sp], span_impedance_im[sp]);
          var schem_inv = one_over_complex(re, im);
          await invoke('arc_smith_points', {
            x1: parseFloat(start[0]),
            y1: parseFloat(start[1]),
            x2: parseFloat(start[0] + schem_inv[0]),
            y2: parseFloat(start[1] + schem_inv[1]),
            type: schematic[i].type,
            rotate: true,
            beta: 0.0,
            start_at_qtr_wl: 0.0,
            z0: schematic[0].z0,
            resolution: parseInt(resolution),
            verbose: false,
          })
            .then((result) => {
              temp_array = [
                result.x_coord,
                result.y_coord,
                result.end_x_coord,
                result.end_y_coord,
                result.real_old,
                result.imag_old,
                result.start_x_coord,
                result.start_y_coord,
                result.x1,
                result.y1,
                result.x2,
                result.y2,
              ];
              schem_inv = one_over_complex(start[0] + schem_inv[0], start[1] + schem_inv[1]);
              span_impedance_re[sp] = schem_inv[0];
              span_impedance_im[sp] = schem_inv[1];
            })
            .catch((error) => {
              console.log('ERROR (smith_tool.js: arc_smith_points: parallel): ' + error);
            });
        } else if (schematic[i].type[0] == 's' || schematic[i].type[0] == 'b' || schematic[i].type == 'customZ') {
          //For series elements plotted on normal curves....
          start_impedance[0] = span_impedance_re[sp];
          start_impedance[1] = span_impedance_im[sp];
          await invoke('arc_smith_points', {
            x1: parseFloat(span_impedance_re[sp]),
            y1: parseFloat(span_impedance_im[sp]),
            x2: parseFloat(re + span_impedance_re[sp]),
            y2: parseFloat(im + span_impedance_im[sp]),
            type: 'impedance',
            rotate: false,
            beta: 0.0,
            start_at_qtr_wl: 0.0,
            z0: schematic[0].z0,
            resolution: parseInt(resolution),
            verbose: false,
          })
            .then((result) => {
              temp_array = [
                result.x_coord,
                result.y_coord,
                result.end_x_coord,
                result.end_y_coord,
                result.real_old,
                result.imag_old,
                result.start_x_coord,
                result.start_y_coord,
                result.x1,
                result.y1,
                result.x2,
                result.y2,
              ];
              span_impedance_re[sp] = span_impedance_re[sp] + re;
              span_impedance_im[sp] = span_impedance_im[sp] + im;
            })
            .catch((error) => {
              console.log('ERROR (smith_tool.js: arc_smith_points: series): ' + error);
            });
        } else if (schematic[i].type == 'tl') {
          //For transmission lines...
          start_impedance[0] = span_impedance_re[sp];
          start_impedance[1] = span_impedance_im[sp];
          await invoke('arc_smith_points', {
            x1: parseFloat(span_impedance_re[sp]),
            y1: parseFloat(span_impedance_im[sp]),
            x2: parseFloat(ln_length),
            y2: parseFloat(schematic[i].line_zo),
            type: 'transmission_line',
            rotate: false,
            beta: (2 * Math.PI * frequency_at_sp * Math.sqrt(schematic[0].er)) / 3e8,
            start_at_qtr_wl: 0.0,
            z0: schematic[0].z0,
            resolution: parseInt(resolution),
            verbose: false,
          })
            .then((result) => {
              temp_array = [
                result.x_coord,
                result.y_coord,
                result.end_x_coord,
                result.end_y_coord,
                result.real_old,
                result.imag_old,
                result.start_x_coord,
                result.start_y_coord,
                result.x1,
                result.y1,
                result.x2,
                result.y2,
              ];
              span_impedance_re[sp] = temp_array[4];
              span_impedance_im[sp] = temp_array[5];
            })
            .catch((error) => {
              console.log('ERROR (smith_tool.js: arc_smith_points: transmission_line): ' + error);
            });
        } else if (schematic[i].type == 'xfmr') {
          //For series elements plotted on normal curves....
          start_impedance[0] = span_impedance_re[sp];
          start_impedance[1] = span_impedance_im[sp];
          span_impedance_re[sp] = span_impedance_re[sp] + re[0];
          span_impedance_im[sp] = span_impedance_im[sp] + im[0];
          start = one_over_complex(span_impedance_re[sp], span_impedance_im[sp]);
          var schem_inv = one_over_complex(re[1], im[1]);
          var schem_inv = one_over_complex(start[0] + schem_inv[0], start[1] + schem_inv[1]);
          span_impedance_re[sp] = schem_inv[0];
          span_impedance_im[sp] = schem_inv[1];
          span_impedance_re[sp] = span_impedance_re[sp] + re[2];
          span_impedance_im[sp] = span_impedance_im[sp] + im[2];
          var start = one_over_complex(start_impedance[0], start_impedance[1]);
          var end = one_over_complex(span_impedance_re[sp], span_impedance_im[sp]);

          await invoke('arc_smith_points', {
            x1: parseFloat(start[0]),
            y1: parseFloat(start[1]),
            x2: parseFloat(end[0]),
            y2: parseFloat(end[1]),
            type: schematic[i].type,
            rotate: true,
            beta: 0.0,
            start_at_qtr_wl: 0.0,
            z0: schematic[0].z0,
            resolution: parseInt(resolution),
            verbose: false,
          })
            .then((result) => {
              temp_array = [
                result.x_coord,
                result.y_coord,
                result.end_x_coord,
                result.end_y_coord,
                result.real_old,
                result.imag_old,
                result.start_x_coord,
                result.start_y_coord,
                result.x1,
                result.y1,
                result.x2,
                result.y2,
              ];
              span_impedance_re[sp] = temp_array[4];
              span_impedance_im[sp] = temp_array[5];
            })
            .catch((error) => {
              console.log('ERROR (smith_tool.js: arc_smith_points: transmission_line): ' + error);
            });
        }

        //If at original frequency, save and plot the data points
        if (sp == span_res) {
          x_points = temp_array[0];
          y_points = temp_array[1];
          end_x_coord = temp_array[2];
          end_y_coord = temp_array[3];
          real_old = span_impedance_re[sp];
          imag_old = span_impedance_im[sp];
          var start_x_coord = temp_array[6];
          var start_y_coord = temp_array[7];
          temp_trace = {
            x: x_points,
            y: y_points,
            line: {
              color: arcColor,
              width: 4,
            },
            mode: 'lines',
            type: 'scatter',
          };
          trace.push(temp_trace);

          //add a data point rectangle to the smith chart
          dataPoints.push({ re: (z0 * Number(start_impedance[0])).toFixed(precision), im: (z0 * Number(start_impedance[1])).toFixed(precision) });
          if (show_labels_DP) {
            layout_shapes.push({
              type: 'circle',
              fillcolor: arcColor,
              line: { color: arcColor },
              x0: Number(start_x_coord) - 0.01,
              y0: Number(start_y_coord) - 0.01,
              x1: Number(start_x_coord) + 0.01,
              y1: Number(start_y_coord) + 0.01,
            });
            if (idealArc)
              textbox_trace.push({ x: [Number(start_x_coord) + 0.04], y: [Number(start_y_coord) - 0.03], text: ['DP' + (i - 1)], mode: 'text' });
          }
        }
        if (!idealArc || span_freq == 0) break; //if trying different tolerances, don't compute all the spans
      }
    }

    //If only the black box exists...
    if (schematic.length == 2) {
      temp_array = [];
      try {
        await invoke('find_smith_coord', {
          re: parseFloat(schematic[1].real),
          im: parseFloat(schematic[1].imaginary),
          rotate: false,
          verbose: false,
        })
          .then((result) => {
            end_x_coord = result[0];
            end_y_coord = result[1];
          })
          .catch((error) => {
            console.log('ERROR (smith_tool.js: black box points): ' + error);
          });
      } finally {
        real_old = schematic[1].real;
        imag_old = schematic[1].imaginary;
      }
    }

    //Create rectangles indicating end data points
    if (show_labels_DP) {
      layout_shapes.push({
        type: 'circle',
        fillcolor: arcColor,
        line: { color: arcColor },
        x0: Number(end_x_coord) - 0.01,
        y0: Number(end_y_coord) - 0.01,
        x1: Number(end_x_coord) + 0.01,
        y1: Number(end_y_coord) + 0.01,
      });
      if (idealArc) textbox_trace.push({ x: [Number(end_x_coord) + 0.04], y: [Number(end_y_coord) - 0.03], text: ['DP' + (i - 1)], mode: 'text' });
    }
  }

  //draw the components
  for (i = 2; i < schematic.length; i++) {
    newDiv = draw_schematic(i);
    schemEl.appendChild(newDiv);
  }

  dataPoints.push({ re: (z0 * Number(real_old)).toFixed(precision), im: (z0 * Number(imag_old)).toFixed(precision) });

  let reflection_mag, reflection_phase;
  [reflection_mag, reflection_phase] = calcOutputValues(real_old, imag_old, temp_array);

  //redefine the labels in case z0 has changed
  define_labels();

  //draw span curve
  var sp_coord_x = [],
    sp_coord_y = [];
  var refl_mag = [],
    refl_phase = [];
  var temp_refl_re, temp_refl_im, temp_refl_ph;
  for (i = 0; i < span_impedance_re.length; i++) {
    try {
      await invoke('find_smith_coord', {
        re: parseFloat(span_impedance_re[i]),
        im: parseFloat(span_impedance_im[i]),
        rotate: false,
        verbose: false,
      })
        .then((result) => {
          sp_coord_x.push(result[0]);
          sp_coord_y.push(result[1]);
        })
        .catch((error) => {
          console.log('ERROR (smith_tool.js, span points): ' + error);
        });
    } finally {
      temp_array = one_over_complex(span_impedance_re[i] * z0 + z0, span_impedance_im[i] * z0);
      let bot_real = temp_array[0];
      let bot_imag = temp_array[1];
      temp_refl_re = (span_impedance_re[i] * z0 - z0) * bot_real - span_impedance_im[i] * z0 * bot_imag;
      temp_refl_im = span_impedance_im[i] * z0 * bot_real + (span_impedance_re[i] * z0 - z0) * bot_imag;
      refl_mag.push(Number(Math.sqrt(temp_refl_re * temp_refl_re + temp_refl_im * temp_refl_im)));
      if (temp_refl_re == 0) var temp_refl_ph = 0;
      else var temp_refl_ph = (360 * Math.atan(temp_refl_im / temp_refl_re)) / (2 * Math.PI);
      if (temp_refl_re < 0) temp_refl_ph += 180;
      refl_phase.push(temp_refl_ph);
    }
  }
  let span_trace = {
    x: sp_coord_x,
    y: sp_coord_y,
    line: {
      color: 'rgb(200, 0, 0)',
      width: 4,
    },
    mode: 'lines',
    type: 'scatter',
  };
  if (span_impedance_re.length > 1) {
    let x_off, y_off;
    if (Number(sp_coord_y[0]) < Number(sp_coord_y[1])) y_off = 0.04;
    else y_off = -0.04;
    if (Number(sp_coord_x[0]) < Number(sp_coord_x[1])) x_off = 0.03;
    else x_off = -0.03;
    //draw a data box at each end of the span curve

    layout_shapes.push({
      type: 'rectangle',
      x0: Number(sp_coord_x[0]) - 0.01,
      y0: Number(sp_coord_y[0]) - 0.01,
      x1: Number(sp_coord_x[0]) + 0.01,
      y1: Number(sp_coord_y[0]) + 0.01,
    });
    textbox_trace.push({ x: [Number(sp_coord_x[0]) - x_off], y: [Number(sp_coord_y[0]) - y_off], text: ['F-span'], mode: 'text' });

    layout_shapes.push({
      type: 'rectangle',
      x0: Number(sp_coord_x[span_impedance_re.length - 1]) - 0.01,
      y0: Number(sp_coord_y[span_impedance_re.length - 1]) - 0.01,
      x1: Number(sp_coord_x[span_impedance_re.length - 1]) + 0.01,
      y1: Number(sp_coord_y[span_impedance_re.length - 1]) + 0.01,
    });
    textbox_trace.push({
      x: [Number(sp_coord_x[span_impedance_re.length - 1]) + x_off],
      y: [Number(sp_coord_y[span_impedance_re.length - 1]) + y_off],
      text: ['F+span'],
      mode: 'text',
    });
  }
  //console.log(span_impedance_re,span_impedance_im,span_trace)

  //Add custom markers so user can specify specific impedances which they could aim for
  for (i = 0; i < customMarkers.length; i++) {
    let sp_coord = [];
    try {
      await invoke('find_smith_coord', {
        re: parseFloat(customMarkers[i].re / z0),
        im: parseFloat(customMarkers[i].im / z0),
        rotate: false,
        verbose: false,
      })
        .then((result) => {
          x = result[0];
          y = result[1];
        })
        .catch((error) => {
          console.log('ERROR (smith_tool.js: custom markers): ' + error);
        });
    } finally {
      layout_shapes.push({ type: 'circle', line: { color: 'red' }, x0: x - 0.01, y0: y - 0.01, x1: x + 0.01, y1: y + 0.01 });
      // textbox_trace.push({ x: [x + 0.06], y: [y], text: ["MP" + i], mode: 'text' });
      textbox_trace.push({ x: [x + 0.06], y: [y], text: [customMarkers[i].name], mode: 'text' });
    }
  }

  //Add a VSWR circle, which is a new circle centered in the middle of the Smith Chart, with radius defined by VSWR
  if (vswr != 0.0) {
    //get coord of middle of smith chart (could search in the code but I'm lazy)
    let center_coord = [];
    let vswr_rad = [];
    try {
      await invoke('find_smith_coord', { re: parseFloat(1), im: parseFloat(0), rotate: false, verbose: false })
        .then((result) => {
          center_coord = result;
        })
        .catch((error) => {
          console.log('ERROR (smith_tool.js: vswr center): ' + error);
        });
      //get the radius of the VSWR
      await invoke('find_smith_coord', { re: parseFloat(vswr), im: parseFloat(0), rotate: false, verbose: false })
        .then((result) => {
          vswr_rad = result;
        })
        .catch((error) => {
          console.log('ERROR (smith_tool.js: vswr circle): ' + error);
        });
    } finally {
      x0 = 2 * Number(center_coord[0]) - Number(vswr_rad[0]);
      x1 = Number(vswr_rad[0]);
      y0 = Number(vswr_rad[0]);
      y1 = 2 * Number(center_coord[1]) - Number(vswr_rad[0]);
      if (color_of_smith_curves == 'colorful') var vswr_color = 'orangered';
      else var vswr_color = 'limegreen';
      layout_shapes.push({ type: 'circle', line: { color: vswr_color }, x0: x0, y0: y0, x1: x1, y1: y1 });
    }
  }
  if (constQ != 0.0) {
    //Create a 100-point line from Z=0 to Z=20*z0 with logarithmic steps
    var constQZArray = [0];
    var step = Math.log(20) / 200;
    var constQ_trace_x = [];
    var constQ_trace_y = [];
    // for (i = 1; i < 200; i++) {
    for (i = 1; i < 200; i++) {
      constQZArray.push(Math.E ** (i * step) - 1);
    }
    constQZArray.push(1e10); //~inf

    try {
      for (i = 0; i < constQZArray.length; i++) {
        await invoke('find_smith_coord', {
          re: parseFloat(constQZArray[i]),
          im: parseFloat(constQZArray[i] * constQ),
          rotate: false,
          verbose: false,
        })
          .then((result) => {
            constQ_trace_x.push(result[0]);
            constQ_trace_y.push(result[1]);
          })
          .catch((error) => {
            console.log('ERROR (smith_tool.js: constant Q top): ' + error);
          });
      }
      for (i = constQZArray.length - 1; i >= 0; i--) {
        await invoke('find_smith_coord', {
          re: parseFloat(constQZArray[i]),
          im: parseFloat(-constQZArray[i] * constQ),
          rotate: false,
          verbose: false,
        })
          .then((result) => {
            constQ_trace_x.push(result[0]);
            constQ_trace_y.push(result[1]);
          })
          .catch((error) => {
            console.log('ERROR (smith_tool.js: constant Q bottom): ' + error);
          });
      }
    } finally {
      var constQ_trace = {
        x: constQ_trace_x,
        y: constQ_trace_y,
        line: {
          color: 'mediumblue',
          width: 2,
        },
        mode: 'lines',
        type: 'scatter',
      };
    }
  } else var constQ_trace = {};

  var data = trace.concat(textbox_trace, trace_im_neg, trace_im_pos, trace_real, trace_adm, trace_sus_pos, trace_sus_neg, span_trace, constQ_trace);

  //
  //Create a plot for reflection coefficient plotted on its own
  //
  var exWidth = document.getElementById('myDiv').offsetWidth;
  // var exWidth = document.getElementById("myDiv").offsetWidth
  var PlLayout = {
    paper_bgcolor: 'rgba(255,255,255,0.2)',
    plot_bgcolor: 'rgba(255,255,255,0.0)',
    showlegend: false,
    margin: layout.margin,
    height: exWidth,
    width: exWidth,
    hovermode: layout.hovermode,
    xaxis: layout.xaxis,
    yaxis: layout.yaxis,
    shapes: layout.shapes.concat(layout_shapes),
  };
  var config = {
    displayModeBar: false, // this is the line that hides the hover bar.
  };
  Plotly.react('myDiv', data, PlLayout, config);

  //
  //Create a plots for distance to Vmax and Vmin
  //
  var markX, markY;
  try {
    await invoke('find_smith_coord', { re: parseFloat(real_old), im: parseFloat(imag_old), rotate: false, verbose: false })
      .then((result) => {
        markX = result[0];
        markY = result[1];
      })
      .catch((error) => {
        console.log('ERROR (smith_tool.js: vmax/vmin): ' + error);
      });
  } finally {
    //Create 2 arcs, one to Vmax and one to Vmin
    var arcRad = 1.1;
    var arcStartAng = (reflection_phase * Math.PI) / 180;
    var arcStartX = Math.cos(arcStartAng) * arcRad;
    var arcStartY = Math.sin(arcStartAng) * arcRad;
    var pathMax = 'M ' + arcStartX + ' ' + arcStartY;
    var arcAng;
    for (i = 100; i >= 0; i--) {
      arcAng = (arcStartAng * i) / 100;
      arcStartX = Math.cos(arcAng) * arcRad;
      arcStartY = Math.sin(arcAng) * arcRad;
      pathMax += ' L ' + arcStartX + ' ' + arcStartY;
    }
    pathMax += ' L ' + (arcRad + 0.05) + ' 0.05';
    pathMax += ' M ' + arcRad + ' 0';
    pathMax += ' L ' + (arcRad - 0.05) + ' 0.05';

    arcRad = 1.2;
    if (arcStartAng < Math.PI) arcStartAng = arcStartAng + 2 * Math.PI;
    arcStartX = Math.cos(arcStartAng) * arcRad;
    arcStartY = Math.sin(arcStartAng) * arcRad;
    var pathMin = 'M ' + arcStartX + ' ' + arcStartY;
    for (i = 0; i < 101; i++) {
      arcAng = arcStartAng - ((arcStartAng - Math.PI) * i) / 100;
      arcStartX = Math.cos(arcAng) * arcRad;
      arcStartY = Math.sin(arcAng) * arcRad;
      pathMin += ' L ' + arcStartX + ' ' + arcStartY;
    }
    pathMin += ' L ' + (-arcRad - 0.05) + ' -0.05';
    pathMin += ' M ' + -arcRad + ' 0';
    pathMin += ' L ' + (-arcRad + 0.05) + ' -0.05';

    var layout_lambda = {
      autosize: true,
      margin: {
        l: 20,
        r: 20,
        b: 20,
        t: 20,
      },
      hovermode: false,
      showlegend: false,
      paper_bgcolor: 'rgba(0,0,0,0)',
      plot_bgcolor: 'rgba(0,0,0,0)',
      xaxis: {
        range: [-1.3, 1.3],
        zeroline: false,
        showgrid: false,
        visible: false,
        fixedrange: true,
      },
      yaxis: {
        range: [-1.3, 1.3],
        zeroline: false,
        showgrid: false,
        visible: false,
        fixedrange: true,
      },
      shapes: [
        //draw the perimiter
        {
          type: 'circle',
          xref: 'x',
          yref: 'y',
          x0: -1,
          y0: -1,
          x1: 1,
          y1: 1,
          line: {
            color: 'black',
          },
        },
        //draw an arc
        {
          type: 'path',
          path: pathMax,
          line: {
            color: 'rgb(93, 164, 214)',
          },
        },
        {
          type: 'path',
          path: pathMin,
          line: {
            color: 'rgb(93, 164, 214)',
          },
        },
      ],
    };

    var data_lambda = [
      //show the data marker
      {
        x: [0],
        y: [0],
        mode: 'markers',
        marker: {
          size: 20,
        },
      },
      {
        x: [markX],
        y: [markY],
        mode: 'markers',
        marker: {
          size: 20,
          symbol: 'x',
          color: 'rgb(37, 50, 64)',
        },
      },
      //dashed line from 0,0, thru point, to rotation
      {
        x: [0, Math.cos(arcStartAng) * arcRad],
        y: [0, Math.sin(arcStartAng) * arcRad],
        line: {
          dash: 'dot',
          width: 1,
          color: 'black',
        },
        mode: 'lines',
        type: 'scatter',
      },
      //Vmin and Vmax labels
      {
        x: [0.9, -0.9],
        y: [0, 0],
        text: ['Vmax', 'Vmin'],
        mode: 'text',
        textfont: {
          size: fontsize,
        },
      },
    ];

    var smith_lambda = document.getElementById('smith_lambda').offsetWidth;
    layout_lambda.width = smith_lambda;
    layout_lambda.height = smith_lambda;
    Plotly.react('LambdaPlot', data_lambda, layout_lambda, config);

    //
    //Create a plots showing the S-parameters
    //
    var traceS11 = {
      line: {
        color: 'blue',
      },
      name: 'Magnitude',
      type: 'scatter',
    };

    var traceS11Ph = {
      line: {
        color: 'red',
      },
      name: 'Phase',
      yaxis: 'y2',
      type: 'scatter',
    };

    var sParamLayout = {
      yaxis: {
        tickfont: { color: 'blue' },
        zeroline: false,
        showgrid: true,
        gridcolor: 'rgb(37, 50, 64)',
        fixedrange: true,
        title: 'S11 (dB)',
        automargin: true,
      },
      yaxis2: {
        tickfont: { color: 'red' },
        side: 'right',
        zeroline: false,
        // showgrid: true,
        gridcolor: 'rgb(37, 50, 64)',
        fixedrange: true,
        title: 'Phase (deg)',
        automargin: true,
      },
      xaxis: {
        automargin: true,
        title: 'frequency (' + domFreqSel.value + ')',
        zeroline: false,
        showgrid: false,
        fixedrange: true,
      },
      autosize: true,
      margin: {
        l: 20,
        r: 20,
        b: 20,
        t: 20,
      },
      hovermode: false,
      showlegend: false,
      // legend: {
      //   x: 1,
      //   xanchor: 'right',
      //   y: 1
      // },
      paper_bgcolor: 'rgba(0,0,0,0)',
      plot_bgcolor: 'rgba(0,0,0,0)',
    };

    var scaledFreq = freq / schematic[0].freq_unit.multiplier;
    //just show 1 point
    traceS11.y = [];
    traceS11Ph.y = [];
    if (span_freq == 0) {
      var newSpanFreq = 1;
      traceS11.x = [scaledFreq];
      traceS11Ph.x = [scaledFreq];
      if (reflection_mag == 0) {
        traceS11.y.push(0);
        traceS11Ph.y.push(0);
      } else {
        traceS11.y.push(20 * Math.log10(reflection_mag));
        traceS11Ph.y.push(reflection_phase);
      }
      // traceS22.x = [scaledFreq];
      // traceS22.y = [0.5];
      // sParamLayout.yaxis.range = [0, 2];
      // sParamLayout.yaxis2.range = [0, 2];
    } else {
      // [reflectio_coeff_real, reflectio_coeff_imag, reflection_mag, reflection_phase] = impedanceToReflectionCoefficient (real_old, imag_old, z0)
      traceS11.x = [];
      traceS11Ph.x = [];
      for (i = 0; i < span_impedance_re.length; i++) {
        let reflectio_coeff_real, reflectio_coeff_imag, reflectio_mag, reflection_phase;
        [reflectio_coeff_real, reflectio_coeff_imag, reflection_mag, reflection_phase] = impedanceToReflectionCoefficient(
          span_impedance_re[i],
          span_impedance_im[i],
          z0
        );
        if (reflection_mag == 0) {
          traceS11.y.push(0);
          traceS11Ph.y.push(0);
        } else {
          traceS11.y.push(20 * Math.log10(reflection_mag));
          traceS11Ph.y.push(reflection_phase);
        }
        traceS11.x.push((freq + (span_freq * (i - span_res)) / span_res) / schematic[0].freq_unit.multiplier);
        traceS11Ph.x.push((freq + (span_freq * (i - span_res)) / span_res) / schematic[0].freq_unit.multiplier);
      }
      newSpanFreq = span_freq / schematic[0].freq_unit.multiplier;
    }

    sParamLayout.xaxis.range = [scaledFreq - newSpanFreq, scaledFreq + newSpanFreq];

    var data = [traceS11, traceS11Ph];

    Plotly.react('SParamPlot', data, sParamLayout, config);

    //update the HTML tables
    drawMakerTable();
  }
}

export function update_schem_tol(i, tol) {
  schematic[i].tol = Math.abs(tol.value);
  update_smith_chart();
}

export var trace_im_neg,
  trace_im_pos,
  trace_real,
  trace_adm,
  trace_sus_pos,
  trace_sus_neg = {};

export function define_labels() {
  trace_im_neg = {};
  trace_im_pos = {};
  trace_real = {};
  trace_adm = {};
  trace_sus_pos = {};
  trace_sus_neg = {};

  // console.log(color_of_smith_curves);
  let color_im, color_real, color_adm, color_sus;
  if (color_of_smith_curves == 'bland') {
    color_im = 'rgba(0, 0, 0,0.5)';
    color_real = 'rgba(0, 0, 0,0.5)';
    color_adm = 'rgba(0, 0, 0,0.3)';
    color_sus = 'rgba(0, 0, 0,0.3)';
  } else {
    color_im = 'rgba(252, 114, 2,0.5)';
    color_real = 'rgba(150, 0, 0,0.5)';
    color_adm = 'rgba(0, 10, 163,0.3)';
    color_sus = 'rgba(255, 0, 250,0.3)';
  }

  if (show_labels_res) {
    trace_im_pos = {
      x: [0.95, 0.9, 0.63, 0.05, -0.54, -0.86],
      y: [0.14, 0.33, 0.73, 0.95, 0.8, 0.4],
      text: [
        '<b>' + 10 * z0 + '</b>',
        '<b>' + 5 * z0 + '</b>',
        '<b>' + 2 * z0 + '</b>',
        '<b>' + 1 * z0 + '</b>',
        '<b>' + 0.5 * z0 + '</b>',
        '<b>' + 0.2 * z0 + '</b>',
      ],
      mode: 'text',
      textfont: {
        color: color_im,
        size: fontsize,
      },
    };

    trace_im_neg = {
      x: [0.95, 0.9, 0.63, 0.05, -0.54, -0.86],
      y: [-0.14, -0.33, -0.73, -0.95, -0.8, -0.4],
      text: [
        '<b>' + 10 * z0 + '</b>',
        '<b>' + 5 * z0 + '</b>',
        '<b>' + 2 * z0 + '</b>',
        '<b>' + 1 * z0 + '</b>',
        '<b>' + 0.5 * z0 + '</b>',
        '<b>' + 0.2 * z0 + '</b>',
      ],
      mode: 'text',
      textfont: {
        color: color_im,
        size: fontsize,
      },
    };
  }

  if (show_labels_res) {
    trace_real = {
      x: [0.96, 0.88, 0.66, 0.38, 0.05, -0.29, -0.62, -0.98],
      y: [0.03, 0.03, 0.03, 0.03, 0.03, 0.03, 0.03, 0.03, 0.03],
      text: [
        '<b>∞</b>',
        '<b>' + 10 * z0 + '</b>',
        '<b>' + 4 * z0 + '</b>',
        '<b>' + 2 * z0 + '</b>',
        '<b>' + 1 * z0 + '</b>',
        '<b>' + 0.5 * z0 + '</b>',
        '<b>' + 0.2 * z0 + '</b>',
        '<b>0</b>',
      ],
      mode: 'text',
      textfont: {
        color: color_real,
        size: fontsize,
      },
    };
  }
  if (show_labels_adm) {
    trace_adm = {
      x: [0.53, 0.26, -0.07, -0.4, -0.74, -0.88],
      y: [-0.03, -0.03, -0.03, -0.03, -0.03, -0.03, -0.03],
      text: [
        '<b>' + (1000 / 4 / z0).toFixed(precision) + '</b>m',
        '<b>' + (1000 / 2 / z0).toFixed(precision) + '</b>m',
        '<b>' + (1000 / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 2) / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 5) / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 10) / z0).toFixed(precision) + '</b>m',
      ],
      mode: 'text',
      textfont: {
        color: color_adm,
        size: fontsize,
      },
    };
  }

  if (show_labels_adm) {
    trace_sus_pos = {
      x: [0.86, 0.53, -0.07, -0.62, -0.89, -0.92],
      y: [0.4, 0.79, 0.97, 0.72, 0.31, 0.15],
      text: [
        '<b>' + (1000 / 5 / z0).toFixed(precision) + '</b>m',
        '<b>' + (1000 / 2 / z0).toFixed(precision) + '</b>m',
        '<b>' + (1000 / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 2) / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 5) / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 10) / z0).toFixed(precision) + '</b>m',
      ],
      mode: 'text',
      textfont: {
        color: color_sus,
        size: fontsize,
      },
    };

    trace_sus_neg = {
      x: [0.86, 0.53, -0.07, -0.62, -0.89, -0.92],
      y: [-0.4, -0.79, -0.97, -0.72, -0.31, -0.15],
      text: [
        '<b>' + (1000 / 5 / z0).toFixed(precision) + '</b>m',
        '<b>' + (1000 / 2 / z0).toFixed(precision) + '</b>m',
        '<b>' + (1000 / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 2) / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 5) / z0).toFixed(precision) + '</b>m',
        '<b>' + ((1000 * 10) / z0).toFixed(precision) + '</b>m',
      ],
      mode: 'text',
      textfont: {
        color: color_sus,
        size: fontsize,
      },
    };
  }
}

// //function intersectTwoCircles(x1,y1,r1, x2,y2,r2) {
// // Finds the intersection between two circles, one with magnitude 'real', the other with 'imaginary'
// export function find_smith_coord(real, imaginary, rotate, verbose = false) {
//   if (verbose) {
//     console.log('');
//     console.log('real = ' + real + ', imaginary = ' + imaginary + ', rotate = ' + rotate);
//   }
//   if (imaginary > 0) imaginary = Math.max(imaginary, 0.001);
//   else imaginary = Math.min(imaginary, -0.001);
//   real = Math.max(real, 0.001);

//   if (rotate == true) {
//     var realn = real / (real * real + imaginary * imaginary);
//     var imaginaryn = imaginary / (real * real + imaginary * imaginary);
//     real = realn;
//     imaginary = imaginaryn;
//   }

//   //to prevent weird anomolys in the plot
//   if (imaginary > 0) imaginary = Math.max(imaginary, 0.001);
//   else imaginary = Math.min(imaginary, -0.001);
//   real = Math.max(real, 0.001);

//   if (rotate == true) var x1 = -real / (real + 1);
//   else var x1 = real / (real + 1);
//   var y1 = 0;
//   var r1 = 1 / (real + 1);
//   if (rotate == true) var x2 = -1;
//   else var x2 = 1;
//   var y2 = 1 / imaginary;
//   var r2 = 1 / Math.abs(imaginary);

//   var centerdx = x1 - x2;
//   var centerdy = y1 - y2;
//   var R = Math.sqrt(centerdx * centerdx + centerdy * centerdy);
//   if (!(Math.abs(r1 - r2) <= R && R <= r1 + r2)) {
//     // no intersection
//     return []; // empty list of results
//   }
//   // intersection(s) should exist

//   var R2 = R * R;
//   var R4 = R2 * R2;
//   var a = (r1 * r1 - r2 * r2) / (2 * R2);
//   var r2r2 = r1 * r1 - r2 * r2;
//   var c = Math.sqrt((2 * (r1 * r1 + r2 * r2)) / R2 - (r2r2 * r2r2) / R4 - 1);

//   var fx = (x1 + x2) / 2 + a * (x2 - x1);
//   var gx = (c * (y2 - y1)) / 2;
//   var ix1 = (fx + gx).toFixed(5);
//   var ix2 = (fx - gx).toFixed(5);
//   // var ix1 = fx + gx;
//   // var ix2 = fx - gx;

//   var fy = (y1 + y2) / 2 + a * (y2 - y1);
//   var gy = (c * (x1 - x2)) / 2;
//   var iy1 = (fy + gy).toFixed(5);
//   var iy2 = (fy - gy).toFixed(5);
//   // var iy1 = fy + gy;
//   // var iy2 = fy - gy;

//   let ix, iy;
//   if (iy1 == 0) {
//     iy = iy2;
//     ix = ix2;
//   } else {
//     iy = iy1;
//     ix = ix1;
//   }

//   // note if gy == 0 and gx == 0 then the circles are tangent and there is only one solution
//   // but that one solution will just be duplicated as the code is currently written
//   if (rotate == true) {
//     (iy = -iy), (ix = -ix);
//   }

//   if (verbose) {
//     console.log('ix = ' + ix + ', iy = ' + iy);
//   }
//   return [ix, iy];
// }

//functions that are run at startup
update_smith_chart();
drawMakerTable();

window.update_schem_abs = update_schem_abs;
window.update_schem_tol = update_schem_tol;
window.update_smith_chart = update_smith_chart;
