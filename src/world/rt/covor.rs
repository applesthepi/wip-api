use crate::{TileCover, RTTile};

#[derive(Clone, Copy)]
pub struct RTCover {
	pub tile: TileCover,
	pub selected: bool,
}

impl RTCover {
	pub fn new(
		tile: TileCover,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}

impl RTTile for RTCover {
	// fn texture_idx(&self) -> u32 {
	// 	self.tile.texture_idx
	// }

	// fn set(&mut self, texture_idx: u32) {
	// 	self.tile.texture_idx = texture_idx;
	// }
}