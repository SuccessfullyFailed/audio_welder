#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, NoiseGate };



	#[test]
	fn test_effect_noise_gate_constant_audio() {
		let square_shape:Vec<f32> = (0..100).map(|index| if index / 5 % 2 == 0 { -1.0 } else { 1.0 }).collect();

		// Create buffer and apply effects.
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![square_shape], 10);
		buffer.add_effect(NoiseGate::new(0.2, 0.05));
		buffer.apply_effects();
		
		// Validate data.
		let flat_abs_data:Vec<f32> = buffer.raw_data()[0].iter().map(|sample| sample.abs()).collect();
		for (index, actual) in flat_abs_data.iter().enumerate() {
			if index < 20 {
				assert!((*actual - (index + 1) as f32 / 20.0).abs() < 0.01);
			} else {
				assert!((*actual - 1.0).abs() < 0.01);
			}
		}
	}

	#[test]
	fn test_effect_noise_gate_interrupted_constant_audio() {
		let square_shape:Vec<f32> = (0..100).map(|index| if index >= 30 && index < 45 { 0.0 } else { 1.0 }).collect();

		// Create buffer and apply effects.
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![square_shape], 10);
		buffer.add_effect(NoiseGate::new(0.2, 0.05));
		buffer.apply_effects();
		
		// Validate data.
		let flat_abs_data:Vec<f32> = buffer.raw_data()[0].iter().map(|sample| sample.abs()).collect();
		for (index, actual) in flat_abs_data.iter().enumerate() {
			if index < 20 {
				assert!((*actual - (index + 1) as f32 / 20.0).abs() < 0.01); // Initial peak.
			} else if index < 30 {
				assert!((*actual - 1.0).abs() < 0.01); // Top after initial peak.
			} else if index < 45 {
				assert!((*actual - 0.0).abs() < 0.01); // Silence.
			} else if index < 60 {
				assert!((*actual - ((index - 45 + 1) as f32 / 20.0 + 0.25)).abs() < 0.01); // Secondary peak, starting from 0.25
			} else {
				assert!((*actual - 1.0).abs() < 0.01); // Top after secondary peak.
			}
		}
	}

	#[test]
	fn test_effect_noise_gate_click() {
		let square_shape:Vec<f32> = (0..100).map(|index| if index == 30 { -1.0 } else if index == 31 { 0.6 } else { 0.0 }).collect();

		// Create buffer and apply effects.
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![square_shape], 10);
		buffer.add_effect(NoiseGate::new(0.2, 0.05));
		buffer.apply_effects();
		
		// Validate data.
		assert!(buffer.raw_data().iter().flatten().all(|sample| sample.abs() < 0.1));
	}
}