#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, AudioEffect, StereoShaper };



	/* CHANNEL COUNT */

	#[test]
	fn test_effect_stereo_shaper_count_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let saw_shape_stereo:Vec<f32> = saw_shape.iter().zip(&saw_shape).map(|(left, right)| vec![left, right]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape_stereo.clone(), 2, 10);
		buffer.resample_channel_count(1);
		assert_eq!(buffer.processed_data(), &saw_shape);
		assert_eq!(buffer.channel_count(), 1);
	}

	#[test]
	fn test_effect_stereo_shaper_count_up() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let saw_shape_stereo:Vec<f32> = saw_shape.iter().zip(&saw_shape).map(|(left, right)| vec![left, right]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.resample_channel_count(2);
		assert_eq!(buffer.processed_data(), &saw_shape_stereo);
		assert_eq!(buffer.channel_count(), 2);
	}

	#[test]
	fn test_effect_stereo_shaper_count_up_and_down() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		buffer.resample_channel_count(2);
		buffer.apply_effects();
		buffer.resample_channel_count(1);
		assert_eq!(buffer.processed_data(), &saw_shape);
		assert_eq!(buffer.channel_count(), 1);
	}



	/* STEREO FLIP */

	#[test]
	fn test_effect_stereo_shaper_stereo_flip_stereo() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let square_shape:Vec<f32> = (0..10).map(|index| (index / 5) as f32).collect();
		let stereo_shape:Vec<f32> = saw_shape.iter().zip(&square_shape).map(|(left, right)| vec![left, right]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		let flipped_stereo_shape:Vec<f32> = square_shape.iter().zip(&saw_shape).map(|(left, right)| vec![left, right]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(stereo_shape.clone(), 2, 10);
		buffer.flip_stereo(1.0);
		assert_eq!(buffer.processed_data(), &flipped_stereo_shape);
		assert_eq!(buffer.channel_count(), 2);
	}

	#[test]
	fn test_effect_stereo_shaper_stereo_partial_flip_stereo() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let square_shape:Vec<f32> = (0..10).map(|index| (index / 5) as f32).collect();
		let stereo_shape:Vec<f32> = saw_shape.iter().zip(&square_shape).map(|(left, right)| vec![left, right]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		let processed:Vec<f32> = square_shape.iter().zip(&saw_shape).map(|(left, right)| vec![left + (right - left) * 0.5, right + (left - right) * 0.5]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(stereo_shape.clone(), 2, 10);
		buffer.flip_stereo(0.5);
		assert_eq!(buffer.processed_data(), &processed);
		assert_eq!(buffer.channel_count(), 2);
	}



	/* SETTINGS */
	
	#[test]
	fn test_settings() {
		StereoShaper::new(1.0, 1.0, 1.0, 1.0).settings_test();
	}
}