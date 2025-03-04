use std::error::Error;



pub struct AudioMod { factor:f32, mod_type:AudioModType }
#[derive(PartialEq)]
pub enum AudioModType { Volume, Duration }



pub struct AudioBuffer {
	data:Vec<f32>,
	channel_count:usize,
	sample_rate:u32,

	modifications:Vec<AudioMod>
}
impl AudioBuffer {

	/* CONSTRUCTOR METHODS */

	/// Create a new buffer from a list of samples.
	pub fn from_samples(samples:Vec<f32>, channel_count:usize, sample_rate:u32) -> AudioBuffer {
		AudioBuffer {
			data: samples,
			channel_count,
			sample_rate,

			modifications: Vec::new()
		}
	}

	/// Create a new buffer from a list of channel data.
	pub fn from_channels(channels_data:Vec<Vec<f32>>, sample_rate:u32) -> AudioBuffer {
		let channel_count:usize = channels_data.len();
		AudioBuffer::from_samples(
			channels_data.into_iter().flatten().collect(),
			channel_count,
			sample_rate
		)
	}

	/// Read the wav file at the given filepath and return a buffer.
	pub fn from_wav(file_path:&str) -> Result<AudioBuffer, Box<dyn Error>> {
		use hound::{ WavReader, SampleFormat, WavSpec };
		use std::{ fs::File, io::BufReader };
		
		// Read the WAV file using hound crate.
		let wav_reader:WavReader<BufReader<File>> = WavReader::open(file_path)?;
		let spec:WavSpec = wav_reader.spec();
		
		// Retrieve the audio data.
		let sample_data:Vec<f32> = match spec.sample_format {
			SampleFormat::Int => wav_reader.into_samples::<i16>().map(|sample| sample.unwrap() as f32 / i16::MAX as f32).collect(),
			SampleFormat::Float => wav_reader.into_samples::<f32>().map(|s| s.unwrap()).collect(),
		};

		// Return audio buffer.
		Ok(AudioBuffer::from_samples(sample_data, spec.channels as usize, spec.sample_rate))
	}



	/* EFFECT SCHEDULING */

	/// Add a volume multiplication to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn multiply_volume(&mut self, multiplication:f32) {
		self.modifications.push(AudioMod { factor: multiplication, mod_type: AudioModType::Volume });
		self.combine_scheduled_modifications();
	}

	/// Add a speed multiplication to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn multiply_duration(&mut self, multiplication:f32) {
		self.modifications.push(AudioMod { factor: multiplication, mod_type: AudioModType::Duration });
		self.combine_scheduled_modifications();
	}

	/// Combine scheduled effects where possible.
	fn combine_scheduled_modifications(&mut self) {
		for right_index in (1..self.modifications.len()).rev() {
			let left_index:usize = right_index - 1;
			if self.modifications[left_index].mod_type == self.modifications[right_index].mod_type {
				self.modifications[left_index].factor *= self.modifications[right_index].factor;
				self.modifications.remove(right_index);
			}
		}
	}



	/* EFFECT APPLICATION */

	/// Apply all current scheduled effects.
	pub fn apply_effects(&mut self) {
		while (!self.modifications.is_empty()) {
			let effect:AudioMod = self.modifications.remove(0);
			match effect.mod_type {
				AudioModType::Volume => self.apply_volume_modification(effect.factor),
				AudioModType::Duration => self.apply_speed_modification(effect.factor),
			}
		}
	}

	/// Apply a volume modification to the data of the buffer.
	fn apply_volume_modification(&mut self, factor:f32) {
		self.data.iter_mut().for_each(|sample| *sample *= factor);
	}

	/// Apply a speed modification to the data of the buffer.
	fn apply_speed_modification(&mut self, mut factor:f32) {

		// Reverse data if factor is less than 0.
		if factor < 0.0 {
			self.data.reverse();
			factor = factor.abs();
		}

		// Calculate how much to increment the source index per each incrementation of the target index.
		let source_sample_count:f32 = self.data.len() as f32;
		let target_sample_count:f32 = source_sample_count * factor.floor();
		let source_index_increment:f32 = 1.0 / factor;

		// For each new sample, calculate a new sample based on the progress between samples in the source.
		let mut new_data:Vec<f32> = Vec::with_capacity(target_sample_count as usize);
		let mut source_index:f32 = 0.0;
		while source_index < source_sample_count {
			let source_index_left:usize = source_index.floor() as usize;
			let source_index_right:usize = source_index_left + 1;
			let source_index_fact:f32 = source_index % 1.0;
			new_data.push((self.data[source_index_left] * source_index_fact + self.data[source_index_right] * (1.0 - source_index_fact)) * 0.5);
			
			source_index += source_index_increment;
		}
		self.data = new_data
	}
}