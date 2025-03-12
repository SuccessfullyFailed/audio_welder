use crate::{ audio_effect::create_effect_id, AudioEffect };
use std::any::Any;



#[derive(PartialEq)]
pub struct DurationModifier {
	id:usize,
	target_sample_rate:Option<f32>,
	duration_multiplier:f32
}
impl DurationModifier {

	/// Create a new duration multiplier.
	pub fn new(multiplier:f32) -> DurationModifier {
		DurationModifier {
			id: create_effect_id(),
			target_sample_rate: None,
			duration_multiplier: multiplier
		}
	}

	/// Create a duration multiplier that sets the sample-rate to the given sample-rate.
	pub fn new_sample_rate_modifier(sample_rate:u32) -> DurationModifier {
		DurationModifier {
			id: create_effect_id(),
			target_sample_rate: Some(sample_rate as f32),
			duration_multiplier: 1.0
		}
	}
}
impl AudioEffect for DurationModifier {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}

	/// Get the name of the effect.
	fn name(&self) -> &str {
		"duration_modifier"
	}

	/// Return the time multiplier of this effect.
	fn sample_multiplier(&self, sample_rate:u32, _channel_count:usize) -> f32 {
		(1.0 / sample_rate as f32 * self.target_sample_rate.unwrap_or(sample_rate as f32) as f32) * self.duration_multiplier
	}
	
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(DurationModifier {
			id: create_effect_id(),
			target_sample_rate: self.target_sample_rate.clone(),
			duration_multiplier: self.duration_multiplier
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<Vec<f32>>, sample_rate:&mut u32, channel_count:&mut usize) {
		print!("\t\t\t\tFROM {} TO ", data[0].len());

		// Reverse data if factor is less than 0.
		let mut multiplier:f32 = self.sample_multiplier(*sample_rate, *channel_count);
		if multiplier < 0.0 {
			data.reverse();
			multiplier = multiplier.abs();
		}

		// Return if no change needed.
		if multiplier == 1.0 || data.is_empty() {
			return;
		}

		// Calculate how much to increment the source index per each incrementation of the target index.
		let source_sample_count:f32 = data[0].len() as f32;
		let target_sample_count:f32 = (source_sample_count * multiplier).floor();
		let source_index_increment:f32 = 1.0 / multiplier;
		let source_index_max:usize = source_sample_count as usize - 1;

		// For each new sample, calculate a new sample based on the progress between samples in the source.
		let mut new_data:Vec<Vec<f32>> = vec![Vec::with_capacity(target_sample_count as usize); *channel_count];
		let mut source_index:f32 = 0.0;
		while source_index < source_sample_count {
			let source_index_left:usize = (source_index.floor() as usize).min(source_index_max);
			let source_index_right:usize = (source_index_left + 1).min(source_index_max);
			let source_index_fact:f32 = source_index % 1.0;
			for channel_index in 0..*channel_count {
				new_data[channel_index].push(data[channel_index][source_index_left] + (data[channel_index][source_index_right] - data[channel_index][source_index_left]) * source_index_fact);
			}
			
			source_index += source_index_increment;
		}

		// Set new data.
		if let Some(rate) = self.target_sample_rate {
			*sample_rate = rate as u32;
		}
		*data = new_data;
	}



	/* SETTING METHODS */

	/// Get a list of settings with their names.
	fn settings(&self) -> Vec<(&str, &f32)> {
		if let Some(sample_rate) = &self.target_sample_rate {
			vec![
				("target_sample_rate", sample_rate),
				("duration_multiplier", &self.duration_multiplier)
			]	
		} else {
			vec![
				("duration_multiplier", &self.duration_multiplier)
			]
		}
	}

	/// Get a mutable list of settings with their names.
	fn settings_mut(&mut self) -> Vec<(&str, &mut f32)> {
		if let Some(sample_rate) = &mut self.target_sample_rate {
			vec![
				("target_sample_rate", sample_rate),
				("duration_multiplier", &mut self.duration_multiplier)
			]	
		} else {
			vec![
				("duration_multiplier", &mut self.duration_multiplier)
			]
		}
	}
}