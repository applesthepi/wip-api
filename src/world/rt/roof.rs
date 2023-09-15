use crate::{TileRoof, RTTile};

#[derive(Clone, Copy)]
pub struct RTRoof {
	pub tile: TileRoof,
	pub selected: bool,
}

impl RTRoof {
	pub fn new(
		tile: TileRoof,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}

impl RTTile for RTRoof {
	fn texture_idx(&self) -> u32 {
		self.tile.texture_idx
	}

	fn set(&mut self, texture_idx: u32) {
		self.tile.texture_idx = texture_idx;
	}
}