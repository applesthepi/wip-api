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
		tile_work: u32,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileRoof {
				texture_idx: 0,
				work: tile_work,
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