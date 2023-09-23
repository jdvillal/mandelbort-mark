const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

let c_width = 800;
let c_height = 800;


async function draw_fractal2(){
  var x = Math.floor(Math.random() * canvasWidth);
  var y = Math.floor(Math.random() * canvasHeight);
  var r = Math.floor(Math.random() * 256);
  var g = Math.floor(Math.random() * 256);
  var b = Math.floor(Math.random() * 256);
  var off = (y * id.width + x) * 4;
  pixels[off] = r;
  pixels[off + 1] = g;
  pixels[off + 2] = b;
  pixels[off + 3] = 255;
  console.log("flag");
  ctx.putImageData(id, 0, 0);
}
function draw_fractal(result, ctx, imageData){
  let off = 0;
  let lenght = c_width*c_height*3;
  for(let i = 0; i < lenght; i+=3){
    imageData.data[i+off] = result[i];
    imageData.data[i+off + 1] = result[i+1];
    imageData.data[i+off + 2] = result[i+2];
    imageData.data[i+off + 3] = 255;
    off = off + 1;
  }
  console.log(imageData)
  ctx.putImageData(imageData, 0, 0);
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  
  let fractal_canvas = document.getElementById("fractal-canvas");

  let canvas = document.getElementById('fractal-canvas');
  let ctx = canvas.getContext('2d');
  let canvasWidth = canvas.width;
  let canvasHeight = canvas.height;
  console.log(canvasWidth, canvasHeight);
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);
  //let imageData = ctx.getImageData(0, 0, canvasWidth, canvasHeight);
  let imageData = ctx.createImageData(c_width,c_height);
  document.getElementById('calc-btn').addEventListener('click', async ()=>{
    let result = await invoke("calc_mandelbrot",{a0: -2.5, b0: -2, af:1.5, bf: 2, width: c_width, height: c_height, maxIterations: 500});
    console.log(result.lenght, result);
    draw_fractal(result, ctx, imageData);
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


