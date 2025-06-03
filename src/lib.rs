use std::f32::consts::PI;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(greeting: f32) -> f32 {
    // alert(&format!("Hello {}!", greeting));
    
    let sample = (greeting * 440.0 * 2.0 * PI).sin();

    sample
}