use crate::{ audio_effect::create_effect_id, AudioEffect };
use std::any::Any;



#[derive(PartialEq)]
pub struct NoiseGate {
	id:usize,
	threshold:f32,
	acceleration:f32,
	deceleration:f32,
	position:f32
}
impl NoiseGate {

	/// Create a new volume amplifier.
	pub fn new(threshold:f32, acceleration:f32) -> NoiseGate {
		NoiseGate {
			id: create_effect_id(),
			threshold,
			acceleration,
			deceleration: acceleration,
			position: 0.0
		}
	}
}
impl AudioEffect for NoiseGate {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}

	/// Get the name of the effect.
	fn name(&self) -> &str {
		"noise_gate"
	}
	
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(NoiseGate {
			id: create_effect_id(),
			threshold: self.threshold,
			acceleration: self.acceleration,
			deceleration: self.deceleration,
			position: self.position
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<Vec<f32>>, _sample_rate:&mut u32, _channel_count:&mut usize) {

		// Loop through data.
		for sample_index in 0..data[0].len() {
			for channel in &mut *data {
				let sample:&mut f32 = &mut channel[sample_index];

				// Update noice-gate position.
				if sample.abs() > self.threshold {
					self.position = (self.position + self.acceleration).min(1.0);
				} else {
					self.position = (self.position - self.deceleration).max(0.0);
				}

				// Mod sample.
				if self.position != 1.0 {
					*sample *= self.position;
				}
			}
		}
	}



	/* SETTING METHODS */

	/// Get a list of settings with their names.
	fn settings(&self) -> Vec<(&str, &f32)> {
		vec![
			("threshold", &self.threshold),
			("acceleration", &self.acceleration),
			("deceleration", &self.deceleration),

			("position", &self.position)
		]
	}

	/// Get a mutable list of settings with their names.
	fn settings_mut(&mut self) -> Vec<(&str, &mut f32)> {
		vec![
			("threshold", &mut self.threshold),
			("acceleration", &mut self.acceleration),
			("deceleration", &mut self.deceleration)
		]
	}
}