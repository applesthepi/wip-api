use std::str::FromStr;

use crate::{TileBuilding, ProtocolNoise2d, TCHardness, BuildingType, RTBuilding};

#[derive(Clone)]
pub struct ProtocolBuilding {
	pub name: Option<String>,
	pub tile: TileBuilding,
	pub noise: ProtocolNoise2d,
}

impl ProtocolBuilding {
	pub fn new(
		name: &str,
		tc_hardness: TCHardness,
		building_type: BuildingType,
		force_solo: bool,
		work: u32,
		noise: ProtocolNoise2d,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileBuilding {
				texture_idx: 0,
				tc_hardness,
				building_type,
				force_solo,
				work,
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