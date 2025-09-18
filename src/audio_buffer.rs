use crate::{ AudioBufferDataLength, AudioEffect, AudioGenerator, DurationModifier, StereoShaper, VolumeAmplifier };
use std::{ error::Error, ops::Add, time::Duration };



#[derive(Clone, PartialEq)]
enum ProgressionTracker { Cursor(usize), Drain }



#[derive(Clone, PartialEq)]
pub struct AudioBuffer {
	data:Vec<Vec<f32>>,
	channel_count:usize,
	sample_rate:u32,
	effects:Vec<Box<dyn AudioEffect>>,
	progression_tracker:ProgressionTracker
}
impl AudioBuffer {

	/* CONSTRUCTOR METHODS */

	/// Create a new buffer from a list of samples.
	pub fn new(mut samples:Vec<Vec<f32>>, sample_rate:u32) -> AudioBuffer {

		// Grow short channels.
		let data_size:usize = samples.iter().map(|channel| channel.len()).max().unwrap_or(0);
		for channel in &mut samples {
			if channel.len() != data_size {
				channel.extend(vec![0.0; data_size - channel.len()]);
			}
		}

		// Return buffer.
		let channel_count:usize = samples.len();
		AudioBuffer {
			data: samples,
			channel_count,
			sample_rate,
			effects: Vec::new(),
			progression_tracker: ProgressionTracker::Cursor(0)
		}
	}

	/// Read the wav file at the given filepath and return a buffer.
	pub fn wav(file_path:&str) -> Result<AudioBuffer, Box<dyn Error>> {
		use hound::{ WavReader, SampleFormat, WavSpec };
		use std::{ fs::File, io::BufReader };
		
		// Read the WAV file using hound crate.
		let wav_reader:WavReader<BufReader<File>> = WavReader::open(file_path)?;
		let spec:WavSpec = wav_reader.spec();
		let channel_count:usize = spec.channels as usize;
		
		// Retrieve the audio data.
		let mut sample_data:Vec<f32> = match spec.sample_format {
			SampleFormat::Int => wav_reader.into_samples::<i16>().map(|sample| sample.unwrap() as f32 / i16::MAX as f32).collect(),
			SampleFormat::Float => wav_reader.into_samples::<f32>().map(|s| s.unwrap()).collect(),
		};
		if sample_data.len() % channel_count != 0 {
			sample_data.extend(vec![0.0; channel_count - (sample_data.len() % channel_count)]);
		}

		// Convert flat data to channeled data.
		let sample_data:Vec<Vec<f32>> = if channel_count == 1 {
			vec![sample_data]
		} else {
			let mut channel_data:Vec<Vec<f32>> = vec![Vec::with_capacity(sample_data.len() / channel_count); channel_count];
			for (index, sample) in sample_data.iter().enumerate() {
				channel_data[index % channel_count].push(*sample);
			}
			channel_data
		};

		// Return audio buffer.
		Ok(AudioBuffer::new(sample_data, spec.sample_rate))
	}



	/* BUILDER METHODS */

	/// Return self with a new sample rate and channel count.
	pub fn resampled(mut self, sample_rate:u32, channel_count:usize) -> Self {
		let sample_rate_multiplier:f32 = 1.0 / self.sample_rate as f32 * sample_rate as f32;
		let channel_count_multiplier:f32 = 1.0 / self.channel_count as f32 * channel_count as f32;
		let mut sample_rate_modifier:DurationModifier = DurationModifier::new_sample_rate_modifier(sample_rate);
		let mut channel_count_modifier:StereoShaper = StereoShaper::new_channel_count_modifier(channel_count);
		if sample_rate_multiplier < channel_count_multiplier {
			sample_rate_modifier.apply_to(&mut self.data, &mut self.sample_rate, &mut self.channel_count);
			channel_count_modifier.apply_to(&mut self.data, &mut self.sample_rate, &mut self.channel_count);
		} else {
			channel_count_modifier.apply_to(&mut self.data, &mut self.sample_rate, &mut self.channel_count);
			sample_rate_modifier.apply_to(&mut self.data, &mut self.sample_rate, &mut self.channel_count);
		}
		self
	}

	/// Return self with draining progression tracker.
	pub fn drain_progression(mut self) -> Self {
		if let ProgressionTracker::Cursor(cursor) = self.progression_tracker {
			self.data.drain(..cursor);
		}
		self.progression_tracker = ProgressionTracker::Drain;
		self
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



	/* EFFECT SETTING METHODS */

	/// Try to get the value of an effect settings.
	pub fn get_effect_setting(&self, effect_name:&str, setting_name:&str) -> Option<f32> {
		if let Some(effect) = self.effects.iter().find(|effect| effect.name() == effect_name) {
			if let Some((_, setting_value)) = effect.settings().iter().find(|(name, _)| *name == setting_name) {
				return Some(**setting_value);
			}
		}
		None
	}

	/// Try to set the value of an effect settings.
	pub fn set_effect_setting(&mut self, effect_name:&str, setting_name:&str, value:f32) {
		if let Some(effect) = self.effects.iter_mut().find(|effect| effect.name() == effect_name) {
			if let Some((_, setting_value)) = effect.settings_mut().iter_mut().find(|(name, _)| *name == setting_name) {
				**setting_value = value;
			}
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

	/// Get the size of the sample.
	pub(crate) fn sample_size(&self) -> usize {
		self.data.iter().map(|channel| channel.len()).min().unwrap_or(0)
	}

	/// Get the total duration multiplier created by the effects.
	fn effects_duration_multiplier(&self) -> f32 {
		let mut effect_sample_multiplier:f32 = 1.0;
		for effect in &self.effects {
			effect_sample_multiplier *= effect.sample_multiplier(self.sample_rate, self.channel_count);
		}
		effect_sample_multiplier
	}

	/// Get the duration of the sample.
	pub fn duration(&self) -> Duration {
		Duration::from_secs_f32(self.sample_size() as f32 / self.sample_rate as f32 * self.effects_duration_multiplier())
	}

	/// Get the unprocessed data.
	pub fn raw_data(&self) -> &Vec<Vec<f32>> {
		&self.data
	}

	/// Get the processed data flat.
	pub fn processed_data(&mut self) -> &Vec<Vec<f32>> {
		self.apply_effects();
		self.raw_data()
	}

	/// Get the amount of modifications scheduled.
	#[cfg(test)]
	pub(super) fn mod_count(&self) -> usize {
		self.effects.len()
	}

	/// Create a PNG image displaying the data in this buffer.
	#[cfg(test)]
	pub fn write_png(&self, path:&str) -> Result<(), Box<dyn Error>> {
		use image::{ ImageBuffer, Rgba };

		const MAX_IMAGE_WIDTH:u32 = 8000;
		const CHANNEL_HEIGHT:u32 = 200;
		const HALF_CHANNEL_HEIGHT:u32 = CHANNEL_HEIGHT / 2;
		const FILL_COLOR:Rgba<u8> = Rgba([0xFF, 0x00, 0x00, 0x22]);
		const EDGE_COLOR:Rgba<u8> = Rgba([0xFF, 0x00, 0x00, 0xFF]);

		// Calculate image to wave scale.
		let bigget_channel_size:u32 = self.data.iter().map(|channel| channel.len() as u32).max().unwrap_or(0);
		let width:u32 = bigget_channel_size.min(MAX_IMAGE_WIDTH);
		let image_height:u32 = CHANNEL_HEIGHT * self.data.len() as u32;
		let image_to_wave_scale:f32 = bigget_channel_size as f32 / width as f32;

		// Create image.
		let mut img:ImageBuffer<Rgba<_>, Vec<_>> = ImageBuffer::new(width as u32, image_height);
		for channel_index in 0..self.data.len() {
			let channel_y_offset:u32 = CHANNEL_HEIGHT * channel_index as u32;
			let channel_data:&[f32] = &self.data[channel_index];
			for pixel_x in 0..width as u32 {
				let wave_x:f32 = pixel_x as f32 * image_to_wave_scale;
				let wave_y:f32 = channel_data[(wave_x as usize).min(channel_data.len())];
				let pixel_y:u32 = ((channel_y_offset as i32 + HALF_CHANNEL_HEIGHT as i32 + (-wave_y * HALF_CHANNEL_HEIGHT as f32) as i32) as u32).min(CHANNEL_HEIGHT - 1);
				
				for y in HALF_CHANNEL_HEIGHT.min(pixel_y)..HALF_CHANNEL_HEIGHT.max(pixel_y) {
					*img.get_pixel_mut(pixel_x, y) = FILL_COLOR;
				}
				*img.get_pixel_mut(pixel_x, pixel_y.min(CHANNEL_HEIGHT - 1)) = EDGE_COLOR;
			}
		}
		img.save(path)?;

		// Return success.
		Ok(())
	}
}
impl AudioGenerator for AudioBuffer {

	/// Take a specific amount of data.
	fn take<T>(&mut self, duration:T) -> Vec<Vec<f32>> where T:AudioBufferDataLength {

		// Calculate sub-sample size.
		let sample_size:usize = self.sample_size();
		let target_sample_len:usize = duration.as_buffer_length(self.sample_rate).min(sample_size);
		let target_sample_len_before_effects:usize = (target_sample_len as f32 / self.effects_duration_multiplier()) as usize;

		// Grab sub-sample.
		let mut sub_data:Vec<Vec<f32>> = match &mut self.progression_tracker {
			ProgressionTracker::Cursor(cursor) => {
				let start:usize = *cursor;
				*cursor = (*cursor + target_sample_len_before_effects).min(sample_size);
				self.data.iter().map(|channel| channel[start..*cursor].to_vec()).collect::<Vec<Vec<f32>>>()
			},
			ProgressionTracker::Drain => {
				self.data.iter_mut().map(|channel| channel.drain(..target_sample_len_before_effects).collect()).collect::<Vec<Vec<f32>>>()
			}
		};
		for channel in &mut sub_data {
			channel.extend(vec![0.0; target_sample_len_before_effects - channel.len()]);
		}

		// Apply effects.
		if !sub_data.is_empty() {
			let mut sample_rate:u32 = self.sample_rate;
			let mut channel_count:usize = self.channel_count;
			for effect in &mut self.effects {
				effect.apply_to(&mut sub_data, &mut sample_rate, &mut channel_count);
			}
		}

		// Stretch to target length.
		for channel in &mut sub_data {
			if channel.len() < target_sample_len {
				let shortage:usize = target_sample_len - channel.len();
				for insertion_index in 0..shortage {
					let insertion_position:usize = insertion_index * channel.len() / shortage;
					channel.insert(insertion_position, channel[insertion_position]);
				}
			}
			if channel.len() > target_sample_len {
				let overflow:usize = target_sample_len - channel.len();
				for removal_index in 0..overflow {
					let removal_position:usize = removal_index * channel.len() / overflow;
					channel.remove(removal_position);
				}
			}
		}

		// Return data.
		sub_data
	}
}
impl Add<AudioBuffer> for AudioBuffer {
	type Output = AudioBuffer;

	fn add(mut self, rhs:AudioBuffer) -> Self::Output {
		let rhs:AudioBuffer = rhs.resampled(self.sample_rate, self.channel_count);
		self.data.extend(rhs.data);
		self
	}
}