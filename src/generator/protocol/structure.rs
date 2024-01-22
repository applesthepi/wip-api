use std::str::FromStr;
use std::sync::Arc;
use wip_primal::{ChunkPositionAbs, TilePositionAbs, TilePositionRel};

use crate::{ProtocolNoise2d, TileStructure, RTStructure, TCHardness, ProtocolStructureForm, Protocol, ProtocolBuilding, IntermediateChunk, RawPtr, ProtocolBuildingForm, NoiseProxy, VegitationConfig, ConfigNoise, Mountain, MountainConfig, Gen};
use crate::prelude::op_11_01;

#[derive(Clone)]
pub struct ProtocolStructure {
	pub name: Option<String>,
	pub tile: TileStructure,
	pub form: ProtocolStructureForm,
}

impl ProtocolStructure {
	pub fn new(
		name: &str,
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		form: ProtocolStructureForm,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileStructure {
				texture_idx: 0,
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			form,
		}
	}
}

impl Protocol for ProtocolStructure {
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
			ProtocolStructureForm::Noise(maps) => {
				for valid_map
				in maps.maps.iter() {
					pregen_tile_structure(
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

fn pregen_tile_structure(
	mut intermediate_chunk: RawPtr<IntermediateChunk>,
	chunk_position_abs: &ChunkPositionAbs,
	tile_position_abs: &TilePositionAbs,
	tile_position_rel: &TilePositionRel,
	valid_map: &Arc<dyn NoiseProxy + Send + Sync>,
	protocol: &ProtocolStructure,
) {
	let valid_sample = valid_map.get(chunk_position_abs, &tile_position_rel) as f32;

	let est = op_11_01(valid_sample);
	if est < 0.5 { return; }
	intermediate_chunk.get_mut().mountain.noise.add_form(
		MountainConfig {
		},
		ConfigNoise {
			est,
			rel: *tile_position_rel,
		},
		protocol.tile,
	);
}