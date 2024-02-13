use std::{str::FromStr, sync::Arc};

use bevy::prelude::info;
use wip_primal::{TilePositionRel, TilePositionAbs, ChunkPositionAbs, Bounds};

use crate::{RTTerrain, TileTerrain, TCHardness, ProtocolTerrainForm, ProtocolNoise3d, Protocol, IntermediateChunk, RawPtr, NoiseProxy, Gen, SubsurfaceConfig, prelude::{op_11_03, op_11_01}, ConfigFlat, ConfigNoise};

#[derive(Clone)]
pub struct ProtocolTerrain {
	pub un_protocol: String,
	pub tile: TileTerrain,
	pub form: ProtocolTerrainForm,
}

impl ProtocolTerrain {
	pub fn new(
		un_protocol: impl Into<String>,
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		form: ProtocolTerrainForm,
	) -> Self {
		Self {
			un_protocol: un_protocol.into(),
			tile: TileTerrain {
				texture_idx: 0,
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			form,
		}
	}
}

impl Protocol for ProtocolTerrain {
	fn pregen_chunk(
		&self,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		chunk_position_abs: &ChunkPositionAbs,
	) {
		match &self.form {
			ProtocolTerrainForm::Flat(maps) => {
				for (
					height,
					valid,
				) in maps.iter() {
					pregen_chunk_flat(
						intermediate_chunk,
						chunk_position_abs,
						&self,
						*height,
						*valid,
					);
				}
			},
			_ => {},
		}
	}

	fn pregen_tile(
		&self,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		chunk_position_abs: &ChunkPositionAbs,
		tile_position_abs: &TilePositionAbs,
		tile_position_rel: &TilePositionRel,
	) {
		match &self.form {
			ProtocolTerrainForm::Noise(maps) => {
				for (
					height_map,
					valid_map,
				) in maps.maps.iter() {
					pregen_tile_noise(
						intermediate_chunk,
						chunk_position_abs,
						tile_position_abs,
						tile_position_rel,
						height_map,
						valid_map,
						&self,
					);
				}
			},
			_ => {},
		}
	}
}

fn pregen_chunk_flat(
	mut intermediate_chunk: RawPtr<IntermediateChunk>,
	chunk_position_abs: &ChunkPositionAbs,
	protocol: &ProtocolTerrain,
	height: u8,
	est: f32,
) {
	intermediate_chunk.get_mut().subsurface.flat.add_form(
		SubsurfaceConfig {
			height,
		},
		ConfigFlat {
			est,
		},
		protocol.tile,
	);
}

fn pregen_tile_noise(
	mut intermediate_chunk: RawPtr<IntermediateChunk>,
	chunk_position_abs: &ChunkPositionAbs,
	tile_position_abs: &TilePositionAbs,
	tile_position_rel: &TilePositionRel,
	map_height: &Arc<dyn NoiseProxy + Send + Sync>,
	map_valid: &Arc<dyn NoiseProxy + Send + Sync>,
	protocol: &ProtocolTerrain,
) {
	let height_sample = map_height.get(chunk_position_abs, &tile_position_rel) as f32;
	let valid_sample = map_valid.get(chunk_position_abs, &tile_position_rel) as f32;

	let height = (op_11_03(height_sample) + 0.001).round() as u8;
	if height >= 3 { return; }
	// TODO: modifiers (ClimateSunlight etc.)
	let est = op_11_01(valid_sample);
	intermediate_chunk.get_mut().subsurface.noise.add_form(
		SubsurfaceConfig {
			height,
		},
		ConfigNoise {
			est,
			rel: *tile_position_rel,
		},
		protocol.tile,
	);
}