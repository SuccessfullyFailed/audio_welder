use std::error::Error;



pub struct AudioBuffer {
	data:Vec<f32>,
	channel_count:usize,
	sample_rate:u32
}
impl AudioBuffer {

	/* CONSTRUCTOR METHODS */

	/// Create a new buffer from a list of samples.
	pub fn from_samples(samples:Vec<f32>, channel_count:usize, sample_rate:u32) -> AudioBuffer {
		AudioBuffer {
			data: samples,
			channel_count,
			sample_rate
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
}