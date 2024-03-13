use crate::{EffectiveMaterial, TCHardness, Tile};
use crate::prelude::DOSize;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AmmoType {
	#[default]
	Cal50,
	Cal50APIT,
}

impl AmmoType {
	/// Perforation of steel in centimeters is used as the base value, other materials
	/// are derived from this. Returns the remaining perforation distance available for
	/// this bullet. A negative value means the bullet was consumed. The second value
	/// is between [0, 1] determining the damage percent of this simulation.
	pub fn simulate_perforation(
		&self,
		effective_material: &EffectiveMaterial,
		material_health: f32,
		bullet_speed_factor: f32,
	) -> (f32, f32) {
		effective_material.simulate_perforation(bullet_speed_factor * match self {
			Self::Cal50 => 1.8,
			Self::Cal50APIT => 2.25,
		}, material_health)
	}
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct ArmsAimStats {
	/// Furthest lock-on range of this weapon, will only fire if its a decent shot.
	pub max_target_range: i32,
	/// Rounds-per-minute.
	pub rpm: i32,
	/// Maximum half-spread in radians.
	pub max_spread: f32,
	/// Determines the base stat of the bullet.
	pub compatible_ammo: [Option<AmmoType>; 2],
	/// (0, 1] - Effect of the speed of the bullet based on the weapon.
	pub bullet_speed_factor: f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BuildingType {
	/// Cable under the floor (ex: power conduit).
	#[default]
	Conduit,
	/// Floor material over terrain and conduits (ex: wood floor).
	Floor,
	/// Building that sits on top of the floor (ex: table, floor lamp, door).
	Table,
	/// Same as table but renders an aimable top by a colonist (ex: auto turret).
	Arms,
	/// Aimed graphic for `Arms` (ex: auto turret aim).
	/// Not for use in the physical world, just for world rendering purposes.
	ArmsAim(DOSize, ArmsAimStats),
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