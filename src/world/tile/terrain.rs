use crate::Tile;

#[derive(Clone, Copy)]
pub enum TCHardness {
	Solid,
	Soft,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileTerrain {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub work: u32,
}

impl Tile for TileTerrain {}

impl Default for TileTerrain {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			tc_hardness: TCHardness::Solid,
			work: 1,
		}
	}
}