use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct BitcrusherModule {
    bits: usize,
    phaser: f32,
    last: f32,
}

#[wasm_bindgen]
impl BitcrusherModule {
    pub fn new(bits: usize) -> BitcrusherModule {
        BitcrusherModule { bits, phaser: 0.0, last: 0.0 }
    }

    pub fn process(&mut self, samples: &[f32], normfreq: f32) -> Vec<f32> {
        
        let step = 0.5_f32.powf(self.bits as f32);

        let mut output = Vec::new();

        // Loop over samples and apply bitcrusher effect
        for sample in samples {
            self.phaser += normfreq;
            if self.phaser >= 1.0 {
                self.phaser -= 1.0;
                self.last = step * (sample / step + 0.5).floor(); 
            }
            output.push(self.last);
        }

        output
    }
}