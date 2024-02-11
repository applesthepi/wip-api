use std::str::FromStr;

use crate::{TileItem, ProtocolNoise2d, ICState, RTItem, DropTable};

#[derive(Clone)]
pub struct ProtocolItem {
	pub name: Option<String>,
	pub tile: TileItem,
}

impl ProtocolItem {
	pub fn new(
		name: impl Into<String>,
		ic_state: ICState,
	) -> Self {
		Self {
			name: Some(name.into()),
			tile: TileItem {
				texture_idx: 0,
				ic_state,
			},
		}
	}

	// pub fn instantiate(
	// 	&self,
	// ) -> Option<RTItem> {
	// 	Some(RTItem::new(self.tile.clone()))
	// }
}