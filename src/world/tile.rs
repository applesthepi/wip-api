mod terrain;
use nalgebra::{Vector2, vector};
pub use terrain::*;
mod floor;
pub use floor::*;
mod building;
pub use building::*;
mod roof;
pub use roof::*;
mod structure;
pub use structure::*;
mod item;
pub use item::*;
mod covor;
pub use covor::*;

use crate::{PT_MOD_WCOUNT, RTTerrain, RTFloor, RTRoof, RTStructure, RTItem, RTCover, RTBuilding};

// TODO: covor (like filth/rubble etc.)
pub struct WorldTile {
	pub terrain: [Option<RTTerrain>; 4],
	pub floor: Option<RTFloor>,
	pub roof: Option<RTRoof>,
	pub building: [Option<RTBuilding>; 16],
	pub structure: Option<RTStructure>,
	pub cover: [Option<RTCover>; 16],
	pub item: Option<RTItem>,
}

impl Default for WorldTile {
	fn default() -> Self {
		Self {
			terrain: [(); 4].map(|_| None),
			floor: None,
			roof: None,
			building: [(); 16].map(|_| None),
			structure: None,
			cover: [(); 16].map(|_| None),
			item: None,
		}
	}
}

impl WorldTile {
	pub fn get_high_terrain(
		&self,
	) -> Option<usize> {
		for (i, x) in self.terrain.iter().enumerate() {
			if x.is_none() {
				if i == 0 {
					return None;
				}
				return Some(i - 1);
			}
		}
		Some(self.terrain.len() - 1)
	}

	pub fn set_high_cover(
		&mut self,
		rt: Option<RTCover>,
	) -> bool {
		for x in self.cover.iter_mut() {
			if let None = x {
				*x = rt;
				return true;
			}
		}
		false
	}
}