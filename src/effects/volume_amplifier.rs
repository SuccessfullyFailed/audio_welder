use crate::{ audio_effect::create_effect_id, AudioEffect };
use std::any::Any;



#[derive(PartialEq)]
pub struct VolumeAmplifier {
	id:usize,
	maximize_target_volume:Option<f32>,
	multiplier:f32
}
impl VolumeAmplifier {

	/// Create a new volume amplifier.
	pub fn new(multiplier:f32) -> VolumeAmplifier {
		VolumeAmplifier {
			id: create_effect_id(),
			maximize_target_volume: None,
			multiplier
		}
	}

	/// Create a volume amplifier that maximizes the volume so the highest peak is 1.0.
	pub fn new_maximizer() -> VolumeAmplifier {
		VolumeAmplifier::new_maximizer_to(1.0)
	}

	/// Create a volume amplifier that maximizes the volume so the highest peak matches the given volume.
	pub fn new_maximizer_to(peak_volume:f32) -> VolumeAmplifier {
		VolumeAmplifier {
			id: create_effect_id(),
			maximize_target_volume: Some(peak_volume),
			multiplier: 1.0
		}
	}
}
impl AudioEffect for VolumeAmplifier {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}
	
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(VolumeAmplifier {
			id: create_effect_id(),
			maximize_target_volume: self.maximize_target_volume,
			multiplier: self.multiplier
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, _sample_rate:&mut u32, _channel_count:&mut usize) {
		let mut multiplier:f32 = 1.0;

		if let Some(target_volume) = self.maximize_target_volume {
			let mut max:f32 = 0.0;
			for sample in data.iter() {
				let sample_abs:f32 = sample.abs();
				if sample_abs > max {
					max = sample_abs;
				}
			}
			multiplier = 1.0 / max * target_volume;
		}
		multiplier *= self.multiplier;
		if multiplier != 1.0 {
			data.iter_mut().for_each(|sample| *sample *= multiplier);
		}
	}
    
	/// Try to combine two instances of the audio effect into one.
	fn combine(&self, other:&dyn AudioEffect) -> Option<Box<dyn AudioEffect>> {
		if let Some(other) = other.as_any().downcast_ref::<VolumeAmplifier>() {
			Some(Box::new(
				if let Some(other_maximize_target) = other.maximize_target_volume {
					VolumeAmplifier {
						id: create_effect_id(),
						maximize_target_volume: Some(other_maximize_target),
						multiplier: other.multiplier
					}
				} else {
					VolumeAmplifier {
						id: create_effect_id(),
						maximize_target_volume: self.maximize_target_volume,
						multiplier: self.multiplier * other.multiplier
					}
				}
			))
		} else {
			None
		}
	}



	/* SETTING METHODS */

	/// Get a list of settings with their names.
	fn settings(&self) -> Vec<(&str, &f32)> {
		if let Some(maximize_target_volume) = &self.maximize_target_volume {
			vec![
				("maximize_target_volume", maximize_target_volume),
				("multiplier", &self.multiplier)
			]	
		} else {
			vec![
				("multiplier", &self.multiplier)
			]
		}
	}

	/// Get a mutable list of settings with their names.
	fn settings_mut(&mut self) -> Vec<(&str, &mut f32)> {
		if let Some(maximize_target_volume) = &mut self.maximize_target_volume {
			vec![
				("maximize_target_volume", maximize_target_volume),
				("multiplier", &mut self.multiplier)
			]	
		} else {
			vec![
				("multiplier", &mut self.multiplier)
			]
		}
	}
}