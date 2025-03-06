use cpal::{ traits::{ DeviceTrait, HostTrait }, Device as CpalDevice, Host, SupportedStreamConfig };
use std::error::Error;

use crate::AudioBuffer;



pub(super) struct AudioDevice {
	pub channel_count:usize,
	pub sample_rate:u32,
	pub cpal_device:CpalDevice
}
impl AudioDevice {

	/* CONSTRUCTOR METHODS */

	/// Create a new audio device.
	pub fn new(device_name:&str, is_output_device:bool) -> Result<AudioDevice, Box<dyn Error>> {
		let device:CpalDevice = AudioDevice::find_device(device_name, is_output_device)?;
		let config:SupportedStreamConfig = if is_output_device { device.default_output_config() } else { device.default_input_config() }?;
		Ok(AudioDevice {
			channel_count: config.channels() as usize,
			sample_rate: config.sample_rate().0,
			cpal_device: device
		})
	}

	/// Find a specific device.
	fn find_device(name:&str, is_output_device:bool) -> Result<CpalDevice, Box<dyn Error>> {
		let host:Host = cpal::default_host();

		// Try default.
		if name.to_lowercase() == "default" {
			return match if is_output_device { host.default_output_device() } else { host.default_input_device() } {
				Some(device) => Ok(device),
				None => Err("Unable to find default device".into())
			}
		}

		// Create list of devices.
		let devices = if is_output_device { host.output_devices()? } else { host.input_devices()? };

		// Filter for name.
		for device in devices.into_iter() {
			if let Ok(device_name) = &device.name() {
				if device_name.contains(name){
					return Ok(device);
				}
			}
		}

		Err(format!("Could not find {} device '{}'.", if is_output_device { "output" } else { "input" }, name).into())
	}



	/* USAGE METHODS */

	/// Resample buffer to match audio device sample rate and channel count.
	pub fn prepare_buffer(&self, buffer:&mut AudioBuffer) {
		buffer.resample_sample_rate(self.sample_rate);
		buffer.resample_channel_count(self.channel_count);
	}
}