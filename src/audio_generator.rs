use crate::AudioBufferDataLength;



pub trait AudioGenerator {

	/// Take a specific amount of data.
	fn take<T>(&mut self, duration:T) -> Vec<Vec<f32>> where T:AudioBufferDataLength;

	/// Take a specific amount of flattened data.
	fn take_flat<T>(&mut self, duration:T) -> Vec<f32> where T:AudioBufferDataLength {

		// Get non-flat data.
		let input_data:Vec<Vec<f32>> = self.take(duration);
		if input_data.is_empty() || input_data[0].is_empty() {
			return Vec::new();
		}
		assert!(input_data.iter().all(|channel_data| channel_data.len() == input_data[0].len()));

		// Flatten.
		let mut output_data:Vec<f32> = Vec::with_capacity(input_data.len() * input_data[0].len());
		for sample_index in 0..input_data[0].len() {
			for channel_index in 0..input_data.len() {
				output_data.push(input_data[channel_index][sample_index]);
			}
		}

		// Return flat data.
		output_data
	}
}