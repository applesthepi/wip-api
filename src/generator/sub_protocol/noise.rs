use std::sync::Arc;

use noise::NoiseFn;
use wip_primal::{TilePositionRel, ChunkPositionAbs, CHUNK_WIDTH};

pub trait NoiseProxy {
	fn get(
		&self,
		chunk_position_abs: &ChunkPositionAbs,
		tile_position_rel: &TilePositionRel,
	) -> f64;

	// fn write_to_file(
	// 	&self,
	// 	file: &str,
	// 	abs_chunk_position: [i32; 2],
	// );
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
		chunk_position_abs: &ChunkPositionAbs,
		tile_position_rel: &TilePositionRel,
	) -> f64 {
		self.noise.get([
			tile_position_rel.x as f64 + (chunk_position_abs.x as f64 * CHUNK_WIDTH as f64),
			tile_position_rel.y as f64 + (chunk_position_abs.y as f64 * CHUNK_WIDTH as f64),
		])
	}

	// fn write_to_file(
	// 	&self,
	// 	file: &str,
	// 	abs_chunk_position: [i32; 2],
	// ) {
	// 	let translation = TranslatePoint::new(&self.noise)
	// 		.set_x_translation(abs_chunk_position[0] as f64 * 2.0)
	// 		.set_y_translation(abs_chunk_position[1] as f64 * 2.0);
	// 	let noise_map = PlaneMapBuilder::new(translation)
	// 		.set_size(PT_MOD_WCOUNT, PT_MOD_WCOUNT)
	// 		.build();
	// 	noise_map.write_to_file(file);
	// }
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

// impl Debug for ProtocolNoise3d {
// 	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		Ok(())
// 	}
// }

#[derive(Clone)]
pub struct ProtocolNoise2d {
	/// VALID: <0.5 is none, >0.5 is some.
	pub maps: Vec<Arc<dyn NoiseProxy + Send + Sync>>,
}

// impl Debug for ProtocolNoise2d {
// 	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		Ok(())
// 	}
// }