#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn calc_mandelbrot(a0: f64, b0: f64, af: f64, bf: f64) -> Vec<f64>{
    let a_step: f64 = (af - a0) / 800.0;
    let b_step: f64 = (bf - b0) / 800.0;
    let max_iterations: i32 = 100;

    let mut result: Vec<f64> = Vec::new();
    for i in 0..800{
        let ai: f64 = a0 + i as f64 * a_step;
        for j in 0..800{
            let bj: f64 = b0 + j as f64 * b_step;

            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;

            let mut iteration: i32 = 0;
            
            let mut a2: f64 = 0.0;
            let mut b2: f64 = 0.0;

            while a2+b2 <= 4.0 && iteration < max_iterations {
                b = 2.0 * a * b + bj;
                a = a2 - b2 + ai;
                a2 = a * a;
                b2 = b * b;
                iteration = iteration + 1;
            }
            let n = iteration as f64;
            let red: f64 = 0.5 * (a*n).sin() + 0.5;
            let green = 0.5 * (a * n + 2.094).sin() + 0.5;
            let blue = 0.5 * (a * n + 4.188).sin() + 0.5;
            result.push(red);
            result.push(green);
            result.push(blue);
        }
    }
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calc_mandelbrot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
