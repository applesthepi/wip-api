use std::sync::Arc;

use noise::utils::NoiseMap;

use core::fmt::Debug;

#[derive(Clone)]
pub struct ProtocolNoise {
	/// (
	/// 	HEIGHT: 0.0 is level zero, 1.0 is level five.
	/// 	VALID:  <0.5 is none, >0.5 is some.
	/// )
	pub maps: Vec<(Arc<NoiseMap>, Arc<NoiseMap>)>,
}

impl Debug for ProtocolNoise {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}