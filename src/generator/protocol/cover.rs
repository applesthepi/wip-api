use std::str::FromStr;

use crate::{ProtocolNoise2d, TileCover, RTCover};

#[derive(Clone)]
pub struct ProtocolCover {
	pub un_protocol: String,
	pub tile: TileCover,
	pub noise: ProtocolNoise2d,
}

impl ProtocolCover {
	pub fn new(
		un_protocol: impl Into<String>,
		tile_work: u32,
		noise: ProtocolNoise2d,
	) -> Self {
		Self {
			un_protocol: un_protocol.into(),
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