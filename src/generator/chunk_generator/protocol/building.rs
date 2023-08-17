use std::str::FromStr;

use crate::{TileBuilding, noise::ProtocolNoise, RTBuilding};

#[derive(Clone)]
pub struct ProtocolBuilding {
	pub name: Option<String>,
	pub tile: TileBuilding,
	pub noise: ProtocolNoise,
}

impl ProtocolBuilding {
	pub fn new(
		name: &str,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileBuilding {
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