use crate::{AudioEffect, DurationModifier, StereoShaper, VolumeAmplifier};
use std::{ error::Error, time::Duration };



#[derive(Clone, PartialEq)]
pub struct AudioBuffer {
	data:Vec<f32>,
	channel_count:usize,
	sample_rate:u32,

	effects:Vec<Box<dyn AudioEffect>>
}
impl AudioBuffer {

	/* CONSTRUCTOR METHODS */

	/// Create a new buffer from a list of samples.
	pub fn from_samples(samples:Vec<f32>, channel_count:usize, sample_rate:u32) -> AudioBuffer {
		AudioBuffer {
			data: samples,
			channel_count,
			sample_rate,

			effects: Vec::new()
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



	/* EFFECT METHODS */

	/// Add a sample-rate modification. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn resample_sample_rate(&mut self, sample_rate:u32) {
		self.add_effect(DurationModifier::new_sample_rate_modifier(sample_rate));
	}

	/// Add a channel modification. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn resample_channel_count(&mut self, channel_count:usize) {
		self.add_effect(StereoShaper::new_channel_count_modifier(channel_count));
	}

	/// Add a volume multiplication to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn multiply_volume(&mut self, multiplication:f32) {
		self.add_effect(VolumeAmplifier::new(multiplication));
	}

	/// Add a speed multiplication to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn multiply_duration(&mut self, multiplication:f32) {
		self.add_effect(DurationModifier::new(multiplication));
	}

	/// Add stereo flip effect. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn flip_stereo(&mut self, factor:f32) {
		self.add_effect(StereoShaper::new(1.0 - factor, 1.0 - factor, factor, factor));
	}

	/// Add a new effect to the sample. Does not apply it yet. The effect will be applied using the apply_effects method or when the audio is used.
	pub fn add_effect<T>(&mut self, effect:T) where T:AudioEffect {
		self.effects.push(effect.boxed());

		// Combine scheduled effects where possible.
		for right_index in (1..self.effects.len()).rev() {
			let left_index:usize = right_index - 1;
			if let Some(combined) = self.effects[left_index].combine(&*self.effects[right_index]) {
				self.effects[left_index] = combined;
				self.effects.remove(right_index);
			}
		}
	}

	/// Apply all current scheduled effects.
	pub fn apply_effects(&mut self) {
		while !self.effects.is_empty() {
			let mut effect:Box<dyn AudioEffect> = self.effects.remove(0);
			effect.apply_to(&mut self.data, &mut self.sample_rate, &mut self.channel_count);
		}
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
		self.effects.len()
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