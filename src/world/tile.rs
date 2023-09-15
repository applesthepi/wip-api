use wip_primal::CHUNK_WIDTH;
use wip_primal::TilePositionRel;

use crate::{RTTerrain, RTRoof, RTStructure, RTItem, RTCover, RTBuilding};

mod terrain;
mod floor;
mod building;
mod roof;
mod structure;
mod item;
mod cover;

pub use self::terrain::*;
pub use self::floor::*;
pub use self::building::*;
pub use self::roof::*;
pub use self::structure::*;
pub use self::item::*;
pub use self::cover::*;

pub trait RTTile {
	fn texture_idx(&self) -> u32;
	fn set(&mut self, texture_idx: u32);
}

/// Slice for a tile. Some tile types can have multiple of itself (4 levels of terrain per tile).
#[derive(Clone, Copy)]
pub struct RTSlice<Rt: RTTile + Clone + Copy, const LEN: usize> {
	slice: [Option<Rt>; LEN],
}

impl<Rt: RTTile + Clone + Copy, const LEN: usize> Default for RTSlice<Rt, LEN> {
	fn default() -> Self {
		Self {
			slice: [None; LEN],
		}
	}
}

impl<Rt: RTTile + Clone + Copy, const LEN: usize> RTSlice<Rt, LEN> {
	/// Retrives the highest `RTTile` in the slice.
	pub fn get_high_rt<'get>(
		&'get self,
	) -> Option<&'get Rt> {
		for (i, x) in self.slice.iter().enumerate() {
			if x.is_none() {
				if i == 0 {
					return None;
				}
				// Return the last successful access.
				return Some(self.slice[i - 1].as_ref().unwrap());
			}
		}
		Some(self.slice.last().unwrap().as_ref().unwrap())
	}

	pub fn from_combination<const LEN1: usize, const LEN2: usize>(
		rt_slice_1: RTSlice<Rt, LEN1>,
		rt_slice_2: RTSlice<Rt, LEN2>,
	) -> Self {
		let slice = {
			let mut rt_slice: RTSlice<Rt, LEN> = RTSlice::<Rt, LEN>::default();
			let (
				a,
				b,
			) = rt_slice.slice.split_at_mut(LEN1);
			a.clone_from_slice(&rt_slice_1.slice);
			b.clone_from_slice(&rt_slice_2.slice);
			rt_slice
		};
		slice
	}

	pub fn set_rt(
		&mut self,
		rt: Rt,
	) {
		for local_rt in self.slice.iter_mut() {
			if local_rt.is_none() {
				*local_rt = Some(rt);
			}
		}
	}
}

/// Sub surface map of tiles; restricted depth of 3/4 (gurantees surface has atleast one).
#[derive(Default, Clone, Copy)]
pub struct TileSubsurface {
	pub terrain: RTSlice<RTTerrain, 3>,
}

/// Map for a whole chunk of `TileSubsurface`. Used when mapping to tiles for the `PhysicalChunk`.
#[derive(Default, Clone, Copy)]
pub struct ChunkSubsurface {
	pub tiles: [[TileSubsurface; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
}

/// Flat map of surface tiles that will drop down elevation.
#[derive(Default, Clone, Copy)]
pub struct TileSurface {
	pub terrain: RTSlice<RTTerrain, 1>,
}

/// Map for a whole chunk of `TileSurface`. Used when mapping to tiles for the `PhysicalChunk`.
#[derive(Default, Clone, Copy)]
pub struct ChunkSurface {
	pub tiles: [[TileSurface; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
}

/// Topical map of features (including items and buildings).
#[derive(Default, Clone, Copy)]
pub struct TileTopical {
	pub item: RTSlice<RTItem, 1>,
	pub building: RTSlice<RTBuilding, 16>,
	pub structure: RTSlice<RTStructure, 1>,
	pub roof: RTSlice<RTRoof, 1>,
	pub cover: RTSlice<RTCover, 16>,
}

/// Map for a whole chunk of `TileTopical`. Used when mapping to tiles for the `PhysicalChunk`.
#[derive(Default, Clone, Copy)]
pub struct ChunkTopical {
	pub tiles: [[TileTopical; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
}

/// Operation result for `WorldTile`.
pub enum WTOperation {
	Success,
	ObjectInWay,
	OutOfSpace,
}

/// Tile for `PhysicalChunk`. All tile type specific operations are done
/// on the `RTSlice`s.
#[derive(Default, Copy, Clone)]
pub struct WorldTile {
	pub terrain: RTSlice<RTTerrain, 4>,
	pub item: RTSlice<RTItem, 1>,
	pub building: RTSlice<RTBuilding, 16>,
	pub structure: RTSlice<RTStructure, 1>,
	pub roof: RTSlice<RTRoof, 1>,
	pub cover: RTSlice<RTCover, 16>,
}

impl WorldTile {
	pub fn new(
		tile_subsurface: TileSubsurface,
		tile_surface: TileSurface,
		tile_topical: TileTopical,
	) -> Self {
		Self {
			terrain: RTSlice::from_combination(
				tile_subsurface.terrain,
				tile_surface.terrain,
			),
			item: tile_topical.item,
			building: tile_topical.building,
			structure: tile_topical.structure,
			roof: tile_topical.roof,
			cover: tile_topical.cover,
		}
	}
}