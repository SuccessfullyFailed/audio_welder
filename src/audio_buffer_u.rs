#[cfg(test)]
mod tests {
	use crate::AudioBuffer;



	/* CONSTRUCTOR TESTS */

	#[test]
	fn test_constructor_from_samples() {
		const RAW_SAMPLES:&[f32] = &[1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0];

		for channel_count in 0..9 {
			let sample_rate:u32 = (channel_count as u32) << channel_count;
			println!("Testing channel count {channel_count}, sample rate {sample_rate}");
			
			let samples:Vec<f32> = RAW_SAMPLES.iter().map(|sample| vec![*sample; channel_count]).flatten().collect();
			let buffer:AudioBuffer = AudioBuffer::from_samples(samples.clone(), channel_count, sample_rate);
			assert_eq!(buffer.channel_count(), channel_count);
			assert_eq!(buffer.sample_rate(), sample_rate);
			assert_eq!(buffer.raw_data(), &samples);
			assert_eq!(buffer.raw_channels_data(), vec![RAW_SAMPLES; channel_count]);
		}
	}

	#[test]
	fn test_constructor_from_channels() {
		const RAW_SAMPLES:&[f32] = &[1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0];

		for channel_count in 0..9 {
			let sample_rate:u32 = (channel_count as u32) << channel_count;
			println!("Testing channel count {channel_count}, sample rate {sample_rate}");
			
			let samples:Vec<f32> = RAW_SAMPLES.iter().map(|sample| vec![*sample; channel_count]).flatten().collect();
			let buffer:AudioBuffer = AudioBuffer::from_channels(vec![RAW_SAMPLES.to_vec(); channel_count], sample_rate);
			assert_eq!(buffer.channel_count(), channel_count);
			assert_eq!(buffer.sample_rate(), sample_rate);
			assert_eq!(buffer.raw_data(), &samples);
			assert_eq!(buffer.raw_channels_data(), vec![RAW_SAMPLES; channel_count]);
		}
	}

	#[test]
	fn test_constructor_from_channels_bad_data() {
		let buffer:AudioBuffer = AudioBuffer::from_channels(vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5], vec![0.0, 0.1, 0.2, 0.3]], 100);
		assert_eq!(buffer.raw_channels_data(), vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5], vec![0.0, 0.1, 0.2, 0.3, 0.0, 0.0]]);
	}



	/* EFFECT SCHEDULING TESTS */

	#[test]
	fn test_effect_scheduling() {
		let mut buffer:AudioBuffer = AudioBuffer::from_samples(vec![1.0; 10], 1, 10);
		assert_eq!(buffer.mod_count(), 0);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 1);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 1);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 1);
		buffer.multiply_duration(0.5);
		assert_eq!(buffer.mod_count(), 2);
		buffer.multiply_duration(0.5);
		assert_eq!(buffer.mod_count(), 2);
		buffer.apply_effects();
		assert_eq!(buffer.mod_count(), 0);
	}

	#[test]
	fn test_effect_volume() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut one_vol_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		one_vol_mod_buffer.multiply_volume(0.5);
		assert_eq!(one_vol_mod_buffer.processed_data(), &saw_shape.iter().map(|sample| sample * 0.5).collect::<Vec<f32>>());
		
		let mut two_vol_mods_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		two_vol_mods_buffer.multiply_volume(2.0);
		two_vol_mods_buffer.multiply_volume(1.5);
		assert_eq!(two_vol_mods_buffer.processed_data(), &saw_shape.iter().map(|sample| sample * 3.0).collect::<Vec<f32>>());
	}

	#[test]
	fn test_effect_duration() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut one_dur_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		one_dur_mod_buffer.multiply_duration(0.5);
		assert_eq!(one_dur_mod_buffer.processed_data(), &(0..5).map(|index| saw_shape[index * 2]).collect::<Vec<f32>>());
		
		let mut two_dur_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		two_dur_mod_buffer.multiply_duration(2.0);
		two_dur_mod_buffer.multiply_duration(2.0);
		assert_eq!(two_dur_mod_buffer.processed_data().len(), 40);
		for (left, right) in  two_dur_mod_buffer.processed_data().iter().zip(&(0..40).map(|index| index as f32 * 0.025).collect::<Vec<f32>>()) {
			println!("{left}, {right}");
			assert!((*left - *right) < 0.01);
		}
	}

	#[test]
	fn test_effect_sample_rate() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		
		let mut sample_rate_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		sample_rate_mod_buffer.resample_sample_rate(5);
		assert_eq!(sample_rate_mod_buffer.processed_data(), &(0..5).map(|index| saw_shape[index * 2]).collect::<Vec<f32>>());
		assert_eq!(sample_rate_mod_buffer.sample_rate(), 5);

		let test_sample_rates:Vec<u32> = (0..20).map(|index| 1 << index).collect();
		for in_sample_rate in &test_sample_rates {
			for out_sample_rate in &test_sample_rates {
				println!("{in_sample_rate} => {out_sample_rate}");
				let mut sample_rate_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 2, *in_sample_rate);
				sample_rate_mod_buffer.resample_sample_rate(*out_sample_rate);
				sample_rate_mod_buffer.apply_effects();
			}
		}
	}

	#[test]
	fn test_effect_channel_count() {
		let saw_shape:Vec<f32> = (0..10).map(|index| index as f32 / 10.0).collect();
		let saw_shape_stereo:Vec<f32> = saw_shape.iter().zip(&saw_shape).map(|(left, right)| vec![left, right]).flatten().map(|value| value.to_owned()).collect::<Vec<f32>>();
		
		let mut channel_count_up_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape.clone(), 1, 10);
		channel_count_up_mod_buffer.resample_channel_count(2);
		assert_eq!(channel_count_up_mod_buffer.processed_data(), &saw_shape_stereo);
		assert_eq!(channel_count_up_mod_buffer.channel_count(), 2);
		
		let mut channel_count_down_mod_buffer:AudioBuffer = AudioBuffer::from_samples(saw_shape_stereo, 2, 10);
		channel_count_down_mod_buffer.resample_channel_count(1);
		assert_eq!(channel_count_down_mod_buffer.processed_data(), &saw_shape);
		assert_eq!(channel_count_down_mod_buffer.channel_count(), 1);
	}
}