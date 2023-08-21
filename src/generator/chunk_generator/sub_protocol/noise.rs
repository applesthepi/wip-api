use std::sync::Arc;

use noise::{utils::NoiseMap, TranslatePoint, NoiseFn};

use core::fmt::Debug;

use crate::PT_MOD_WCOUNT;

pub trait NoiseProxy {
	fn get(
		&self,
		abs_chunk_position: [i32; 2],
		rel_position: [usize; 2],
	) -> f64;
}

pub struct NoiseContainer<N: NoiseFn<f64, 2>> {
	pub noise: N,
}

impl<N: NoiseFn<f64, 2>> NoiseContainer<N> {
	pub fn new(
		noise: N,
	) -> Self {
		Self {
			noise,
		}
	}
}

impl<N: NoiseFn<f64, 2>> NoiseProxy for NoiseContainer<N> {
	fn get(
		&self,
		abs_chunk_position: [i32; 2],
		rel_position: [usize; 2],
	) -> f64 {
		let translation = TranslatePoint::new(&self.noise)
			.set_x_translation((abs_chunk_position[0] as f64 / (PT_MOD_WCOUNT as f64 / 2.0)) as f64)
			.set_y_translation((abs_chunk_position[1] as f64 / (PT_MOD_WCOUNT as f64 / 2.0)) as f64);
		translation.get([rel_position[0] as f64, rel_position[1] as f64])
	}
}

#[derive(Clone)]
pub struct ProtocolNoise3d {
	/// (
	/// 	HEIGHT: 0.0 is level zero, 1.0 is level five.
	/// 	VALID:  <0.5 is none, >0.5 is some.
	/// )
	pub maps: Vec<(
		Arc<dyn NoiseProxy + Send + Sync>,
		Arc<dyn NoiseProxy + Send + Sync>,
	)>,
}

impl Debug for ProtocolNoise3d {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}

#[derive(Clone)]
pub struct ProtocolNoise2d {
	/// VALID: <0.5 is none, >0.5 is some.
	pub maps: Vec<Arc<dyn NoiseProxy + Send + Sync>>,
}

impl Debug for ProtocolNoise2d {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}