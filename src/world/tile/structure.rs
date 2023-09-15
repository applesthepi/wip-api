use crate::TCHardness;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileStructure {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub work: u32,
}

impl Default for TileStructure {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			tc_hardness: TCHardness::Solid,
			work: 1,
		}
	}
}