use crate::{TileItem, RTTile};

#[derive(Clone, Copy)]
pub struct RTItem {
	pub tile: TileItem,
	pub selected: bool,
}

impl RTItem {
	pub fn new(
		tile: TileItem,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}

impl RTTile for RTItem {
	fn texture_idx(&self) -> u32 {
		self.tile.texture_idx
	}

	fn set(&mut self, texture_idx: u32) {
		self.tile.texture_idx = texture_idx;
	}
}