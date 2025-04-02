#[cfg(test)]
mod tests {
	use crate::{ AudioBufferDataLength, AudioFrequency };
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



	/* FREQUENCY TESTS */

	#[test]
	fn test_frequency_from_number() {
		for source in [1.0, 2.0, 5.0, 25.0, 100.0, 1000.0] {
			assert_eq!(source.to_hz(), source);
			assert_eq!((source as usize).to_hz(), source);
		}
	}

	#[test]
	fn test_frequency_from_note() {
		for (note, frequency) in [("A4", 440.0), ("A#4", 466.0), ("B4", 494.0), ("C4", 523.0), ("C#4", 554.0), ("D4", 587.0), ("D#4", 622.0), ("E4", 659.0), ("F4", 698.0), ("F#4", 740.0), ("A6", 1397.0)] {
			println!("{}\t{}", note, note.to_hz().round());
			assert_eq!(note.to_hz().round(), frequency);
		}
		assert_eq!("FAKE_NOTE".to_hz().round(), 440.0);
	}
}