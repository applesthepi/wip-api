use crate::{TileBuilding, RTTile};

#[derive(Clone, Copy)]
pub struct RTBuilding {
	pub tile: TileBuilding,
	pub selected: bool,
}

impl RTBuilding {
	pub fn new(
		tile: TileBuilding,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}

impl RTTile for RTBuilding {
	fn texture_idx(&self) -> u32 {
		self.tile.texture_idx
	}

	fn set(&mut self, texture_idx: u32) {
		self.tile.texture_idx = texture_idx;
	}
}