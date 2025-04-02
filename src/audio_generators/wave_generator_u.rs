#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, AudioGenerator, WaveGenerator };
	use std::{ f32::consts::PI, fs::{create_dir_all, remove_dir_all}, time::Duration };



	fn test_wave_quality(generator_name:&str, generator_creator:fn(f32, u32) -> WaveGenerator, validation_function:fn(usize, f32, u32) -> f32) {
		const FULL_DURATION_MILLIS:u64 = 1000;
		const ROUNDING_SCALE:f32 = 1000.0;

		// Prepare images dir.
		let images_dir:String = "target/debug_images/".to_string() + generator_name;
		remove_dir_all(&images_dir).unwrap_or_default();
		create_dir_all(&images_dir).expect("Could not create debug images dir.");
		println!("Validating {generator_name}");

		// Loop through frequencies, batch sizes and sample rates.
		for frequency in [1.0, 2.0, 4.0] {
			for batch_size_splitter in [1, 2, 4] {
				let batch_duration_ms:u64 = (FULL_DURATION_MILLIS as f32 / batch_size_splitter as f32) as u64;
				let batch_duration:Duration = Duration::from_millis(batch_duration_ms);
				for sample_rate in [10, 25, 100] {

					// Prepare file paths.
					let file_path_no_extension:String = format!("{images_dir}/{batch_size_splitter}x{batch_duration_ms}ms_{sample_rate}sps_{frequency}hz");
					let wave_file_path:String = file_path_no_extension.clone() + ".png";
					let validation_file_path:String = file_path_no_extension + "_validation.png";
					println!("\t{}", wave_file_path);

					// Get wave data.
					let mut generator:WaveGenerator = generator_creator(frequency, sample_rate);
					let wave_data:Vec<f32> = (0..batch_size_splitter).map(|_| generator.take(batch_duration)[0].to_vec()).flatten().collect();
					let validation_wave_data:Vec<f32> = (0..wave_data.len()).map(|sample_index| validation_function(sample_index, frequency, sample_rate)).collect();

					// Create images.
					AudioBuffer::new(vec![wave_data.clone()], sample_rate).write_png(&wave_file_path).expect(&format!("Could not write png file {}", &wave_file_path));
					AudioBuffer::new(vec![validation_wave_data.clone()], sample_rate).write_png(&validation_file_path).expect(&format!("Could not write png file {}", &validation_file_path));

					// Validate similarity.
					assert_eq!(wave_data.len(), validation_wave_data.len());
					for (sample_index, (generated, validation)) in wave_data.iter().zip(&validation_wave_data).enumerate() {
						let (left, right) = ((generated * ROUNDING_SCALE).round() / ROUNDING_SCALE, (validation * ROUNDING_SCALE).round() / ROUNDING_SCALE);
						if left != right {
							println!("\t\tsample {}/{}", sample_index, wave_data.len());
							assert_eq!((generated * ROUNDING_SCALE).round() / ROUNDING_SCALE, (validation * ROUNDING_SCALE).round() / ROUNDING_SCALE);
						}
					}
				}
			}
		}
	}

	#[test]
	fn test_wave_generator_sine_quality() {
		test_wave_quality(
			"sine",
			WaveGenerator::sine,
			|sample_index, frequency, sample_rate| {
				let progress:f32 = (sample_index as f32 / (sample_rate as f32 / frequency)) % 1.0;
				(progress * PI * 2.0).sin()
			}
		);
	}

	#[test]
	fn test_wave_generator_saw_quality() {
		test_wave_quality(
			"saw",
			WaveGenerator::saw,
			|sample_index, frequency, sample_rate| {
				let progress:f32 = (sample_index as f32 / (sample_rate as f32 / frequency)) % 1.0;
				progress * 2.0 - 1.0
			}
		);
	}

	#[test]
	fn test_wave_generator_square_quality() {
		test_wave_quality(
			"square",
			WaveGenerator::square,
			|sample_index, frequency, sample_rate| {
				let progress:f32 = (sample_index as f32 / (sample_rate as f32 / frequency)) % 1.0;
				if progress < 0.5 { -1.0 } else { 1.0 }
			}
		);
	}

	#[test]
	fn test_wave_generator_custom_quality() {
		test_wave_quality(
			"custom",
			|frequency, sample_rate| WaveGenerator::new(frequency, sample_rate, |progress, frequency, sample_rate, target_sample_count| {
				let progress_per_sample:f32 = 1.0 / (sample_rate as f32 / frequency);
				let mut data:Vec<f32> = Vec::with_capacity(target_sample_count);
				for _ in 0..target_sample_count {
					data.push((*progress * PI).sin() * 2.0 - 1.0);
					*progress = (*progress + progress_per_sample) % 1.0;
				}
				data
			}),
			|sample_index, frequency, sample_rate| {
				let progress:f32 = (sample_index as f32 / (sample_rate as f32 / frequency)) % 1.0;
				(progress * PI).sin() * 2.0 - 1.0
			}
		);
	}
}