use pitch_detection::detector::{mcleod::McLeodDetector, PitchDetector};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PitchDetectorModule {
    sample_rate: usize,
    fft_size: usize,
    detector: McLeodDetector<f32>
}

#[wasm_bindgen]
impl PitchDetectorModule {
    pub fn new(sample_rate: usize, fft_size: usize) -> PitchDetectorModule {
        let padding = fft_size / 2;
        PitchDetectorModule { sample_rate, fft_size, detector: McLeodDetector::<f32>::new(fft_size, padding), }
    }

    pub fn get_pitch(&mut self, samples: &[f32]) -> f32 {
        if samples.len() < self.fft_size {
            panic!("Not enough samples!")
        }

        // Settings from pitch-detection example
        // Only include notes within a threshold of the amplitude of the frequency. Ranges from ??-??
        const POWER_THRESHOLD: f32 = 5.0;
        // Another threshold for the "clarity" of sound or how "sharp" a noise is
        const CLARITY_THRESHOLD: f32 = 0.7;

        
        let pitch_result = self.detector
            .get_pitch(&samples, self.sample_rate, POWER_THRESHOLD, CLARITY_THRESHOLD);

        match pitch_result {
            Some(pitch) => pitch.frequency,
            None => 0.0,
        }

    }
}
