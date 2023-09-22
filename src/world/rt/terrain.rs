use crate::{TileTerrain, RTTile};

#[derive(Clone, Copy)]
pub struct RTTerrain {
	pub tile: TileTerrain,
	pub selected: bool,
}

impl RTTerrain {
	pub fn new(
		tile: TileTerrain,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}

impl RTTile for RTTerrain {
	// fn texture_idx(&self) -> u32 {
	// 	self.tile.texture_idx
	// }

	// fn set(&mut self, texture_idx: u32) {
	// 	self.tile.texture_idx = texture_idx;
	// }
}