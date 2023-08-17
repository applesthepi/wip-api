use std::str::FromStr;

use crate::{noise::ProtocolNoise, TileCovor, RTCovor};

#[derive(Clone)]
pub struct ProtocolCovor {
	pub name: Option<String>,
	pub tile: TileCovor,
	pub noise: ProtocolNoise,
}

impl ProtocolCovor {
	pub fn new(
		name: &str,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileCovor {
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTCovor> {
		Some(RTCovor::new(self.tile.clone()))
	}
}