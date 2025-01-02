import { clicked_cell, update_smith_chart } from './assets/js/smith_tool.js';
import {
  toggle_zoom_en,
  toggle_labels_DP,
  toggle_labels_imag,
  toggle_labels_real,
  toggle_circles_adm,
  toggle_circles_res,
} from './assets/js/draw.js';
import { updateFromDom } from './assets/js/util.js';
import { update_constQ, update_vswr } from './assets/js/defaults.js';
import { createCustomZModal, checkCustomZValid } from './assets/js/custom.js';

let modeSelEl, impSelEl, freqEl, freqSelEl, spanEl, spanSelEl, z0El, erEl;
let seriesCapEl, shuntCapEl, seriesIndEl, shuntIndEl, seriesResEl, shuntResEl, tlineEl, openStubEl, shortStubEl, xfmrEl, prlcEl, srlcEl, customZEl;
let zoomEl, showLabelsEl, toggleLabelsAdmittanceEl, toggleLabelsResistanceEl, toggleCirclesAdmEl, toggleCirclesResEl;
let vswrCircleEl, qCircleEl, toggleColorSchemeEl, toggleTraceIntensityEl;

window.addEventListener('DOMContentLoaded', () => {
  modeSelEl = document.getElementById('mode_sel');
  impSelEl = document.getElementById('imp_sel');
  spanEl = document.getElementById('span');
  spanSelEl = document.getElementById('span_sel');
  freqEl = document.getElementById('freq');
  freqSelEl = document.getElementById('freq_sel');
  z0El = document.getElementById('z0');
  erEl = document.getElementById('er');
  seriesCapEl = document.getElementById('series_cap');
  shuntCapEl = document.getElementById('shunt_cap');
  seriesIndEl = document.getElementById('series_ind');
  shuntIndEl = document.getElementById('shunt_ind');
  seriesResEl = document.getElementById('series_res');
  shuntResEl = document.getElementById('shunt_res');
  tlineEl = document.getElementById('tline');
  openStubEl = document.getElementById('open_stub');
  shortStubEl = document.getElementById('short_stub');
  xfmrEl = document.getElementById('xfmr');
  prlcEl = document.getElementById('prlc');
  srlcEl = document.getElementById('srlc');
  // customZEl = document.getElementById('custom_z');
  zoomEl = document.getElementById('inlineCheckbox4');
  showLabelsEl = document.getElementById('inlineCheckbox1');
  toggleLabelsAdmittanceEl = document.getElementById('inlineCheckbox2');
  toggleLabelsResistanceEl = document.getElementById('inlineCheckbox3');
  toggleCirclesAdmEl = document.getElementById('toggle_circles_adm');
  toggleCirclesResEl = document.getElementById('toggle_circles_res');
  vswrCircleEl = document.getElementById('vswr_circle');
  qCircleEl = document.getElementById('q_circle');
  toggleColorSchemeEl = document.getElementById('toggle_color_scheme');
  toggleTraceIntensityEl = document.getElementById('toggle_trace_intensity');

  modeSelEl.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });
  // impSelEl.addEventListener('change', (e) => {
  //   e.preventDefault();
  //   updateFromDom();
  // });
  freqEl.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });
  freqSelEl.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });
  spanEl.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });
  spanSelEl.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });
  z0El.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });
  erEl.addEventListener('change', (e) => {
    e.preventDefault();
    updateFromDom();
  });

  seriesCapEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('sc');
  });
  shuntCapEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('pc');
  });
  seriesIndEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('si');
  });
  shuntIndEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('pi');
  });
  seriesResEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('sr');
  });
  shuntResEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('pr');
  });
  tlineEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('tl');
  });
  openStubEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('so');
  });
  shortStubEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('ss');
  });
  xfmrEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('xfmr');
  });
  prlcEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('prlc');
  });
  srlcEl.addEventListener('click', (e) => {
    e.preventDefault();
    clicked_cell('srlc');
  });
  // customZEl.addEventListener('click', (e) => {
  //   e.preventDefault();
  //   clicked_cell('customZ');
  // });

  zoomEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_zoom_en();
  });
  showLabelsEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_labels_DP();
  });
  toggleLabelsAdmittanceEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_labels_imag();
  });
  toggleLabelsResistanceEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_labels_real();
  });
  toggleCirclesAdmEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_circles_adm();
  });
  toggleCirclesResEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_circles_res();
  });

  vswrCircleEl.addEventListener('change', (e) => {
    e.preventDefault();
    // vswr = Number(vswrCircleEl.value);
    // update_smith_chart();
    update_vswr(vswrCircleEl.value);
  });
  qCircleEl.addEventListener('change', (e) => {
    e.preventDefault();
    // constQ = Number(qCircleEl.value);
    // update_smith_chart();
    update_constQ(Number(qCircleEl.value));
  });
  toggleColorSchemeEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_color_scheme_fn();
  });

  toggleTraceIntensityEl.addEventListener('change', (e) => {
    e.preventDefault();
    toggle_trace_intensity_fn();
  });
});
