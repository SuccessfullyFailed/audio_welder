use std::time::Duration;



pub trait AudioBufferDataLength {
	fn as_buffer_length(self, sample_rate:u32) -> usize;
}
impl AudioBufferDataLength for usize {
	fn as_buffer_length(self, _sample_rate:u32) -> usize {
		self
	}
}
impl AudioBufferDataLength for Duration {
	fn as_buffer_length(self, sample_rate:u32) -> usize {
		(self.as_secs_f32() * sample_rate as f32) as usize
	}
}