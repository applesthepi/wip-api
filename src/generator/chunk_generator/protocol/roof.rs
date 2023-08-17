use std::str::FromStr;

use crate::{noise::ProtocolNoise, TileRoof, RTRoof};

#[derive(Clone)]
pub struct ProtocolRoof {
	pub name: Option<String>,
	pub tile: TileRoof,
	pub noise: ProtocolNoise,
}

impl ProtocolRoof {
	pub fn new(
		name: &str,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileRoof {
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTRoof> {
		Some(RTRoof::new(self.tile.clone()))
	}
}