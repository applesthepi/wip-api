use std::str::FromStr;

use crate::{noise::ProtocolNoise, TileStructure, RTStructure};

#[derive(Clone)]
pub struct ProtocolStructure {
	pub name: Option<String>,
	pub tile: TileStructure,
	pub noise: ProtocolNoise,
}

impl ProtocolStructure {
	pub fn new(
		name: &str,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileStructure {
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTStructure> {
		Some(RTStructure::new(self.tile.clone()))
	}
}