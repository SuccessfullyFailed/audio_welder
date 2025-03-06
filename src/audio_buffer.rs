use std::{ error::Error, time::Duration };



#[derive(Clone, PartialEq)]
pub struct AudioMod { factor:f32, mod_type:AudioModType }
#[derive(Clone, PartialEq)]
pub enum AudioModType { Volume, Duration, SampleRate, ChannelCount }



#[derive(Clone, PartialEq)]
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
	pub fn from_channels(mut channels_data:Vec<Vec<f32>>, sample_rate:u32) -> AudioBuffer {
		if channels_data.is_empty() {
			return AudioBuffer::from_samples(Vec::new(), 0, sample_rate);
		}

		// Fill shortest channels data with 0.
		let max_size:usize = channels_data.iter().map(|channel| channel.len()).max().unwrap();
		for channel in &mut channels_data {
			channel.extend(vec![0.0; max_size - channel.len()]);
		}

		// Return new buffer.
		let channel_count:usize = channels_data.len();
		AudioBuffer::from_samples(
			(0..channels_data[0].len()).map(|sample_index| channels_data.iter().map(|channel| channel[sample_index]).collect::<Vec<f32>>()).flatten().collect(),
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
		self.add_effect(multiplication, AudioModType::Volume);
	}

	/// Add a speed multiplication to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn multiply_duration(&mut self, multiplication:f32) {
		self.add_effect(multiplication, AudioModType::Duration);
	}

	/// Add a sample-rate modification. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn resample_sample_rate(&mut self, sample_rate:u32) {
		self.add_effect(sample_rate as f32, AudioModType::SampleRate);
	}

	/// Add a channel modification. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn resample_channel_count(&mut self, channel_count:usize) {
		self.add_effect(channel_count as f32, AudioModType::ChannelCount);
	}

	/// Add a new effect to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	fn add_effect(&mut self, factor:f32, mod_type:AudioModType) {
		self.modifications.push(AudioMod { factor, mod_type });

		// Combine scheduled effects where possible.
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
		while !self.modifications.is_empty() {
			let effect:AudioMod = self.modifications.remove(0);
			match effect.mod_type {
				AudioModType::Volume => self.apply_volume_modification(effect.factor),
				AudioModType::Duration => self.apply_duration_modification(effect.factor),
				AudioModType::SampleRate => self.apply_sample_rate_modification(effect.factor as u32),
				AudioModType::ChannelCount => self.apply_channel_count_modification(effect.factor as usize),
			}
		}
	}

	/// Apply a volume modification to the data of the buffer.
	fn apply_volume_modification(&mut self, factor:f32) {
		if factor != 0.0 {
			self.data.iter_mut().for_each(|sample| *sample *= factor);
		}
	}

	/// Apply a speed modification to the data of the buffer.
	fn apply_duration_modification(&mut self, mut factor:f32) {

		// Reverse data if factor is less than 0.
		if factor < 0.0 {
			self.data.reverse();
			factor = factor.abs();
		}

		// Return if no change needed.
		if factor == 1.0 {
			return;
		}

		// Calculate how much to increment the source index per each incrementation of the target index.
		let source_sample_count:f32 = self.data.len() as f32;
		let target_sample_count:f32 = (source_sample_count * factor).floor();
		let source_index_increment:f32 = 1.0 / factor;
		let source_index_max:usize = source_sample_count as usize - 1;

		// For each new sample, calculate a new sample based on the progress between samples in the source.
		let mut new_data:Vec<f32> = Vec::with_capacity(target_sample_count as usize);
		let mut source_index:f32 = 0.0;
		while source_index < source_sample_count {
			let source_index_left:usize = source_index.floor() as usize;
			let source_index_right:usize = (source_index_left + 1).min(source_index_max);
			let source_index_fact:f32 = source_index % 1.0;
			new_data.push(self.data[source_index_left] + (self.data[source_index_right] - self.data[source_index_left]) * source_index_fact);
			
			source_index += source_index_increment;
		}
		self.data = new_data;
	}

	/// Modify the sample rate of the buffer..
	fn apply_sample_rate_modification(&mut self, sample_rate:u32) {
		if self.sample_rate != sample_rate {
			self.apply_duration_modification(1.0 / self.sample_rate as f32 * sample_rate as f32);
			self.sample_rate = sample_rate;
		}
	}

	/// Modify the channel count of the buffer.
	fn apply_channel_count_modification(&mut self, channel_count:usize) {

		// Size same.
		if channel_count == self.channel_count {
		}

		// Zero size.
		else if channel_count == 0 || self.channel_count == 0 {
			self.data = Vec::new();
		}

		// Size down
		else if channel_count < self.channel_count {
			self.data = self.data.chunks(self.channel_count).map(|chunk| &chunk[..channel_count]).flatten().cloned().collect();
		}
		
		// Size up.
		else {
			let mut new_data:Vec<f32> = Vec::with_capacity(self.data.len() / self.channel_count * channel_count);
			let mut cursor:usize = 0;
			let cursor_end:usize = self.data.len() - (self.channel_count - 1);
			while cursor < cursor_end {
				new_data.extend_from_slice(&self.data[cursor..cursor + self.channel_count]);
				for addition_index in 0..channel_count - self.channel_count {
					new_data.push(self.data[cursor + (addition_index % self.channel_count)]);
				}
				cursor += self.channel_count;
			}
			self.data = new_data;
		}

		self.channel_count = channel_count;
	}



	/* PROPERTY GETTER METHODS */

	/// Get the channel-count.
	pub fn channel_count(&self) -> usize {
		self.channel_count
	}

	/// Get the sample-rate in samples per second.
	pub fn sample_rate(&self) -> u32 {
		self.sample_rate
	}

	/// Get the duration of the sample.
	pub fn duration(&self) -> Duration {
		Duration::from_secs_f32(self.data.len() as f32 / self.channel_count as f32 / self.sample_rate as f32)
	}

	/// Get the unprocessed data flat.
	pub fn raw_data(&self) -> &Vec<f32> {
		&self.data
	}

	/// Get the unprocessed data in separate lists per channel.
	pub fn raw_channels_data(&self) -> Vec<Vec<f32>> {
		match self.channel_count {
			0 => Vec::new(),
			1 => vec![self.data.clone()],
			channel_count => {
				let channel_sample_count:usize = self.data.len() / channel_count + 1;
				let mut channels_data:Vec<Vec<f32>> = vec![Vec::with_capacity(channel_sample_count); channel_count];
				let mut channel_index:usize = 0;
				for sample in self.data.clone() {
					channels_data[channel_index].push(sample);
					channel_index += 1;
					if channel_index == channel_count {
						channel_index = 0;
					}
				}
				channels_data
			}
		}
	}

	/// Get the processed data flat.
	pub fn processed_data(&mut self) -> &Vec<f32> {
		self.apply_effects();
		self.raw_data()
	}

	/// Get the processed data flat.
	pub fn processed_channels_data(&mut self) -> Vec<Vec<f32>> {
		self.apply_effects();
		self.raw_channels_data()
	}

	/// Take a specific duration of data.
	pub fn take_processed_data<T>(&mut self, duration:T) -> Vec<f32> where T:AudioBufferDataLength {
		self.apply_effects();
		self.data.drain(..duration.as_buffer_length(self).min(self.data.len())).collect()
	}

	#[cfg(test)]
	/// Get the amount of modifications scheduled.
	pub(super) fn mod_count(&self) -> usize {
		self.modifications.len()
	}
}



pub trait AudioBufferDataLength {
	fn as_buffer_length(self, buffer:&AudioBuffer) -> usize;
}
impl AudioBufferDataLength for usize {
	fn as_buffer_length(self, buffer:&AudioBuffer) -> usize {
		self * buffer.channel_count
	}
}
impl AudioBufferDataLength for Duration {
	fn as_buffer_length(self, buffer:&AudioBuffer) -> usize {
		(self.as_secs_f32() * buffer.sample_rate as f32) as usize * buffer.channel_count
	}
}