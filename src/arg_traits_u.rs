#[cfg(test)]
mod tests {
	use crate::AudioBufferDataLength;
	use std::time::Duration;



	/* BUFFER LENGTH TESTS */

	#[test]
	fn test_buffer_length_from_usize() {
		for sample_count in [0, 5, 10, 25, 100, 25000] {
			for sample_rate in [0, 5, 10, 25, 100, 25000] {
				assert_eq!(sample_count.as_buffer_length(sample_rate), sample_count);
			}
		}
	}

	#[test]
	fn test_buffer_length_from_duration() {
		for duration_millis in [0, 5, 10, 25, 100, 25000] {
			for sample_rate in [0, 5, 10, 25, 100, 25000] {
				assert_eq!(Duration::from_millis(duration_millis).as_buffer_length(sample_rate), (duration_millis as f32 * 0.001 * sample_rate as f32) as usize);
			}
		}
	}
}