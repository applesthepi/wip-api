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

pub enum WTOperation {
	Success,
	ObjectInWay,
	OutOfSpace,
}

// TODO: covor (like filth/rubble etc.)
pub struct WorldTile {
	terrain: [Option<RTTerrain>; 4],
	item: Option<RTItem>,
	building: [Option<RTBuilding>; 16],
	structure: Option<RTStructure>,
	roof: Option<RTRoof>,
	cover: [Option<RTCover>; 16],
}

impl Default for WorldTile {
	fn default() -> Self {
		Self {
			terrain: [(); 4].map(|_| None),
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
	) -> Option<RTTerrain> {
		for (i, x) in self.terrain.iter().enumerate() {
			if x.is_none() {
				if i == 0 {
					return None;
				}
				return self.terrain[i - 1].clone();
			}
		}
		self.terrain.last().unwrap().clone()
	}

	pub fn get_item(
		&self,
	) -> Option<RTItem> {
		self.item.clone()
	}

	pub fn get_buildings(
		&self,
	) -> [Option<RTBuilding>; 16] {
		self.building.clone()
	}

	pub fn get_structure(
		&self,
	) -> Option<RTStructure> {
		self.structure.clone()
	}

	pub fn get_roof(
		&self,
	) -> Option<RTRoof> {
		self.roof.clone()
	}

	pub fn get_cover(
		&self,
	) -> [Option<RTCover>; 16] {
		self.cover.clone()
	}

	pub fn set_terrain(
		&mut self,
		rt: Option<RTTerrain>,
		height: u8,
	) {
		assert!(height < self.terrain.len() as u8);
		self.terrain[height as usize] = rt;
	}

	pub fn set_item(
		&mut self,
		rt: Option<RTItem>,
	) -> WTOperation {
		if let Some(_) = &self.structure {
			return WTOperation::ObjectInWay;
		}
		self.item = rt;
		WTOperation::Success
	}

	pub fn set_high_building(
		&mut self,
		rt: Option<RTBuilding>,
	) -> WTOperation {
		if let Some(_) = &self.structure {
			return WTOperation::ObjectInWay;
		}
		for x in self.building.iter_mut() {
			if let None = x {
				*x = rt;
				return WTOperation::Success;
			}
		}
		WTOperation::OutOfSpace
	}

	pub fn set_structure(
		&mut self,
		rt: Option<RTStructure>,
	) -> WTOperation {
		if let Some(_) = &self.item {
			return WTOperation::ObjectInWay;
		}
		for building in self.building.iter() {
			if let Some(_) = &building {
				return WTOperation::ObjectInWay;
			}
		}
		for cover in self.cover.iter_mut() {
			*cover = None;
		}
		self.structure = rt;
		WTOperation::Success
	}

	pub fn set_roof(
		&mut self,
		rt: Option<RTRoof>,
	) -> WTOperation {
		self.roof = rt;
		WTOperation::Success
	}

	pub fn set_high_cover(
		&mut self,
		rt: Option<RTCover>,
	) -> WTOperation {
		if let Some(_) = &self.structure {
			return WTOperation::ObjectInWay;
		}
		for x in self.cover.iter_mut() {
			if let None = x {
				*x = rt;
				return WTOperation::Success;
			}
		}
		WTOperation::OutOfSpace
	}
}