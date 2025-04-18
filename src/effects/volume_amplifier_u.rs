#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, AudioEffect, VolumeAmplifier };



	#[test]
	fn test_effect_volume_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone()], 10);
		buffer.multiply_volume(2.0);
		buffer.multiply_volume(1.5);
		assert_eq!(&buffer.processed_data()[0], &saw_shape.iter().map(|sample| sample * 3.0).collect::<Vec<f32>>());
	}

	#[test]
	fn test_effect_volume_up() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone()], 10);
		buffer.multiply_volume(2.0);
		assert_eq!(&buffer.processed_data()[0], &saw_shape.iter().map(|sample| sample * 2.0).collect::<Vec<f32>>());
	}

	#[test]
	fn test_effect_volume_twice() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone()], 10);
		buffer.multiply_volume(2.0);
		buffer.multiply_volume(1.5);
		assert_eq!(&buffer.processed_data()[0], &saw_shape.iter().map(|sample| sample * 3.0).collect::<Vec<f32>>());
	}

	#[test]
	fn test_effect_volume_rounding_accuracy() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let multipliers:Vec<f32> = (0..12).map(|index| (1 << index) as f32).collect();

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone()], 10);
		for multiplier in &multipliers {
			buffer.multiply_volume(*multiplier);
			buffer.apply_effects();
		}
		for multiplier in &multipliers {
			buffer.multiply_volume(1.0 / *multiplier);
			buffer.apply_effects();
		}
		assert_eq!(&buffer.processed_data()[0], &saw_shape);
	}

	#[test]
	fn test_settings() {
		VolumeAmplifier::new(1.0).settings_test();
		VolumeAmplifier::new_maximizer().settings_test();
	}
}