export var lastCustomModal = 0;

export function createCustomZModal(index) {
  var modalTitle = document.getElementById('customZModalTitle');
  var modalBody = document.getElementById('customZModalBody');
  modalTitle.innerHTML = 'Impedance Table for element #' + index;
  modalBody.value = schematic[index].raw;
  lastCustomModal = index;
  checkCustomZValid();
}

export const regexCustomZ = /[^0-9,eE\s\-\+\.]/; //list of acceptable characters
export const regexCustomZComma = /[,]/;
export var customZImpedanceTable = [];

export function checkCustomZValid() {
  var warn = document.getElementById('customZValidWarning');
  var textbox = document.getElementById('customZModalBody');
  var saveButton = document.getElementById('saveLUT');
  var regexRes = textbox.value.match(regexCustomZ);
  var regexResComma = textbox.value.match(regexCustomZComma);
  var customZPrevFreq = 0;
  var customZImpedanceTable = [];
  // if (regexResComma == null) var splitStr = ','
  // else var splitStr = ''
  var allLinesHave3Values = true;
  var allvaluesAreNotBlank = true;
  var frequencyIncreases = true;
  var lines = textbox.value.split(/\r?\n/);
  var splitLines;
  //ToDo - remove trailing blanklines
  for (var i = 0; i < lines.length; i++) {
    lines[i] = lines[i].trim();
    if (lines[i] == '') console.log('blank line found');
    else {
      if (regexResComma == null) splitLines = lines[i].split(/\s+/);
      else splitLines = lines[i].split(',');
      if (splitLines.length == 3) {
        if (splitLines[0] == '' || splitLines[1] == '' || splitLines[2] == '') allvaluesAreNotBlank = false;
        else {
          splitLines[0] = Number(splitLines[0]);
          splitLines[1] = Number(splitLines[1]);
          splitLines[2] = Number(splitLines[2]);
          if (i > 0 && splitLines[0] <= customZPrevFreq) frequencyIncreases = false;
          else {
            customZImpedanceTable.push(splitLines);
            customZPrevFreq = Number(splitLines[0]);
          }
        }
      } else allLinesHave3Values = false;
    }
  }
  if (regexRes == null && allLinesHave3Values && allvaluesAreNotBlank && frequencyIncreases) {
    textbox.classList.remove('is-invalid');
    textbox.classList.add('is-valid');
    saveButton.classList.remove('disabled');
    warn.style.display = 'none';
    schematic[lastCustomModal].lut = customZImpedanceTable;
    schematic[lastCustomModal].raw = textbox.value;
    if (document.getElementById('customz_interp_sah').checked) schematic[lastCustomModal].interp = 'sah';
    else schematic[lastCustomModal].interp = 'linear';
    plotCustomZ();
    // console.log('Pass',splitLines);
  } else {
    textbox.classList.remove('is-valid');
    textbox.classList.add('is-invalid');
    saveButton.classList.add('disabled');
    warn.style.display = 'block';
    // console.log('FAIL', regexRes,allLinesHave3Values, splitLines);
  }
}

export function removeCustom() {
  schematic.splice(lastCustomModal, 1);
  update_smith_chart();
}

export function plotCustomZ() {
  var x = [];
  var y = [];
  var mag;
  var temp;
  // console.log('plotting lut', schematic[lastCustomModal].lut, schematic[lastCustomModal].lut.length);
  for (var i = 0; i < schematic[lastCustomModal].lut.length; i++) {
    temp = schematic[lastCustomModal].lut[i];
    x.push(temp[0]);
    mag = Math.sqrt(temp[1] * temp[1] + temp[2] * temp[2]);
    y.push(mag);
  }
  var trace = {
    x: x,
    y: y,
    mode: 'lines+markers',
  };
  if (document.getElementById('customz_interp_sah').checked) trace.line = { shape: 'hv' };
  var data = [trace];
  var layout = {
    title: 'mag(Impedance) vs Frequency',
  };
  Plotly.react('plotlyCustomZplot', data, layout);
}

window.createCustomZModal = createCustomZModal;
window.checkCustomZValid = checkCustomZValid;
