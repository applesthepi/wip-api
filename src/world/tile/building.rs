use crate::TCHardness;

#[repr(C)]
#[derive(Clone)]
pub struct TileBuilding {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub work: u32,
}

impl Default for TileBuilding {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			tc_hardness: TCHardness::Solid,
			work: 1,
		}
	}
}