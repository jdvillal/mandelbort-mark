const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function calc_maldelbrot(){
  let result = await invoke("calc_mandelbrot",{a0: -2, b0: 2, af: 2, bf: -2, max_iterations: 100});
  console.log(result);
  //calc_mandelbrot(a0: f64, b0: f64, af: f64, bf: f64, a_dim: i32, b_dim: i32, max_iterations: i32)
}

window.addEventListener("DOMContentLoaded", () => {

  let fractal_canvas = document.getElementById("fractal-canvas");

  document.getElementById('calc-btn').addEventListener('click', async ()=>{
    let result = await invoke("calc_mandelbrot",{a0: -2, b0: 2, af: 2, bf: -2, aDim: 800, bDim: 800, maxIterations: 100});
    console.log(result);
  });


  fractal_canvas.addEventListener('click', event =>
  {
      let bound = fractal_canvas.getBoundingClientRect();

      let x = event.clientX - bound.left - fractal_canvas.clientLeft;
      let y = event.clientY - bound.top - fractal_canvas.clientTop;
      console.log(x,y);
      document.getElementById('position-tag').innerText = x + ',' + y;


  });



});


