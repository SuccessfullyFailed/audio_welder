use crate::{audio_effect::create_effect_id, AudioEffect};
use std::any::Any;



#[derive(Clone, PartialEq)]
enum VolumeModifier { Multiplier(f32), Maximize(f32) }



#[derive(PartialEq)]
pub struct VolumeAmplifier {
	id:usize,
	modifier: VolumeModifier
}
impl VolumeAmplifier {

	/// Create a new volume amplifier.
	pub fn new(multiplier:f32) -> VolumeAmplifier {
		VolumeAmplifier {
			id: create_effect_id(),
			modifier: VolumeModifier::Multiplier(multiplier)
		}
	}

	/// Create a volume amplifier that maximizes the volume so the highest peak is 1.0.
	pub fn maximize() -> VolumeAmplifier {
		VolumeAmplifier::maximize_to(1.0)
	}

	/// Create a volume amplifier that maximizes the volume so the highest peak matches the given volume.
	pub fn maximize_to(peak_volume:f32) -> VolumeAmplifier {
		VolumeAmplifier {
			id: create_effect_id(),
			modifier: VolumeModifier::Maximize(peak_volume)
		}
	}
}
impl AudioEffect for VolumeAmplifier {

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, _sample_rate:&mut u32, _channel_count:&mut usize) {
		match self.modifier {
			VolumeModifier::Multiplier(multiplier) => {
				if multiplier != 1.0 {
					data.iter_mut().for_each(|sample| *sample *= multiplier);
				}
			},
			VolumeModifier::Maximize(target_volume) => {
				let mut max:f32 = 0.0;
				for sample in data.iter() {
					let sample_abs:f32 = sample.abs();
					if sample_abs > max {
						max = sample_abs;
					}
				}
				if max != target_volume {
					let multiplier = 1.0 / max * target_volume;
					data.iter_mut().for_each(|sample| *sample *= multiplier);
				}
			}
		}
	}
    
	/// Try to combine two instances of the audio effect into one.
	fn combine(&self, other:&dyn AudioEffect) -> Option<Box<dyn AudioEffect>> {
		if let Some(other) = other.as_any().downcast_ref::<VolumeAmplifier>() {
			Some(
				Box::new(
					match self.modifier {
						VolumeModifier::Multiplier(left_multiplier) => {
							match other.modifier {
								VolumeModifier::Multiplier(right_multiplier) => VolumeAmplifier::new(left_multiplier * right_multiplier),
								VolumeModifier::Maximize(right_peak) => VolumeAmplifier::maximize_to(right_peak)
							}
						},
						VolumeModifier::Maximize(left_peak) => {
							match other.modifier {
								VolumeModifier::Multiplier(right_multiplier) => VolumeAmplifier::maximize_to(left_peak * right_multiplier),
								VolumeModifier::Maximize(right_peak) => VolumeAmplifier::maximize_to(right_peak)
							}
						}
					}
				)
			)
		} else {
			None
		}
	}
	
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(VolumeAmplifier {
			id: create_effect_id(),
			modifier: self.modifier.clone()
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}
}