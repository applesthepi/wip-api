mod terrain;
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

// TODO: covor (like filth/rubble etc.)
pub struct WorldTile {
	pub terrain: [TileTerrain; 4],
	pub floor: TileFloor,
	pub roof: TileRoof,
	pub building: [TileBuilding; 16],
	pub structure: TileStructure,
}

impl Default for WorldTile {
	fn default() -> Self {
		Self {
			terrain: [(); 4].map(|_| TileTerrain::default()),
			floor: TileFloor::default(),
			roof: TileRoof::default(),
			building: [TileBuilding::default(); 16],
			structure: TileStructure::default(),
		}
	}
}