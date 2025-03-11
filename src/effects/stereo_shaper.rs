use crate::{ audio_effect::create_effect_id, AudioEffect };
use std::any::Any;



#[derive(PartialEq)]
pub struct StereoShaper {
	id:usize,
	left_to_left:f32,
	right_to_right:f32, 
	left_to_right:f32,
	right_to_left:f32,
	target_channel_count:Option<f32>
}
impl StereoShaper {

	/// Create a new stereo-shaper.
	pub fn new(left_to_left:f32, right_to_right:f32, left_to_right:f32, right_to_left:f32) -> StereoShaper {
		StereoShaper {
			id: create_effect_id(),
			left_to_left,
			right_to_right,
			left_to_right,
			right_to_left,
			target_channel_count: None
		}
	}

	/// Create a stereo-shaper that modifies the channel count.
	pub fn new_channel_count_modifier(channel_count:usize) -> StereoShaper {
		StereoShaper {
			id: create_effect_id(),
			left_to_left: 1.0,
			right_to_right: 1.0,
			left_to_right: 1.0,
			right_to_left: 1.0,
			target_channel_count: Some(channel_count as f32)
		}
	}
}
impl AudioEffect for StereoShaper {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize {
		self.id
	}

	/// Get the name of the effect.
	fn name(&self) -> &str {
		"stereo_shaper"
	}

	/// Return the time multiplier of this effect.
	fn sample_multiplier(&self, _sample_rate:u32, channel_count:usize) -> f32 {
		1.0 / channel_count as f32 * self.target_channel_count.unwrap_or(channel_count as f32)
	}
    
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect> {
		Box::new(StereoShaper {
			id: create_effect_id(),
			left_to_left: self.left_to_left,
			right_to_right: self.right_to_right,
			left_to_right: self.left_to_right,
			right_to_left: self.right_to_left,
			target_channel_count: self.target_channel_count.clone()
		})
	}

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any {
		self
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, _sample_rate:&mut u32, channel_count:&mut usize) {

		// Modify channel count.
		if let Some(target_channel_count) = self.target_channel_count {
			let target_channel_count:usize = target_channel_count as usize;
			if *channel_count != target_channel_count {
				if *channel_count == 0 || target_channel_count == 0 {
					*data = Vec::new();
				} else if target_channel_count < *channel_count {
					*data = data.chunks(*channel_count).map(|chunk| &chunk[..target_channel_count]).flatten().cloned().collect();
				} else {
					let mut new_data:Vec<f32> = Vec::with_capacity(data.len() / *channel_count * target_channel_count);
					let mut cursor:usize = 0;
					let cursor_end:usize = data.len() - (*channel_count - 1);
					while cursor < cursor_end {
						new_data.extend_from_slice(&data[cursor..cursor + *channel_count]);
						for addition_index in 0..target_channel_count - *channel_count {
							new_data.push(data[cursor + (addition_index % *channel_count)]);
						}
						cursor += *channel_count;
					}
					*data = new_data;
				}
				*channel_count = target_channel_count as usize;
			}
		}

		// Modify stereo data.
		if *channel_count == 2 && self.left_to_left != 1.0 || self.right_to_right != 1.0 || self.left_to_right != 1.0 || self.right_to_left != 1.0 {
			let mut new_samples:Vec<f32> = Vec::with_capacity(data.len());

			let mut cursor:usize = 0;
			while cursor < data.len() {
				let left:f32 = data[cursor];
				let right:f32 = data[cursor + 1];
				let new_left:f32 = left * self.left_to_left + right * self.right_to_left;
				let new_right:f32 = right * self.right_to_right + left * self.left_to_right;
				new_samples.push(new_left);
				new_samples.push(new_right);
				for channel in 2..*channel_count {
					new_samples.push(data[cursor + channel]);
				}
				cursor += *channel_count;
			}
			*data = new_samples;
		}
	}



	/* SETTING METHODS */

	/// Get a list of settings with their names.
	fn settings(&self) -> Vec<(&str, &f32)> {
		if let Some(target_channel_count) = &self.target_channel_count {
			vec![
				("left_to_left", &self.left_to_left),
				("right_to_right", &self.right_to_right),
				("left_to_right", &self.left_to_right),
				("right_to_left", &self.right_to_left),
				("target_channel_count", target_channel_count)
			]	
		} else {
			vec![
				("left_to_left", &self.left_to_left),
				("right_to_right", &self.right_to_right),
				("left_to_right", &self.left_to_right),
				("right_to_left", &self.right_to_left)
			]
		}
	}

	/// Get a mutable list of settings with their names.
	fn settings_mut(&mut self) -> Vec<(&str, &mut f32)> {
		if let Some(target_channel_count) = &mut self.target_channel_count {
			vec![
				("left_to_left", &mut self.left_to_left),
				("right_to_right", &mut self.right_to_right),
				("left_to_right", &mut self.left_to_right),
				("right_to_left", &mut self.right_to_left),
				("target_channel_count", target_channel_count)
			]	
		} else {
			vec![
				("left_to_left", &mut self.left_to_left),
				("right_to_right", &mut self.right_to_right),
				("left_to_right", &mut self.left_to_right),
				("right_to_left", &mut self.right_to_left)
			]
		}
	}
}