use std::str::FromStr;

use crate::{noise::ProtocolNoise, TileCover, RTCover};

#[derive(Clone)]
pub struct ProtocolCover {
	pub name: Option<String>,
	pub tile: TileCover,
	pub noise: ProtocolNoise,
}

impl ProtocolCover {
	pub fn new(
		name: &str,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileCover {
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