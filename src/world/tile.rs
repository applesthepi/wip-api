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

use crate::PT_MOD_WCOUNT;

#[derive(Debug, Clone, Copy)]
pub struct TilePosition {
	pub chunk_coordinate: Vector2<i32>,
	pub tile_coordinate: Vector2<i32>,
}

impl TilePosition {
	pub fn new(
		chunk_coordinate: Vector2<i32>,
		tile_coordinate: Vector2<i32>,
	) -> Self {
		Self {
			chunk_coordinate,
			tile_coordinate,
		}
	}

	pub fn offset_tile(
		mut self,
		tile_offset: Vector2<i32>,
	) -> Self {
		self.tile_coordinate += tile_offset;
		let chunk_offset: Vector2<i32> = vector![
			self.tile_coordinate.x / (PT_MOD_WCOUNT as i32),
			self.tile_coordinate.y / (PT_MOD_WCOUNT as i32)
		];
		self.tile_coordinate = vector![
			self.tile_coordinate.x.rem_euclid(PT_MOD_WCOUNT as i32),
			self.tile_coordinate.y.rem_euclid(PT_MOD_WCOUNT as i32)
		];
		self
	}

	pub fn get_linear_tile_position(
		&self,
	) -> usize {
		debug_assert!(self.tile_coordinate.x >= 0);
		debug_assert!(self.tile_coordinate.y >= 0);
		(self.tile_coordinate.y as usize * PT_MOD_WCOUNT) + self.tile_coordinate.x as usize
	}
}

// TODO: covor (like filth/rubble etc.)
pub struct WorldTile {
	pub terrain: [TileTerrain; 4],
	pub floor: TileFloor,
	pub roof: TileRoof,
	pub building: [TileBuilding; 16],
	pub structure: TileStructure,
	pub covor: [TileCovor; 16],
	pub item: TileItem,
}

impl Default for WorldTile {
	fn default() -> Self {
		Self {
			terrain: [(); 4].map(|_| TileTerrain::default()),
			floor: TileFloor::default(),
			roof: TileRoof::default(),
			building: [TileBuilding::default(); 16],
			structure: TileStructure::default(),
			covor: [TileCovor::default(); 16],
			item: TileItem::default(),
		}
	}
}

impl WorldTile {
	pub fn get_high_terrain(
		&self,
	) -> usize {
		for (i, x) in self.terrain.iter().enumerate() {
			if x.mod_terrain.is_none() {
				debug_assert!(i > 0);
				return i - 1;
			}
		}
		self.terrain.len() - 1
	}
}