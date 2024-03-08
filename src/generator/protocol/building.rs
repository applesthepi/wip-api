use std::{str::FromStr, sync::Arc};

use wip_primal::{ChunkPositionAbs, TilePositionAbs, TilePositionRel};

use crate::{TileBuilding, ProtocolNoise2d, TCHardness, BuildingType, RTBuilding, Protocol, IntermediateChunk, RawPtr, ProtocolBuildingForm, NoiseProxy, prelude::op_11_01, Gen, VegetationConfig, ConfigNoise, RTItem};

#[derive(Clone)]
pub struct ProtocolBuilding {
	pub un_protocol: String,
	pub tile: TileBuilding,
	pub form: ProtocolBuildingForm,
}

impl ProtocolBuilding {
	pub fn new(
		un_protocol: impl Into<String>,
		tc_hardness: TCHardness,
		building_type: BuildingType,
		force_solo: bool,
		work: u32,
		form: ProtocolBuildingForm,
	) -> Self {
		Self {
			un_protocol: un_protocol.into(),
			tile: TileBuilding {
				texture_idx: 0,
				tc_hardness,
				building_type,
				force_solo,
				work,
			},
			form,
		}
	}

	pub fn instantiate(
		&self,
	) -> RTBuilding {
		RTBuilding::new(self.tile.clone())
	}
}

impl Protocol for ProtocolBuilding {
	fn pregen_chunk(
		&self,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		chunk_position_abs: &ChunkPositionAbs,
	) {
		match &self.form {
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
			ProtocolBuildingForm::Vegetation(maps) => {
				for valid_map
				in maps.maps.iter() {
					pregen_tile_vegitation(
						intermediate_chunk,
						chunk_position_abs,
						tile_position_abs,
						tile_position_rel,
						valid_map,
						&self,
					);
				}
			},
			_ => {},
		}
	}
}

fn pregen_tile_vegitation(
	mut intermediate_chunk: RawPtr<IntermediateChunk>,
	chunk_position_abs: &ChunkPositionAbs,
	tile_position_abs: &TilePositionAbs,
	tile_position_rel: &TilePositionRel,
	valid_map: &Arc<dyn NoiseProxy + Send + Sync>,
	protocol: &ProtocolBuilding,
) {
	let valid_sample = valid_map.get(chunk_position_abs, &tile_position_rel) as f32;

	// TODO: modifiers (ClimateSunlight etc.)
	let est = op_11_01(valid_sample);
	if est < 0.5 { return; }
	intermediate_chunk.get_mut().vegetation.noise.add_form(
		VegetationConfig {
		},
		ConfigNoise {
			est,
			rel: *tile_position_rel,
		},
		protocol.tile,
	);
}