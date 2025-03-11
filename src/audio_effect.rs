use std::any::Any;



/// Create an ID for the effect.
pub(crate) fn create_effect_id() -> usize {
	unsafe {
		static mut ID:usize = 0;
		ID += 1;
		ID - 1
	}
}



pub trait AudioEffect:Send + Sync {

	/* PROPERTY GETTER METHODS */

	/// Get the ID of the effect.
	fn id(&self) -> usize;

	/// Get the name of the effect.
	fn name(&self) -> &str;
    
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect>;

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any;
    
	/// Return the time multiplier of this effect.
	fn sample_multiplier(&self, _sample_rate:u32, _channel_count:usize) -> f32 {
		1.0
	}



	/* USAGE METHODS */

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, sample_rate:&mut u32, channel_count:&mut usize);
    
	/// Try to combine two instances of the audio effect into one.
	fn combine(&self, _other:&dyn AudioEffect) -> Option<Box<dyn AudioEffect>> {
		None
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

	/// Get the value of a specific setting.
	fn get_setting(&self, setting_name:&str) -> Option<f32> {
		if let Some(setting) = self.settings().iter().find(|(name, _)| *name == setting_name) {
			Some(*setting.1)
		} else  {
			None
		}
	}

	/// Set the value of a specific setting.
	fn set_setting(&mut self, setting_name:&str, value:f32) {
		if let Some(setting) = self.settings_mut().iter_mut().find(|(name, _)| *name == setting_name) {
			*setting.1 = value;
		}
	}

	/// Test all settings being able to be modified.
	#[cfg(test)]
	fn settings_test(&mut self) {
		let setting_names:Vec<String> = self.settings().iter().map(|(name, _)| name.to_string()).collect();
		for (setting_index, setting_name) in setting_names.iter().enumerate() {
			let new_value:f32 = 867.3 + (setting_index + 8) as f32 * 4.5;
			self.set_setting(setting_name, new_value);
			assert_eq!(self.get_setting(setting_name), Some(new_value));
		}
	}

}
impl Clone for Box<dyn AudioEffect> {
	fn clone(&self) -> Box<dyn AudioEffect> {
		self.boxed()
	}
}
impl PartialEq for Box<dyn AudioEffect> {
	fn eq(&self, other:&Self) -> bool {
		self.id() == other.id()
	}
}