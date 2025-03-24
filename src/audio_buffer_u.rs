#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, AudioGenerator };
	use std::time::Duration;



	/* CONSTRUCTOR TESTS */

	#[test]
	fn test_constructor_from_samples() {
		const RAW_SAMPLES:&[f32] = &[1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0];

		for channel_count in 0..9 {
			let sample_rate:u32 = (channel_count as u32) << channel_count;
			println!("Testing channel count {channel_count}, sample rate {sample_rate}");
			
			let buffer:AudioBuffer = AudioBuffer::new(vec![RAW_SAMPLES.to_vec(); channel_count], sample_rate);
			assert_eq!(buffer.channel_count(), channel_count);
			assert_eq!(buffer.sample_rate(), sample_rate);
			assert_eq!(buffer.raw_data(), &vec![RAW_SAMPLES; channel_count]);
		}
	}

	#[test]
	fn test_constructor_from_channels_bad_data() {
		let buffer:AudioBuffer = AudioBuffer::new(vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5], vec![0.0, 0.1, 0.2, 0.3]], 100);
		assert_eq!(buffer.raw_data(), &vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5], vec![0.0, 0.1, 0.2, 0.3, 0.0, 0.0]]);
	}



	/* DATA GETTER TESTS */

	#[test]
	fn test_take_all() {
		const RAW_SAMPLES:&[f32] = &[1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0];

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![RAW_SAMPLES.to_vec()], 10);
		assert_eq!(&buffer.raw_data()[0], RAW_SAMPLES);
		assert_eq!(&buffer.processed_data()[0], RAW_SAMPLES);
	}

	#[test]
	fn test_take_some() {
		const RAW_SAMPLES:&[f32] = &[1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0];

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![RAW_SAMPLES.to_vec()], 10);
		assert_eq!(&buffer.raw_data()[0], RAW_SAMPLES);
		assert_eq!(&buffer.take(Duration::from_millis(500))[0], &RAW_SAMPLES[..5]);
		assert_eq!(&buffer.raw_data()[0], RAW_SAMPLES);
		assert_eq!(&buffer.take(5)[0], &RAW_SAMPLES[5..10]);
	}

	#[test]
	fn test_take_drain() {
		const RAW_SAMPLES:&[f32] = &[1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0];

		let mut buffer:AudioBuffer = AudioBuffer::new(vec![RAW_SAMPLES.to_vec()], 10).drain_progression();
		assert_eq!(&buffer.raw_data()[0], RAW_SAMPLES);
		assert_eq!(&buffer.take(Duration::from_millis(500))[0], &RAW_SAMPLES[..5]);
		assert_eq!(&buffer.raw_data()[0], &RAW_SAMPLES[5..]);
		assert_eq!(&buffer.take(5)[0], &RAW_SAMPLES[5..10]);
	}
}