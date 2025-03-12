#[cfg(test)]
mod tests {
	use crate::{ AudioBuffer, OutputDevice };
	use std::time::Instant;



	#[test]
	fn test_audio_playing() {
		const RAW_SAMPLES:[f32; 16] = [0.0; 16];

		// Prepare output device.
		let default_device:OutputDevice = OutputDevice::default();

		// Prepare buffer.
		let mut buffer:AudioBuffer = AudioBuffer::new(vec![RAW_SAMPLES.to_vec()], 160);
		default_device.prepare_buffer(&mut buffer);
		buffer.apply_effects();
		

		// Play audio.
		let duration_tracker:Instant = Instant::now();
		default_device.play(buffer).unwrap();
		let time_played:u128 = duration_tracker.elapsed().as_millis();
		println!("played for {time_played}ms");
		assert!(time_played > 100 && time_played < 200);
	}
}