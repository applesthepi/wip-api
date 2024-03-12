use crate::{TCHardness, Tile};
use crate::prelude::DOSize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BuildingType {
	/// Cable under the floor (ex: power conduit).
	Conduit,
	/// Floor material over terrain and conduits (ex: wood floor).
	Floor,
	/// Building that sits on top of the floor (ex: table, floor lamp, door).
	Table,
	/// Same as table but renders an aimable top by a colonist (ex: auto turret).
	Arms,
	/// Aimed graphic for `Arms` (ex: auto turret aim).
	/// Not for use in the physical world, just for world rendering purposes.
	ArmsAim(DOSize),
	/// Wall mounted on the "edge" of a structure tile (ex: wall lamp).
	Mounted,
	/// Transparent buildings mounted to the roof (ex: fire sprinkler).
	Ceiling,
	/// On top of valid terrain OR if another building in the tile validates
	/// plants (ex: flower & `BuildingType::Table` pot).
	Plant,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileBuilding {
	pub texture_idx: u32,
	pub tc_hardness: TCHardness,
	pub building_type: BuildingType,
	/// Only 1 of this building type is allowed per-tile. Building
	/// types `Table` & `Arms` are both counted as `Table`.
	pub force_solo: bool,
	pub work: u32,
}

impl Tile for TileBuilding {}

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