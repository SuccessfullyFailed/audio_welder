# audio_welder

`audio_welder` is a Rust crate that enables generating, loading, modifying, and playing audio through audio devices. It provides a simple API for handling audio buffers, applying effects, and outputting sound to a device.

## Features

- Load audio from files (e.g., WAV format)
- Generate audio from wave functions
- Apply effects such as volume amplification and speed modification
- Play audio through output devices

## Installation

Add `audio_welder` to your `Cargo.toml`:

```toml
[dependencies]
audio_welder = { git="https://github.com/SuccessfullyFailed/audio_welder" }
```

## Usage

Here's an example demonstrating how to load an audio file, apply effects, and play it through an output device:

```rust
use audio_welder::{ OutputDevice, AudioBuffer, VolumeAmplifier, DurationModifier };

fn main() {
	let device:OutputDevice = OutputDevice::new("Speakers").unwrap_or(OutputDevice::default());
	let mut buffer:AudioBuffer = AudioBuffer::wav("example.wav").expect("Could not load wav file");
	
	// Effects are not applied emmediately.
	// When the `take` method is used on the buffer, the effects are applied to the buffer taken from the sample.
	// This shortens loading times of buffers with effects.
	buffer.add_effect(VolumeAmplifier::new_maximizer()); // Normalizes volume
	buffer.add_effect(DurationModifier::new(0.5)); // Speeds up playback
	
	device.play(buffer).unwrap();
}
```

## Effects

- `DurationModifier::new(factor)`: Scales the duration and amount of samples by the given factor.
- `DurationModifier::new_sample_rate_modifier(sample_rate)`: Scales the duration and amount of samples to the set samplerate.
- `StereoShaper::new(l2l, r2r, l2r, r2l)`: Modifies and/or flips the left/right balance of the audio.
- `StereoShaper::new_channel_count_modifier(channel_count)`: Modifies the sample to add or subtract the channel count of the sample inputted.
- `DurationModifier::new(factor)`: Adjusts playback speed and pitch, where `factor < 1.0` slows down and `factor > 1.0` speeds up.
- `VolumeAmplifier::new_maximizer()`: Scales the volume so the peak amplitude reaches 1.0 or -1.0.
- `VolumeAmplifier::new_maximizer_to(peak)`: Scales the volume so the peak amplitude reaches the given amount.

- `TapeStop::new(triggered, duration)`: Gradually slows down the audio when triggered. Much like stopping an audio tape.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## TODO / Future Improvements

- More advanced audio effects (reverb, equalization, etc.)
- Asynchronous playback support