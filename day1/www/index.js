import * as wasm from "day1";
//const wasm = import("../pkg")

const p1in = document.getElementById("p1in");

const solvep1Button = document.getElementById("solvep1");
solvep1Button.addEventListener("click", event => {
    console.log("solving");
    wasm.solvep1(p1in.value);
  });
//const p1in = document.getElementById("p1in");

//const p1solution = document.getElementById("p1solution");
//p1solution.textContent = wasm.p1solution();