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



pub trait AudioFrequency {
	fn to_hz(&self) -> f32;
}
impl AudioFrequency for f32 {
	fn to_hz(&self) -> f32 {
		*self
	}
}
impl AudioFrequency for usize {
	fn to_hz(&self) -> f32 {
		*self as f32
	}
}
impl AudioFrequency for &str {
	fn to_hz(&self) -> f32 {
		const NOTES:&[&str] = &["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#"];
		const DEFAULT_NOTE:&str = "A";
		const DEFAULT_OCTAVE:usize = 4;
		const A4_FREQUENCY:f32 = 440.0;


		// Calculate note and octave.
		let self_upper:String = self.to_uppercase();
		let note_index:usize = match NOTES.iter().position(|note| self_upper.starts_with(note) && !self_upper.starts_with(&(note.to_string() + "#"))) {
			Some(note_index) => note_index,
			None => {
				eprintln!("Could not find note named '{self}', using default of '{DEFAULT_NOTE}'.");
				NOTES.iter().position(|note| *note == DEFAULT_NOTE).unwrap()
			}
		};
		let octave:usize = self_upper.replace(NOTES[note_index], "").trim().parse::<usize>().unwrap_or(DEFAULT_OCTAVE);

		// Calculate frequency.
		// f = f0 * 2 ^ (n / 12) where f0 is the reference pitch and n is the number of semitones above or below the reference pitch.
		let semitones_above_a4:f32 = note_index as f32 + ((octave as f32 - 4.0) * NOTES.len() as f32);
		A4_FREQUENCY * 2.0f32.powf(semitones_above_a4 / 12.0)
	}
}