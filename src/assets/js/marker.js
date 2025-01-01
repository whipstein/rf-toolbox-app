import { dataPoints, verbose } from './defaults.js';

export function addCustomMarker() {
  if (verbose >= 5) console.log('addCustomMarker(' + ')');
  var real = document.getElementById('customMarkerRe').value;
  var imaginary = document.getElementById('customMarkerIm').value;
  var name = document.getElementById('customMarkerName').value;

  if (real) {
    real = Number(real);
  } else real = 0;

  if (imaginary) {
    imaginary = Number(imaginary);
  } else imaginary = 0;

  if (name == '') {
    name = 'MP' + customMarkers.length;
  }

  if (schematic[0].imp == 'diff') {
    customMarkers.push({ re: real / 2, im: imaginary / 2, name: name });
  } else {
    customMarkers.push({ re: real, im: imaginary, name: name });
  }

  update_smith_chart();
}

export function delCustomMarker(i) {
  if (verbose >= 5) console.log('delCustomMarker(i: ' + i + ')');
  customMarkers.splice(i, 1);
  update_smith_chart();
}

//Draw a couple of tables in the HTML page
//#1 - Custom impedances that the user has added
//#2 - Impedance at each Data Point (DP)
export function drawMakerTable() {
  if (verbose >= 5) console.log('drawMakerTable(' + ')');
  var table = document.getElementById('customMarkerTable');
  var inner = "<table class='table table-striped table-sm'><tr><th>Real</th><th>Imaginary</th><th>Name</th><th></th></tr>";
  inner +=
    "<tr><td><input type='text' id='customMarkerRe' style='width:75px'></td><td><input type='text' id='customMarkerIm' style='width:75px'></td><td><input type='text' id='customMarkerName' style='width:75px'></td><td><button onclick=addCustomMarker()>add</button></td></tr>";
  var i = 0;
  for (i = 0; i < customMarkers.length; i++) {
    // inner += "<tr><td>" + customMarkers[i].re + "</td><td>" + customMarkers[i].im + "</td><td>MP" + i + "</td><td><button onClick='delCustomMarker(" + i + ")')>Remove</button></td></tr>"
    inner +=
      '<tr><td>' +
      customMarkers[i].re +
      '</td><td>' +
      customMarkers[i].im +
      '</td><td>' +
      customMarkers[i].name +
      "</td><td><button onClick='delCustomMarker(" +
      i +
      ")')>Remove</button></td></tr>";
  }
  table.innerHTML = inner + '</table>';

  //#2nd table
  table = document.getElementById('DPImpedance');
  inner = "<table class='table table-striped'><tr><th>Data Point (DP)</th><th>Real</th><th>Imaginary</th></tr>";
  for (i = 0; i < dataPoints.length; i++) {
    inner += '<tr><td>' + (i + 1) + '</td><td>' + dataPoints[i].re + '</td><td>' + dataPoints[i].im + '</td></tr>';
  }
  table.innerHTML = inner + '</table>';
}

//Add custom markers from the user, to help matching to exact impedances
export var customMarkers = [];

window.addCustomMarker = addCustomMarker;
window.delCustomMarker = delCustomMarker;
