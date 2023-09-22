use std::{str::FromStr, sync::Arc};

use wip_primal::{TilePositionRel, TilePositionAbs, ChunkPositionAbs, Bounds};

use crate::{RTTerrain, TileTerrain, TCHardness, ProtocolTerrainForm, ProtocolNoise3d, Protocol, IntermediateChunk, RawPtr, NoiseProxy, Gen, SubsurfaceConfig, prelude::{op_11_03, op_11_01}, ConfigFlat};

#[derive(Clone)]
pub struct ProtocolTerrain {
	pub name: Option<String>,
	pub tile: TileTerrain,
	/// Form of terrain has additional configuration.
	pub form: ProtocolTerrainForm,
	pub noise: ProtocolNoise3d,
}

impl ProtocolTerrain {
	pub fn new(
		name: &str,
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		noise: ProtocolNoise3d,
		form: ProtocolTerrainForm,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileTerrain {
				texture_idx: 0,
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			form,
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTTerrain> {
		Some(RTTerrain::new(self.tile.clone()))
	}
}

impl Protocol for ProtocolTerrain {
	fn pregen(
		&self,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		chunk_position_abs: &ChunkPositionAbs,
		tile_position_abs: &TilePositionAbs,
		tile_position_rel: &TilePositionRel,
	) {
		for (
			map_height,
			map_valid,
		) in self.noise.maps.iter() {
			pregen(
				intermediate_chunk,
				chunk_position_abs,
				tile_position_abs,
				tile_position_rel,
				map_height,
				map_valid,
				self.tile,
			);
		}
	}
}

fn pregen(
	mut intermediate_chunk: RawPtr<IntermediateChunk>,
	chunk_position_abs: &ChunkPositionAbs,
	tile_position_abs: &TilePositionAbs,
	tile_position_rel: &TilePositionRel,
	map_height: &Arc<dyn NoiseProxy + Send + Sync>,
	map_valid: &Arc<dyn NoiseProxy + Send + Sync>,
	tile: TileTerrain,
) {
	let height_sample = map_height.get(chunk_position_abs, &tile_position_rel) as f32;
	let valid_sample = map_valid.get(chunk_position_abs, &tile_position_rel) as f32;
	// let patch_gen_data = PatchGenData {
	// 	origin: *tile_position_abs,
	// 	height_sample,
	// 	valid_sample: map_valid.get(chunk_position_abs, &tile_position_rel) as f32,
	// 	applied_bounds: Bounds::from_origin(*tile_position_abs, TilePositionAbs::splat(0)),
	// };
	let height = op_11_03(height_sample) as u8;
	if height >= 3 { return; }
	// TODO: modifiers (ClimateSunlight etc.)
	let est = op_11_01(valid_sample);
	intermediate_chunk.get().subsurface.flat.add_form(
		SubsurfaceConfig {
			height,
		},
		ConfigFlat {
			est,
		},
		tile,
	);
}