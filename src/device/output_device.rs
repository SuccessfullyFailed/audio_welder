use cpal::{ traits::{ DeviceTrait, StreamTrait }, Device as CpalDevice, Stream, StreamConfig, StreamError };
use std::{ error::Error, thread::sleep, time::Duration };
use crate::{ AudioBuffer, AudioGenerator };
use super::audio_device::AudioDevice;



pub struct OutputDevice {
	device:AudioDevice
}
impl OutputDevice {

	/* CONSTRUCTOR METHODS */

	/// Create a new audio device.
	pub fn new(device_name:&str) -> Result<OutputDevice, Box<dyn Error>> {
		Ok(OutputDevice {
			device: AudioDevice::new(device_name, true)?
		})
	}



	/* PROPERTY GETTER METHODS */

	/// Get the sample rate of the device.
	pub fn sample_rate(&self) -> u32 {
		self.device.sample_rate
	}



	/* AUDIO PLAYING METHODS */

	/// Resample buffer to match audio device sample rate and channel count.
	pub fn prepare_buffer(&self, buffer:&mut AudioBuffer) {
		self.device.prepare_buffer(buffer);
	}

	/// Play a wav file through this device.
	pub fn play_wav(&self, wav:&str) -> Result<(), Box<dyn Error>> {
		self.play(AudioBuffer::wav(wav)?.clone())
	}

	/// Play an audio buffer through this device.
	pub fn play(&self, mut buffer:AudioBuffer) -> Result<(), Box<dyn Error>> {
		
		// Modify buffer sample to fit device.
		self.prepare_buffer(&mut buffer);
		let buffer_duration:Duration = buffer.duration();

		// Create output stream.
		let channel_count:usize = self.device.channel_count;
		let cpal_device:&CpalDevice = &self.device.cpal_device;
		let output_stream:Stream = cpal_device.build_output_stream(
			&StreamConfig {
				channels: self.device.channel_count as u16,
				sample_rate: cpal::SampleRate(self.device.sample_rate),
				buffer_size: cpal_device.default_output_config().unwrap().config().buffer_size
			},
			move |data, _| {
				let new_data:Vec<f32> = buffer.take_flat(data.len() / channel_count);
				data[..new_data.len()].clone_from_slice(&new_data);
			},
			|err:StreamError| panic!("{err}"),
			None
		)?;

		// Play audio and await finish.
		output_stream.play()?;
		sleep(buffer_duration);
		Ok(())
	}
}
impl Default for OutputDevice {
	fn default() -> Self {
		OutputDevice::new("default").unwrap()
	}
}