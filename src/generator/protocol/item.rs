use std::str::FromStr;

use crate::{TileItem, ProtocolNoise2d, ICState, RTItem, DropTable};

#[derive(Clone)]
pub struct ProtocolItem {
	pub un_protocol: String,
	pub tile: TileItem,
}

impl ProtocolItem {
	pub fn new(
		un_protocol: impl Into<String>,
		ic_state: ICState,
	) -> Self {
		Self {
			un_protocol: un_protocol.into(),
			tile: TileItem {
				texture_idx: 0,
				ic_state,
			},
		}
	}

	pub fn instantiate(
		&self,
	) -> RTItem {
		RTItem::new(self.tile.clone())
	}
}