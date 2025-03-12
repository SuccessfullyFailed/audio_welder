use crate::{ audio_effect::create_effect_id, AudioEffect };
use std::any::Any;



#[derive(PartialEq)]
pub struct Resampler {
	id:usize,
	sample_rate:f32,
	channel_count:f32
}
impl Resampler {

	/// Create a new resampler.
	pub fn new(sample_rate:u32, channel_count:usize) -> Resampler {
		Resampler {
			id: create_effect_id(),
			sample_rate: sample_rate as f32,
			channel_count: channel_count as f32
		}
	}
}
impl AudioEffect for Resampler {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}

	/// Get the name of the effect.
	fn name(&self) -> &str {
		"resampler"
	}

	/// Return the time multiplier of this effect.
	fn sample_multiplier(&self, sample_rate:u32, channel_count:usize) -> f32 {
		let sample_rate_multiplier:f32 = 1.0 / sample_rate as f32 * self.sample_rate;
		let channel_count_multiplier:f32 = 1.0 / channel_count as f32 * self.channel_count;
		sample_rate_multiplier * channel_count_multiplier
	}
	
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(Resampler {
			id: create_effect_id(),
			sample_rate: self.sample_rate,
			channel_count: self.channel_count
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, sample_rate:&mut u32, channel_count:&mut usize) {
		let target_sample_rate:u32 = self.sample_rate as u32;
		let target_channel_count:usize = self.channel_count as usize;

		// Do nothing if not needed.
		if *sample_rate == target_sample_rate && *channel_count == target_channel_count {
			return;
		}

		// Resample channel count.
		if *channel_count != target_channel_count {
			if target_channel_count < *channel_count {
				*data = data.chunks(*channel_count).map(|chunk| chunk[..target_channel_count].to_vec()).flatten().collect();
			} else {
				*data = data.chunks(*channel_count).map(|chunk| [
					chunk.to_vec(),
					(0..target_channel_count - *channel_count).map(|index| chunk[index % *channel_count]).collect()
				]).flatten().flatten().collect();
			}
			*channel_count = target_channel_count;
		}

		// Resample sample rate.
		if *sample_rate != target_sample_rate {

			// Calculate target to source scale.
			let channel_data:Vec<&[f32]> = data.chunks(1).collect();
			let target_sample_count:f32 = (channel_data.len() as f32 / *sample_rate as f32 * target_sample_rate as f32).floor();
			let target_to_source_scale:f32 = 1.0 / (channel_data.len() - 1) as f32 * target_sample_count as f32;
			let source_sample_count:usize = channel_data.len();

			// Create new buffer.
			let mut new_data:Vec<f32> = Vec::with_capacity(target_sample_count as usize * *channel_count);
			let mut source_index:f32 = 0.0;
			for _ in 0..target_sample_count as usize {
				let cursor_left:usize = (source_index as usize).min(source_sample_count - 2);
				let cursor_right:usize = cursor_left + 1;
				let cursor_factor:f32 = source_index % 1.0;
				for channel_index in 0..*channel_count {
					new_data.push(channel_data[cursor_left][channel_index] + (channel_data[cursor_right][channel_index] - channel_data[cursor_left][channel_index]) * cursor_factor);
				}
				source_index += target_to_source_scale;
			}

			// Set new data.
			*data = new_data;
			*sample_rate = target_sample_rate;
		}
	}
}