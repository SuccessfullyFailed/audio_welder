#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, AudioEffect, StereoShaper };



	/* CHANNEL COUNT */

	#[test]
	fn test_effect_stereo_shaper_count_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone(), vec![0.0; saw_shape.len()]], 10);
		buffer.resample_channel_count(1);
		assert_eq!(buffer.processed_data(), &vec![saw_shape]);
		assert_eq!(buffer.channel_count(), 1);
	}

	#[test]
	fn test_effect_stereo_shaper_count_up() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone()], 10);
		buffer.resample_channel_count(2);
		assert_eq!(buffer.processed_data(), &vec![saw_shape; 2]);
		assert_eq!(buffer.channel_count(), 2);
	}

	#[test]
	fn test_effect_stereo_shaper_count_up_and_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone()], 10);
		buffer.resample_channel_count(2);
		buffer.apply_effects();
		buffer.resample_channel_count(1);
		assert_eq!(&buffer.processed_data()[0], &saw_shape);
		assert_eq!(buffer.channel_count(), 1);
	}



	/* STEREO FLIP */

	#[test]
	fn test_effect_stereo_shaper_stereo_flip_stereo() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let square_shape:Vec<f32> = (0..10).map(|index| (index / 5) as f32).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone(), square_shape.clone()], 10);
		buffer.flip_stereo(1.0);
		assert_eq!(buffer.processed_data(), &vec![square_shape, saw_shape]);
		assert_eq!(buffer.channel_count(), 2);
	}

	#[test]
	fn test_effect_stereo_shaper_stereo_partial_flip_stereo() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let square_shape:Vec<f32> = (0..10).map(|index| (index / 5) as f32).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![saw_shape.clone(), square_shape.clone()], 10);
		buffer.flip_stereo(0.5);
		assert_eq!(&buffer.processed_data()[0], &square_shape.iter().zip(&saw_shape).map(|(left, right)| left + (right - left) * 0.5).collect::<Vec<f32>>());
		assert_eq!(buffer.channel_count(), 2);
	}



	/* SETTINGS */
	
	#[test]
	fn test_settings() {
		StereoShaper::new(1.0, 1.0, 1.0, 1.0).settings_test();
	}
}