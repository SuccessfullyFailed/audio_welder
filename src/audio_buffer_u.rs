#[cfg(test)]
mod tests {
	use crate::AudioBuffer;



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
}