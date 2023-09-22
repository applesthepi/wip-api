use crate::{TileStructure, RTTile};

#[derive(Clone, Copy)]
pub struct RTStructure {
	pub tile: TileStructure,
	pub selected: bool,
}

impl RTStructure {
	pub fn new(
		tile: TileStructure,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}

impl RTTile for RTStructure {
	// fn texture_idx(&self) -> u32 {
	// 	self.tile.texture_idx
	// }

	// fn set(&mut self, texture_idx: u32) {
	// 	self.tile.texture_idx = texture_idx;
	// }
}