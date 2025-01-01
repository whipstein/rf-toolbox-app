import { color_of_smith_curves, schematic, precision, verbose, colors } from './defaults.js';
import { update_smith_chart } from './smith_tool.js';

export var show_labels_DP = true;
export var show_labels_adm = true;
export var show_labels_res = true;
export var show_circles_adm = true;
export var show_circles_res = true;

export function toggle_zoom_en() {
  if (verbose >= 5) console.log('toggle_zoom_en(' + ')');
  var element = document.getElementById('smithChartOverlay');
  element.classList.toggle('noPointerClass');
}

export function toggle_labels_DP() {
  if (verbose >= 5) console.log('toggle_labels_DP(' + ')');
  show_labels_DP = !show_labels_DP;
  update_smith_chart();
}

export function toggle_labels_imag() {
  if (verbose >= 5) console.log('toggle_labels_imag(' + ')');
  show_labels_adm = !show_labels_adm;
  update_smith_chart();
}

export function toggle_labels_real() {
  if (verbose >= 5) console.log('toggle_labels_real(' + ')');
  show_labels_res = !show_labels_res;
  update_smith_chart();
}

export function toggle_circles_adm() {
  if (verbose >= 5) console.log('toggle_circles_adm(' + ')');
  show_circles_adm = !show_circles_adm;
  update_smith_chart();
}

export function toggle_circles_res() {
  if (verbose >= 5) console.log('toggle_circles_res(' + ')');
  show_circles_res = !show_circles_res;
  update_smith_chart();
}

export function draw_schematic(i) {
  if (verbose >= 5) console.log('draw_schematic(i: ' + i + ')');
  //Add the element to the schematic view
  var div = document.createElement('div');
  let unit = [];
  div.setAttribute('class', 'col-6 col-lg-2 g-0');
  //Add a close button, but can't remove black boxes...
  var innerText = '';
  // if (schematic[i].type!='bb') div.innerHTML += "<div class=\"rem\" onclick=\"schematic.splice("+i+",1); update_smith_chart()\"><div class=\"dp_txt\">DP"+i+"</div><div class=\"close-button\"></div></div>";
  // else div.innerHTML += "<div class=\"rem\">DP"+i+"</div>";
  if (schematic[i].type != 'bb')
    innerText +=
      '<div class="row me-2 ms-2" style="height: 26px;"><div class="col"><small>DP' +
      i +
      '</small></div><div class="col text-end"><button type="button" class="btn-close" onclick="schematic.splice(' +
      i +
      ',1); update_smith_chart()"></button></div></div>';
  else innerText += '<div class="row me-2 ms-2" style="height: 26px;"><small>DP' + i + '</small></div>';
  var rows_to_create = [];
  let sch_label, sch_imag, sch_real, sch_abs, sch_icon, sch_svg;
  switch (schematic[i].type) {
    case 'bb':
      sch_label = 'Black Box';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      sch_icon = 'black_box';
      sch_svg = 0;
      rows_to_create = [['Impedance'], ['abs', 'abs'], ['tol']];
      break;
    case 'customZ':
      sch_label = 'Custom';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      sch_icon = 'CustomZ';
      sch_svg = 6500;
      rows_to_create = [['blank-impedance'], ['custom']];
      break;
    case 'pr':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['tol']];
      sch_label = 'Parallel Resistor';
      sch_imag = false;
      sch_real = true;
      sch_abs = true;
      unit = [['mΩ', 'Ω', 'KΩ', 'MΩ']];
      sch_icon = 'resistor_parallel';
      sch_svg = 2500;
      break;
    case 'sr':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['tol']];
      sch_label = 'Series Resistor';
      sch_imag = false;
      sch_real = true;
      sch_abs = true;
      unit = [['mΩ', 'Ω', 'KΩ', 'MΩ']];
      sch_icon = 'resistor_series';
      sch_svg = 3000;
      break;
    case 'pc':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['abs', 'unit_1'], ['tol']];
      sch_label = 'Parallel Capacitor';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      unit = [
        ['Q', 'mΩ', 'Ω'],
        ['mF', 'uF', 'nF', 'pF', 'fF'],
      ];
      sch_icon = 'capacitor_parallel';
      sch_svg = 500;
      break;
    case 'sc':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['abs', 'unit_1'], ['tol']];
      sch_label = 'Series Capacitor';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      unit = [
        ['Q', 'mΩ', 'Ω'],
        ['mF', 'uF', 'nF', 'pF', 'fF'],
      ];
      sch_icon = 'capacitor_series';
      sch_svg = 1000;
      break;
    case 'pi':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['abs', 'unit_1'], ['tol']];
      sch_label = 'Parallel Inductor';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      unit = [
        ['Q', 'mΩ', 'Ω'],
        ['H', 'mH', 'uH', 'nH', 'pH'],
      ];
      sch_icon = 'inductor_parallel';
      sch_svg = 1500;
      break;
    case 'si':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['abs', 'unit_1'], ['tol']];
      sch_label = 'Series Inductor';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      unit = [
        ['Q', 'mΩ', 'Ω'],
        ['H', 'mH', 'uH', 'nH', 'pH'],
      ];
      sch_icon = 'inductor_series';
      sch_svg = 2000;
      break;
    case 'tl':
      rows_to_create = [['blank-impedance'], ['abs', 'unit_0'], ['line_zo']];
      sch_label = 'Transmission Line';
      sch_imag = false;
      sch_real = false;
      sch_abs = true; //is actually length
      unit = [[' m', 'mm', 'um', 'λ']];
      sch_icon = 'transmission_line';
      sch_svg = 3500;
      break;
    case 'ss':
      rows_to_create = [['blank-impedance'], ['abs', 'unit_0'], ['line_zo']];
      sch_label = 'Short Stub';
      sch_imag = false;
      sch_real = false;
      sch_abs = true; //is actually length
      unit = [[' m', 'mm', 'um', 'λ']];
      sch_icon = 'stub_short';
      sch_svg = 4500;
      break;
    case 'so':
      rows_to_create = [['blank-impedance'], ['abs', 'unit_0'], ['line_zo']];
      sch_label = 'Open Stub';
      sch_imag = false;
      sch_real = false;
      sch_abs = true; //is actually length
      unit = [[' m', 'mm', 'um', 'λ']];
      sch_icon = 'stub_open';
      sch_svg = 4000;
      break;
    case 'xfmr':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['abs', 'unit_1'], ['abs', 'unit_2'], ['abs', 'unit_3'], ['tol']];
      sch_label = 'Transformer';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      unit = [
        ['Q', 'mΩ', 'Ω'],
        ['H', 'mH', 'uH', 'nH', 'pH'],
        ['H', 'mH', 'uH', 'nH', 'pH', 'N'],
        ['K', 'H', 'mH', 'uH', 'nH', 'pH'],
      ];
      sch_icon = 'inductor_parallel';
      sch_svg = 6500;
      break;
    case 'rlc':
      rows_to_create = [['Impedance'], ['abs', 'unit_0'], ['abs', 'unit_1'], ['abs', 'unit_2'], ['tol']];
      sch_label = 'Inductor w/ ESR';
      sch_imag = true;
      sch_real = true;
      sch_abs = true;
      unit = [
        ['mΩ', 'Ω', 'KΩ', 'MΩ'],
        ['H', 'mH', 'uH', 'nH', 'pH'],
        ['mF', 'uF', 'nF', 'pF', 'fF'],
      ];
      sch_icon = 'black_box';
      sch_svg = 6000;
      break;
  }
  // add svg image of element
  if (schematic[i].type == 'customZ' || schematic[i].type == 'bb' || schematic[i].type == 'tl') {
    innerText +=
      '<div class="row"><div class="col"><svg viewBox="' +
      sch_svg +
      ' 0 500 500"><use xlink:href="assets/svg/elements_w_custom.svg#rainbow3" alt="' +
      sch_label +
      '" /></svg></div></div>';
  } else {
    innerText +=
      '<div class="row"><div class="col"><svg viewBox="' +
      sch_svg +
      ' 0 500 500"><use xlink:href="assets/svg/elements_update.svg#rainbow3" alt="' +
      sch_label +
      '" /></svg></div></div>';
  }

  var cntR, cntC, ittUnit, boxType, varSelect, unitIndex;
  var absCounter = 0;
  const z0 = schematic[0].z0;
  for (cntR = 0; cntR < rows_to_create.length; cntR++) {
    innerText += '<div class="row ms-3 me-3"><div class="input-group mb-1 p-0">';
    for (cntC = 0; cntC < rows_to_create[cntR].length; cntC++) {
      boxType = rows_to_create[cntR][cntC];
      if (boxType == 'tol') {
        innerText += '<span class="input-group-text">tol &plusmn; </span>';
        innerText +=
          '<input type="text" class="form-control" id="sch_' +
          i +
          '_tol" value="' +
          schematic[i].tol +
          '" name="tol" onchange="update_schem_tol(' +
          i +
          ',this)">';
        // innerText += '<input type="text" class="form-control" id="sch_' + i + '_tol" value="' + schematic[i].tol + '" name="tol">'
        innerText += '<span class="input-group-text">%</span>';
        // onchangeEl.push({
        //     el: "sch_" + i + "_tol",
        //     f: "update_schem_tol",
        //     args: [i , schematic[i].tol],
        // });
      } else if (boxType == 'blank-impedance') {
        innerText += '<div class="fst-italic m-auto">&nbsp</div>';
      } else if (boxType == 'Impedance') {
        innerText += '<div class="fst-italic m-auto">Z = ';
        if (sch_real) innerText += Number((schematic[i].real * z0).toFixed(precision));
        if (sch_real && sch_imag) {
          if (schematic[i].imaginary * z0 >= 0) innerText += ' + ';
          else innerText += ' - ';
        }
        if (sch_imag) innerText += Number(Math.abs(schematic[i].imaginary * z0).toFixed(precision)) + 'j';
        innerText += '</div>';
      } else if (boxType == 'custom') {
        innerText +=
          '<button type="button" class="btn btn-secondary m-auto" data-bs-toggle="modal" id="sch_' +
          i +
          '_btn" data-bs-target="#customZModal" onclick="createCustomZModal(' +
          i +
          ')">Impedance Table</button>';
        // innerText += '<button type="button" class="btn btn-secondary m-auto" data-bs-toggle="modal" id="sch_' + i + '_btn" data-bs-target="#customZModal">Impedance Table</button>';
        // clickEl.push = "sch_" + i + "_btn";
        // onclickEl.push({
        //     el: "sch_" + i + "_btn",
        //     f: "createCustomZModal",
        //     args: [i],
        // });
      } else if (boxType == 'line_zo') {
        innerText += '<span class="input-group-text">Z₀ = </span>';
        innerText +=
          '<input type="text" class="form-control" id="sch_' +
          i +
          '_val" value=' +
          schematic[i][boxType] +
          ' name="' +
          boxType +
          '" onchange="update_schem_abs(' +
          i +
          ',this,0)">';
        // innerText += '<input type="text" class="form-control" id="sch_' + i + '_val" value=' + schematic[i][boxType] + ' name="' + boxType + '" onchange="console.log(this)">'
        // innerText += '<input type="text" class="form-control" id="sch_' + i + '_val" value=' + schematic[i][boxType] + ' name="' + boxType + '">'
        // onchangeEl.push({
        //     el: "sch_" + i + "_val",
        //     f: "update_schem_abs",
        //     args: [i , schematic[i][boxType], 0],
        // });
      } else if (boxType == 'unit_0' || boxType == 'unit_1' || boxType == 'unit_2' || boxType == 'unit_3') {
        unitIndex = boxType.split('_')[1];
        innerText +=
          '<select class="form-select" id="sch_' + i + '_' + unitIndex + '_span" onchange="updatespan(' + i + ', this, ' + unitIndex + ')">';
        // innerText += '<select class="form-select" id="sch_' + i + '_' + unitIndex + '_span">'
        // onchangeEl.push({
        //     el: "sch_" + i + '_' + unitIndex + "_span",
        //     f: "updatespan",
        //     args: [i , "this", unitIndex],
        // });
        for (ittUnit = 0; ittUnit < unit[unitIndex].length; ittUnit++) {
          if (unit[unitIndex][ittUnit] == schematic[i].unit[unitIndex]) varSelect = 'selected';
          else varSelect = '';
          innerText += '<option value=' + unit[unitIndex][ittUnit] + ' ' + varSelect + '>' + unit[unitIndex][ittUnit] + '</option>';
        }
        innerText += '</select>';
        // console.log('Unit', schematic[i].unit[unitIndex], innerText);
      } else {
        if (cntC > 0) innerText += '<span class="input-group-text">+</span>';
        innerText +=
          '<input type="text" class="form-control inputMW" id="sch_' +
          i +
          '_val" value=' +
          schematic[i][boxType][absCounter] +
          ' name="' +
          boxType +
          '" onchange="update_schem_abs(' +
          i +
          ',this,' +
          absCounter +
          ')">';
        // innerText += '<input type="text" class="form-control inputMW" id="sch_' + i + '_val" value=' + schematic[i][boxType][absCounter] + ' name="' + boxType + '" onchange="console.log(this.name)">'
        // innerText += '<input type="text" class="form-control inputMW" id="sch_' + i + '_' + absCounter + '_val" value=' + schematic[i][boxType][absCounter] + ' name="' + boxType + '">'
        // onchangeEl.push({
        //     el: "sch_" + i + "_" + absCounter + "_val",
        //     f: "update_schem_abs",
        //     args: [i , schematic[i][boxType][absCounter], absCounter],
        // });
        // innerText += '<input type="text" class="form-control inputMW" value=' + schematic[i][boxType][absCounter] + ' name="' + boxType + '" onkeyup="update_schem_abs(' + i + ',this,' + absCounter + ')">'
        if (cntC > 0) innerText += '<span class="input-group-text ps-2 pe-2">j</span>';
        if (boxType == 'abs') absCounter = absCounter + 1;
      }
    }
    innerText += '</div></div>';
  }

  div.innerHTML = innerText;

  return div;
}

export var layout = {
  title: 'Circles',
  hovermode: false,
  xaxis: {
    range: [-1, 1],
    zeroline: false,
    showgrid: false,
  },
  yaxis: {
    range: [-1, 1],
    showgrid: false,
  },
  width: 650,
  height: 650,
  showgrid: false,
  margin: {
    l: 0,
    r: 0,
    b: 0,
    t: 0,
  },
};

export function configure_layout_shapes() {
  if (verbose >= 5) console.log('configure_layout_shapes(' + ')');
  // let color_resistance_real, color_resistance_imaginary, color_admittance_real, color_admittance_imaginary;

  // if (color_of_smith_curves == 'bland') {
  //   color_resistance_real = 'rgba(255, 0, 0, 0.2)';
  //   color_resistance_imaginary = 'rgba(255, 0, 0, 0.3)';
  //   color_admittance_real = 'rgba(0, 0, 255, 0.2)';
  //   color_admittance_imaginary = 'rgba(0, 0, 255, 0.3)';
  // } else {
  //   color_resistance_real = 'rgba(150, 0, 0, 0.2)';
  //   color_resistance_imaginary = 'rgba(252, 114, 2, 0.3)';
  //   color_admittance_real = 'rgba(255, 0, 250, 0.2)';
  //   color_admittance_imaginary = 'rgba(0, 10, 163, 0.3)';
  // }

  var shapes_omni = [
    {
      type: 'circle',
      x0: -1,
      y0: -1,
      x1: 1,
      y1: 1,
      line: {
        color: colors.resistance_real,
      },
    },
  ];

  var shapes_res = [
    ///RESISTANCE CIRCLES
    {
      type: 'circle',
      x0: -0.666,
      y0: -0.833,
      x1: 1,
      y1: 0.833,
      line: {
        color: colors.resistance_real,
      },
    },
    {
      type: 'circle',
      x0: -0.333,
      y0: -0.666,
      x1: 1,
      y1: 0.666,
      line: {
        color: colors.resistance_real,
      },
    },
    {
      type: 'circle',
      x0: 0,
      y0: -0.5,
      x1: 1,
      y1: 0.5,
      line: {
        color: colors.resistance_real,
      },
    },
    {
      type: 'circle',
      x0: 0.333,
      y0: -0.333,
      x1: 1,
      y1: 0.333,
      line: {
        color: colors.resistance_real,
      },
    },
    {
      type: 'circle',
      x0: 0.6,
      y0: -0.2,
      x1: 1,
      y1: 0.2,
      line: {
        color: colors.resistance_real,
      },
    },
    {
      type: 'circle',
      x0: 0.818,
      y0: -0.0909,
      x1: 1,
      y1: 0.0909,
      line: {
        color: colors.resistance_real,
      },
    },
  ];

  ///ADMITTANCE CIRCLES
  var shapes_adm = [
    {
      type: 'circle',
      x0: 0.6,
      y0: -0.8,
      x1: -1,
      y1: 0.8,
      line: {
        color: colors.admittance_real,
      },
    },
    {
      type: 'circle',
      x0: 0.333,
      y0: -0.666,
      x1: -1,
      y1: 0.666,
      line: {
        color: colors.admittance_real,
      },
    },
    {
      type: 'circle',
      x0: -1,
      y0: -0.5,
      x1: 0,
      y1: 0.5,
      line: {
        color: colors.admittance_real,
      },
    },
    {
      type: 'circle',
      x0: -1,
      y0: -0.333,
      x1: -0.333,
      y1: 0.333,
      line: {
        color: colors.admittance_real,
      },
    },
    {
      type: 'circle',
      x0: -1,
      y0: -0.166,
      x1: -0.666,
      y1: 0.166,
      line: {
        color: colors.admittance_real,
      },
    },
    {
      type: 'circle',
      x0: -1,
      y0: -0.0909,
      x1: -0.818,
      y1: 0.0909,
      line: {
        color: colors.admittance_real,
      },
    },
  ];

  ///REACTANCE CIRCLES
  var shapes_rea = [
    {
      type: 'circle',
      x0: 0.9,
      y0: 0,
      x1: 1.1,
      y1: 0.2,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: 0.8,
      y0: 0,
      x1: 1.2,
      y1: 0.4,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: 0.5,
      y0: 0,
      x1: 1.5,
      y1: 1,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: 0,
      y0: 0,
      x1: 2,
      y1: 2,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -1,
      y0: 0,
      x1: 3,
      y1: 4,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -4,
      y0: 0,
      x1: 6,
      y1: 10,
      line: {
        color: colors.resistance_imaginary,
      },
    },

    //imaginary
    {
      type: 'circle',
      x0: 0.9,
      y0: 0,
      x1: 1.1,
      y1: -0.2,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: 0.8,
      y0: 0,
      x1: 1.2,
      y1: -0.4,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: 0.5,
      y0: 0,
      x1: 1.5,
      y1: -1,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: 0,
      y0: 0,
      x1: 2,
      y1: -2,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -1,
      y0: 0,
      x1: 3,
      y1: -4,
      line: {
        color: colors.resistance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -4,
      y0: 0,
      x1: 6,
      y1: -10,
      line: {
        color: colors.resistance_imaginary,
      },
    },
  ];

  ///SUSCEPTANCE CIRCLES
  var shapes_sus = [
    {
      type: 'circle',
      x0: -1.1,
      y0: 0,
      x1: -0.9,
      y1: 0.2,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -1.2,
      y0: 0,
      x1: -0.8,
      y1: 0.4,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -1.5,
      y0: 0,
      x1: -0.5,
      y1: 1,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -2,
      y0: 0,
      x1: -0,
      y1: 2,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -3,
      y0: 0,
      x1: 1,
      y1: 4,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -6,
      y0: 0,
      x1: 4,
      y1: 10,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    //negative
    {
      type: 'circle',
      x0: -1.1,
      y0: 0,
      x1: -0.9,
      y1: -0.2,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -1.2,
      y0: 0,
      x1: -0.8,
      y1: -0.4,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -1.5,
      y0: 0,
      x1: -0.5,
      y1: -1,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -2,
      y0: 0,
      x1: -0,
      y1: -2,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -3,
      y0: 0,
      x1: 1,
      y1: -4,
      line: {
        color: colors.admittance_imaginary,
      },
    },
    {
      type: 'circle',
      x0: -6,
      y0: 0,
      x1: 4,
      y1: -10,
      line: {
        color: colors.admittance_imaginary,
      },
    },
  ];
  if (!show_circles_adm) shapes_adm = [];
  if (!show_circles_adm) shapes_sus = [];
  if (!show_circles_res) shapes_res = [];
  if (!show_circles_res) shapes_rea = [];

  var shapes = [].concat(shapes_res, shapes_sus, shapes_rea, shapes_adm, shapes_omni);
  return shapes;
}

export function resizedw() {
  if (verbose >= 5) console.log('resizedw(' + ')');
  update_smith_chart();
}
