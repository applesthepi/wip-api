use crate::{TCHardness, Tile, TileBuilding};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileStructure {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub work: u32,
}

impl Tile for TileStructure {}

impl Default for TileStructure {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			tc_hardness: TCHardness::Solid,
			work: 1,
		}
	}
}