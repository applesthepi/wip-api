use std::str::FromStr;

use crate::{ProtocolNoise2d, TileRoof, RTRoof};

#[derive(Clone)]
pub struct ProtocolRoof {
	pub un_protocol: String,
	pub tile: TileRoof,
	pub noise: ProtocolNoise2d,
}

impl ProtocolRoof {
	pub fn new(
		un_protocol: impl Into<String>,
		tile_work: u32,
		noise: ProtocolNoise2d,
	) -> Self {
		Self {
			un_protocol: un_protocol.into(),
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