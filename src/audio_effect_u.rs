#[cfg(test)]
mod tests {
	use crate::AudioBuffer;



	#[test]
	fn test_effect_combining() {
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![vec![1.0; 10]], 10);
		assert_eq!(buffer.mod_count(), 0);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 1);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 1);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 1);
		buffer.multiply_duration(0.5);
		assert_eq!(buffer.mod_count(), 2);
		buffer.multiply_volume(0.5);
		assert_eq!(buffer.mod_count(), 3);
		buffer.apply_effects();
		assert_eq!(buffer.mod_count(), 0);
	}
}