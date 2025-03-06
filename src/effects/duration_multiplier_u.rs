#[cfg(test)]
mod tests {
	use crate::AudioBuffer;

	

	/* DURATION TESTS */

	#[test]
	fn test_effect_duration_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.multiply_duration(0.5);
		assert_eq!(buffer.processed_data(), &(0..5).map(|index| saw_shape[index * 2]).collect::<Vec<f32>>());
	}

	#[test]
	fn test_effect_duration_up() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.multiply_duration(2.0);
		assert_eq!(buffer.processed_data().len(), 20);
		for (left, right) in  buffer.processed_data().iter().zip(&(0..20).map(|index| index as f32 * 0.05).collect::<Vec<f32>>()) {
			println!("{left}, {right}");
			assert!((*left - *right) < 0.01);
		}
	}

	#[test]
	fn test_effect_duration_up_multiple() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.multiply_duration(2.0);
		buffer.multiply_duration(2.0);
		assert_eq!(buffer.processed_data().len(), 40);
		for (left, right) in  buffer.processed_data().iter().zip(&(0..40).map(|index| index as f32 * 0.025).collect::<Vec<f32>>()) {
			println!("{left}, {right}");
			assert!((*left - *right) < 0.01);
		}
	}



	/* SAMPLE RATE TESTS */

	#[test]
	fn test_effect_sample_rate_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.resample_sample_rate(5);
		assert_eq!(buffer.processed_data(), &(0..5).map(|index| saw_shape[index * 2]).collect::<Vec<f32>>());
		assert_eq!(buffer.sample_rate(), 5);
	}

	#[test]
	fn test_effect_sample_rate_up() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let stretched_saw_shape:Vec<f32> = (0..20).map(|index| index as f32 / 20.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.resample_sample_rate(20);
		for (left, right) in  buffer.processed_data().iter().zip(&stretched_saw_shape) {
			println!("{left}, {right}");
			assert!((*left - *right) < 0.01);
		}
		assert_eq!(buffer.sample_rate(), 20);
	}

	#[test]
	fn test_effect_sample_rate_list() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let test_sample_rates:Vec<u32> = (0..20).map(|index| 1 << index).collect();
		for in_sample_rate in &test_sample_rates {
			for out_sample_rate in &test_sample_rates {
				println!("{in_sample_rate} => {out_sample_rate}");
				let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 2, *in_sample_rate);
				buffer.resample_sample_rate(*out_sample_rate);
				buffer.apply_effects();
			}
		}
	}

	#[test]
	fn test_effect_sample_rate_rounding_accuracy() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let sample_rates:Vec<u32> = vec![20, 30, 50, 80];

		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		for multiplier in &sample_rates {
			buffer.resample_sample_rate(*multiplier);
			buffer.apply_effects();
		}
		for sample_rate in sample_rates.iter().rev() {
			buffer.resample_sample_rate(*sample_rate);
			buffer.apply_effects();
		}
		for (left, right) in  buffer.processed_data().iter().zip(&saw_shape) {
			println!("{left}, {right}");
			assert!((*left - *right) < 0.01);
		}
	}
}