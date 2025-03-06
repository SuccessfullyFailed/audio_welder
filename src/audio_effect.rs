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

	/// Get the ID of the effect.
	fn id(&self) -> usize;

	/// Apply the effect to the given buffer.
	fn apply_to(&mut self, data:&mut Vec<f32>, sample_rate:&mut u32, channel_count:&mut usize);
    
	/// Return the time multiplier of this effect.
	fn sample_multiplier(&self, _sample_rate:u32, _channel_count:usize) -> f32 {
		1.0
	}
    
	/// Try to combine two instances of the audio effect into one.
	fn combine(&self, _other:&dyn AudioEffect) -> Option<Box<dyn AudioEffect>> {
		None
	}
    
	/// Clone the effect into a box.
	fn boxed(&self) -> Box<dyn AudioEffect>;

	/// Allow downcasting.
	fn as_any(&self) -> &dyn Any;
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