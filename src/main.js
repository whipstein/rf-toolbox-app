const { invoke } = window.__TAURI__.core;

let impCalcEl, matchCalcEl, conjCalcEl, smithChartEl;

window.addEventListener("DOMContentLoaded", () => {
    impCalcEl = document.getElementById("impCalc");
    matchCalcEl = document.getElementById("matchCalc");
    conjCalcEl = document.getElementById("conjCalc");
    smithChartEl = document.getElementById("smithChart");

    impCalcEl.addEventListener("click", (e) => {
        e.preventDefault();
        invoke("start_impedance_calculator");
    });

    matchCalcEl.addEventListener("click", (e) => {
        e.preventDefault();
        invoke("start_matching_calculator");
    });

    conjCalcEl.addEventListener("click", (e) => {
        e.preventDefault();
        invoke("start_conjugate_match_calculator");
    });

    smithChartEl.addEventListener("click", (e) => {
        e.preventDefault();
        invoke("start_smith_chart_tool");
    });
});
