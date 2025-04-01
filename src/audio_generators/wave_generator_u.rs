#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, AudioGenerator, WaveGenerator };
	use std::time::Duration;



	fn test_wave_quality(generator_name:&str, generator_creator:fn(f32, u32) -> WaveGenerator) {
		const FULL_DURATION_MILLIS:u64 = 1000;
		const SAMPLE_RATE:u32 = 1000;

		for frequency in [1.0, 2.0, 4.0] {
			for batch_size_splitter in [1, 2, 4] {
				let batch_duration_ms:u64 = (FULL_DURATION_MILLIS as f32 / batch_size_splitter as f32) as u64;
				let batch_duration:Duration = Duration::from_millis(batch_duration_ms);

				let mut generator:WaveGenerator = generator_creator(frequency, SAMPLE_RATE);
				let wave_data:Vec<f32> = (0..batch_size_splitter).map(|_| generator.take(batch_duration)[0].to_vec()).flatten().collect();
				let file_path:String = format!("target/{generator_name}_{batch_size_splitter}x{batch_duration_ms}ms_{SAMPLE_RATE}sps_{frequency}hz.png");
				AudioBuffer::new(vec![wave_data], SAMPLE_RATE).write_png(&file_path).expect(&format!("Could not write png file {}", file_path));
			}
		}
	}

	#[test]
	fn test_wave_generator_sine_quality() {
		test_wave_quality("sine", WaveGenerator::sine);
	}
}