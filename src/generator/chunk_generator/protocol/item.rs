use std::str::FromStr;

use crate::{TileItem, noise::ProtocolNoise, ICState, RTItem};

#[derive(Clone)]
pub struct ProtocolItem {
	pub name: Option<String>,
	pub tile: TileItem,
	pub noise: ProtocolNoise,
}

impl ProtocolItem {
	pub fn new(
		name: &str,
		ic_state: ICState,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileItem {
				texture_idx: 0,
				ic_state,
			},
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTItem> {
		Some(RTItem::new(self.tile.clone()))
	}
}