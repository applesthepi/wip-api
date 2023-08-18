use std::str::FromStr;

use crate::{noise::ProtocolNoise2d, TileCover, RTCover};

#[derive(Clone)]
pub struct ProtocolCover {
	pub name: Option<String>,
	pub tile: TileCover,
	pub noise: ProtocolNoise2d,
}

impl ProtocolCover {
	pub fn new(
		name: &str,
		tile_work: u32,
		noise: ProtocolNoise2d,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileCover {
				texture_idx: 0,
				work: tile_work,
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTCover> {
		Some(RTCover::new(self.tile.clone()))
	}
}