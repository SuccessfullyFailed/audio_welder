use crate::{ AudioBufferDataLength, AudioFrequency, AudioGenerator };



pub struct WaveGenerator {
	frequency:f32,
	sample_rate:u32,
	shape_function:Box<dyn Fn(&mut f32, f32, u32, usize) -> Vec<f32>>,

	progress:f32
}
impl WaveGenerator {

	/// Create a new wave generator with custom function.
	pub fn new<T, R>(frequency:R, sample_rate:u32, shape_function:T) -> WaveGenerator where T:Fn(&mut f32, f32, u32, usize) -> Vec<f32> + 'static, R:AudioFrequency {
		WaveGenerator {
			frequency: frequency.to_hz(),
			sample_rate,
			shape_function: Box::new(shape_function),
			progress: 0.0
		}
	}

	/// Create a new sine-shape generator.
	pub fn sine<R>(frequency:R, sample_rate:u32) -> WaveGenerator where R:AudioFrequency {
		WaveGenerator::new(
			frequency.to_hz(),
			sample_rate,
			|progress, frequency, sample_rate, target_sample_count| {
				const MAX_PROGRESS:f32 = std::f32::consts::PI * 2.0;

				let progress_per_sample:f32 = MAX_PROGRESS / (sample_rate as f32 / frequency);
				let mut data:Vec<f32> = Vec::with_capacity(target_sample_count);
				for _ in 0..target_sample_count {
					data.push(progress.sin());
					*progress = (*progress + progress_per_sample) % MAX_PROGRESS;
				}
				data
			}
		)
	}

	/// Create a new saw-shape generator.
	pub fn saw<R>(frequency:R, sample_rate:u32) -> WaveGenerator where R:AudioFrequency {
		WaveGenerator::new(
			frequency.to_hz(),
			sample_rate,
			|progress, frequency, sample_rate, target_sample_count| {
				let progress_per_sample:f32 = 1.0 / (sample_rate as f32 / frequency); // 1.0 / samples per wave
				let mut data:Vec<f32> = Vec::with_capacity(target_sample_count);
				for _ in 0..target_sample_count {
					data.push(*progress * 2.0 - 1.0);
					*progress = (*progress + progress_per_sample) % 1.0;
				}
				data
			}
		)
	}

	/// Create a new square-shape generator.
	pub fn square<R>(frequency:R, sample_rate:u32) -> WaveGenerator where R:AudioFrequency {
		WaveGenerator::new(
			frequency.to_hz(),
			sample_rate,
			|progress, frequency, sample_rate, target_sample_count| {
				let samples_per_wave:f32 = sample_rate as f32 / frequency;
				let progress_per_sample:f32 = 1.0 / samples_per_wave;
				let mut value:f32 = if *progress < 0.5 { -1.0 } else { 1.0 };

				let mut data:Vec<f32> = Vec::with_capacity(target_sample_count);
				let mut remaining_samples = target_sample_count;
				while remaining_samples > 0 {
					let steps_to_flip:usize = ((0.5 - (*progress % 0.5)) / progress_per_sample).ceil() as usize;
					let batch:usize = steps_to_flip.min(remaining_samples);
					data.extend(vec![value; batch]);
					value *= -1.0;
					*progress = (*progress + (batch as f32 * progress_per_sample)) % 1.0;
					remaining_samples -= batch;
				}

				data
			}
		)
	}
}
impl AudioGenerator for WaveGenerator {
	
	/// Take a specific amount of data.
	fn take<T>(&mut self, duration:T) -> Vec<Vec<f32>> where T:AudioBufferDataLength {
		let sample_size:usize =  duration.as_buffer_length(self.sample_rate);
		vec![(self.shape_function)(&mut self.progress, self.frequency, self.sample_rate, sample_size)]
	}
}