use crate::{ audio_effect::create_effect_id, AudioEffect };
use std::{ any::Any, time::Duration };



pub enum EffectStatus { None, InProgress, Finished }



pub struct TapeStop {
	id:usize,
	trigger:f32,
	effect_duration_ms:f32,

	speed:f32,
	buffer_cache_cursor:f32,
	required_cache_size:usize,
	buffer_cache:Vec<f32>
}
impl TapeStop {

	/// Create a new stereo-shaper.
	pub fn new(triggered:bool, effect_duration:Duration) -> TapeStop {
		TapeStop {
			id: create_effect_id(),
			trigger: if triggered { 1.0 } else { 0.0 },
			effect_duration_ms: effect_duration.as_millis() as f32,

			speed: 1.0,
			buffer_cache_cursor: 0.0,
			required_cache_size: 0,
			buffer_cache: Vec::new()
		}
	}
}
impl TapeStop {

	/// Get the effect status.
	fn effect_status(&self) -> EffectStatus {
		if self.trigger < 0.5 {
			if self.speed == 1.0 {
				EffectStatus::None
			} else {
				EffectStatus::InProgress
			}
		} else {
			if self.speed == 0.0 {
				EffectStatus::Finished
			} else {
				EffectStatus::InProgress
			}
		}
	}
}
impl AudioEffect for TapeStop {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}
    
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(TapeStop {
			id: create_effect_id(),
			trigger: self.trigger,
			effect_duration_ms: self.effect_duration_ms,

			speed: self.speed,
			buffer_cache_cursor: self.buffer_cache_cursor,
			required_cache_size: self.required_cache_size,
			buffer_cache: self.buffer_cache.clone()
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, sample_rate:&mut u32, channel_count:&mut usize) {
		match self.effect_status() {
			EffectStatus::None => {
				if self.speed != 1.0 {
					self.speed = 1.0;
				}
			},
			EffectStatus::InProgress => {

				// Grow data cache.
				if self.buffer_cache_cursor == 0.0 {
					self.required_cache_size = (self.effect_duration_ms * *sample_rate as f32) as usize * *channel_count;
				}
				if self.buffer_cache.len() < self.required_cache_size {
					self.buffer_cache.extend_from_slice(data);
				}

				// Apply effect.
				let data_duration_ms:f32 = (data.len() as f32 / *channel_count as f32) / *sample_rate as f32 * 1000.0;
				for source_sample_index in 0..data.len() {
					self.buffer_cache_cursor += self.speed;
					let source_index_left:usize = (self.buffer_cache_cursor.floor() as usize).min(self.buffer_cache.len() - 2);
					let source_index_right:usize = source_index_left + 1;
					let source_index_fact:f32 = self.buffer_cache_cursor % 1.0;
					data[source_sample_index] = self.buffer_cache[source_index_left] + (self.buffer_cache[source_index_right] - self.buffer_cache[source_index_left]) * source_index_fact;
				}
				self.speed = (self.speed - (1.0 / self.effect_duration_ms * data_duration_ms)).max(0.0);
			},
			EffectStatus::Finished => {
				data.iter_mut().for_each(|sample| *sample = 0.0);
				self.buffer_cache_cursor = 0.0;
				self.required_cache_size = 0;
				self.buffer_cache = Vec::new();
			}
		}
	}



	/* SETTING METHODS */

	/// Get a list of settings with their names.
	fn settings(&self) -> Vec<(&str, &f32)> {
		Vec::new()
	}

	/// Get a mutable list of settings with their names.
	fn settings_mut(&mut self) -> Vec<(&str, &mut f32)> {
		Vec::new()
	}
}