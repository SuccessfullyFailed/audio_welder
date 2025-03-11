#[cfg(test)]
mod tests {
	use crate::{ AudioEffect, TapeStop };
	use std::time::Duration;

	/* SETTINGS */
	
	#[test]
	fn test_settings() {
		TapeStop::new(true, Duration::from_millis(100)).settings_test();
	}
}