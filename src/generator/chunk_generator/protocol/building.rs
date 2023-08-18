use std::str::FromStr;

use crate::{TileBuilding, noise::ProtocolNoise2d, RTBuilding, TCHardness};

#[derive(Clone)]
pub struct ProtocolBuilding {
	pub name: Option<String>,
	pub tile: TileBuilding,
	pub noise: ProtocolNoise2d,
}

impl ProtocolBuilding {
	pub fn new(
		name: &str,
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		noise: ProtocolNoise2d,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileBuilding {
				texture_idx: 0,
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTBuilding> {
		Some(RTBuilding::new(self.tile.clone()))
	}
}