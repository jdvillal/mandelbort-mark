#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Instant;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn calc_mandelbrot(a0: f64, b0: f64, af: f64, bf: f64, width: usize, height: usize, max_iterations: i32) -> Vec<u8>{
    //let width = 500;
    //let height = 500;
    let now = Instant::now();
    let a_step: f64 = (af - a0).abs() / width as f64;
    let b_step: f64 = (bf - b0).abs() / height as f64;
    println!("x_step {:?}", a_step);
    println!("y_step {:?}", b_step);
    //let max_iterations: i32 = 10000;

    let mut result: Vec<u8> = Vec::with_capacity(width*height*3);
    for i in 0..height{
        let py: f64 = b0 + (i as f64 * b_step);
        //println!("px =================>{:?}", px);
        for j in 0..width{
            
            let px: f64 = a0 + (j as f64 * a_step);
            //println!("py ==> {}", py);
            //println!("x,y = {},{}", i, j);
            let mut iteration: i32 = 0;

            let mut x = 0.0;
            let mut y = 0.0;
            
            let mut x2 = 0f64;
            let mut y2 = 0f64;

            while x2 + y2 <= 4f64 && iteration < max_iterations {
                y = 2f64 * x * y + py;
                x = x2 - y2 + px;
                x2 = x * x;
                y2 = y * y;
                iteration = iteration + 1;
            }
            let n = iteration as f64;
            let red: f64 = 0.5 * (0.1 * n).sin() + 0.5;
            let green = 0.5 * (0.1 * n + 2.094).sin() + 0.5;
            let blue = 0.5 * (0.1 * n + 4.188).sin() + 0.5;
            

            result.push((red * 255 as f64) as u8);
            result.push((green * 255 as f64) as u8);
            result.push((blue * 255 as f64) as u8);
            //result.push(255);
        }
    }
    println!("{}", now.elapsed().as_millis());
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calc_mandelbrot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
