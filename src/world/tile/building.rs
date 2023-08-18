use crate::TCHardness;

#[derive(Clone)]
pub enum BuildingType {
	Conduit,
	Floor,
	Table,
	Mounted,
	Ceiling,
}

#[repr(C)]
#[derive(Clone)]
pub struct TileBuilding {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub building_type: BuildingType,
	pub force_solo: bool,
	pub work: u32,
}

impl Default for TileBuilding {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			tc_hardness: TCHardness::Solid,
			building_type: BuildingType::Conduit,
			force_solo: false,
			work: 1,
		}
	}
}