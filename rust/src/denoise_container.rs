use jni::sys::jlong;
use nnnoiseless::DenoiseState;
use crate::util::pointer::JavaPointers;

pub const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;

pub fn frame_count(input_length: usize, output_length: Option<usize>) -> Result<usize, String> {
    if input_length == 0 || !input_length.is_multiple_of(FRAME_SIZE) {
        return Err(format!(
            "Input length must be a positive multiple of {FRAME_SIZE} samples (10 ms at 48 kHz), got {input_length}"
        ));
    }
    if let Some(output_length) = output_length.filter(|&length| length != input_length) {
        return Err(format!("Output length {output_length} doesn't match input length {input_length}"));
    }

    Ok(input_length / FRAME_SIZE)
}

pub struct DenoiseContainer {
    denoise: Box<DenoiseState<'static>>,
    input: Vec<f32>,
    output: Vec<f32>,
}

impl Default for DenoiseContainer {
    fn default() -> DenoiseContainer {
        DenoiseContainer {
            denoise: DenoiseState::new(),
            input: Vec::new(),
            output: Vec::new(),
        }
    }
}

impl DenoiseContainer {
    pub fn process<E>(
        &mut self,
        frames: usize,
        read_input: impl FnOnce(&mut [f32]) -> Result<(), E>
    ) -> Result<(f32, &[f32]), E> {
        let length = frames * FRAME_SIZE;
        self.input.resize(length, 0.0);
        self.output.resize(length, 0.0);
        read_input(&mut self.input)?;

        let probability = self.input.chunks_exact(FRAME_SIZE)
            .zip(self.output.chunks_exact_mut(FRAME_SIZE))
            .map(|(input, output)| self.denoise.process_frame(output, input))
            .fold(0.0, f32::max);

        Ok((probability, &self.output))
    }

    pub fn reset(&mut self) {
        self.denoise = DenoiseState::new();
    }
}

impl JavaPointers<DenoiseContainer> for DenoiseContainer {
    fn into_jlong_pointer(self) -> jlong {
        Box::into_raw(Box::new(self)) as jlong
    }
}
