use std::str::FromStr;

use crate::{noise::ProtocolNoise2d, TileStructure, RTStructure, TCHardness};

#[derive(Clone)]
pub struct ProtocolStructure {
	pub name: Option<String>,
	pub tile: TileStructure,
	pub noise: ProtocolNoise2d,
}

impl ProtocolStructure {
	pub fn new(
		name: &str,
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		noise: ProtocolNoise2d,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileStructure {
				texture_idx: 0,
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTStructure> {
		Some(RTStructure::new(self.tile.clone()))
	}
}