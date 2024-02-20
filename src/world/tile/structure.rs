use crate::{DropTable, TCHardness, Tile, TileBuilding};

#[repr(C)]
#[derive(Clone)]
pub struct TileStructure {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub work: u32,
	pub drop_table: Option<DropTable>,
}

impl Tile for TileStructure {}

impl Default for TileStructure {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			tc_hardness: TCHardness::Solid,
			work: 1,
			drop_table: None,
		}
	}
}