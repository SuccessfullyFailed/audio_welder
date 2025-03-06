use crate::{audio_effect::create_effect_id, AudioEffect};
use std::any::Any;



#[derive(PartialEq)]
pub struct DurationModifier {
	id:usize,
	target_sample_rate:Option<u32>,
	multiplier:f32
}
impl DurationModifier {

	/// Create a new duration multiplier.
	pub fn new(multiplier:f32) -> DurationModifier {
		DurationModifier {
			id: create_effect_id(),
			target_sample_rate: None,
			multiplier
		}
	}

	/// Create a duration multiplier that sets the sample-rate to the given sample-rate.
	pub fn new_sample_rate_modifier(sample_rate:u32) -> DurationModifier {
		DurationModifier {
			id: create_effect_id(),
			target_sample_rate: Some(sample_rate),
			multiplier: 1.0
		}
	}
}
impl AudioEffect for DurationModifier {

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, sample_rate:&mut u32, _channel_count:&mut usize) {

		// Reverse data if factor is less than 0.
		let mut multiplier:f32 = self.sample_multiplier(*sample_rate, *_channel_count);
		if multiplier < 0.0 {
			data.reverse();
			multiplier = multiplier.abs();
		}

		// Return if no change needed.
		if multiplier == 1.0 {
			return;
		}

		// Calculate how much to increment the source index per each incrementation of the target index.
		let source_sample_count:f32 = data.len() as f32;
		let target_sample_count:f32 = (source_sample_count * multiplier).floor();
		let source_index_increment:f32 = 1.0 / multiplier;
		let source_index_max:usize = source_sample_count as usize - 1;

		// For each new sample, calculate a new sample based on the progress between samples in the source.
		let mut new_data:Vec<f32> = Vec::with_capacity(target_sample_count as usize);
		let mut source_index:f32 = 0.0;
		while source_index < source_sample_count {
			let source_index_left:usize = source_index.floor() as usize;
			let source_index_right:usize = (source_index_left + 1).min(source_index_max);
			let source_index_fact:f32 = source_index % 1.0;
			new_data.push(data[source_index_left] + (data[source_index_right] - data[source_index_left]) * source_index_fact);
			
			source_index += source_index_increment;
		}

		// Set new data.
		if let Some(rate) = self.target_sample_rate {
			*sample_rate = rate;
		}
		*data = new_data;
	}

	/// Return the time multiplier of this effect.
	fn sample_multiplier(&self, sample_rate:u32, _channel_count:usize) -> f32 {
		(1.0 / sample_rate as f32 * self.target_sample_rate.unwrap_or(sample_rate) as f32) * self.multiplier
	}

	// Try to combine two instances of the audio effect into one.
	fn combine(&self, other:&dyn AudioEffect) -> Option<Box<dyn AudioEffect>> {
		if let Some(other) = other.as_any().downcast_ref::<DurationModifier>() {
			Some(
				Box::new(
					if let Some(target_sample_rate) = other.target_sample_rate {
						DurationModifier { id: create_effect_id(), target_sample_rate: Some(target_sample_rate), multiplier: other.multiplier }
					} else {
						DurationModifier { id: create_effect_id(), target_sample_rate: self.target_sample_rate, multiplier: self.multiplier * other.multiplier }
					}
				)
			)
		} else {
			None
		}
	}
	
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(DurationModifier {
			id: create_effect_id(),
			target_sample_rate: self.target_sample_rate.clone(),
			multiplier: self.multiplier
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}
}