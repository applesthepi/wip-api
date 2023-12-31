use bevy::prelude::warn;
use wip_primal::CHUNK_WIDTH;

use crate::{RTTerrain, RTRoof, RTStructure, RTItem, RTCover, RTBuilding};

mod terrain;
mod floor;
mod building;
mod roof;
mod structure;
mod item;
mod cover;

pub use terrain::*;
pub use floor::*;
pub use building::*;
pub use roof::*;
pub use structure::*;
pub use item::*;
pub use cover::*;

pub trait Tile {}

#[derive(Clone, Copy)]
pub enum Order {
	Mine,
}

impl Order {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			Order::Mine => "mine",
		}
	}
}

#[derive(Clone, Copy)]
pub struct RTOrder {
	pub order: Order,
}

impl RTTile for RTOrder {}

pub trait RTTile {
	// fn texture_idx(&self) -> u32;
	// fn set(&mut self, texture_idx: u32);
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

	pub fn slice<'get>(
		&'get self,
	) -> &'get [Option<Rt>; LEN] {
		&self.slice
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

	pub fn set_rt_height(
		&mut self,
		height: u8,
		rt: Rt,
	) {
		if height >= self.slice.len() as u8 {
			warn!("tile RT height out of bounds {} >= {}", height, self.slice.len());
			return;
		}
		self.slice[height as usize] = Some(rt);
	}

	pub fn set_rt_lowest(
		&mut self,
		rt: Rt,
	) {
		{
			let first = self.slice.first_mut().unwrap();
			if first.is_none() {
				*first = Some(rt);
				return;
			}
		}
		let mut swap_rt = self.slice[0].unwrap();
		self.slice[0] = Some(rt);
		let mut idx = 1;
		loop {
			if idx == self.slice.len() {
				warn!("no more space left, an RT will be discarded");
				return;
			}
			if self.slice[idx].is_none() {
				self.slice[idx] = Some(swap_rt);
				return;
			}
			let nswap = self.slice[idx].unwrap();
			self.slice[idx] = Some(swap_rt);
			swap_rt = nswap;

		}
	}

	/// Condenses the slice down starting at 0. Does not gurantee that
	/// there are any cells, only that if there are cells, they are
	/// condensed down starting at 0.
	pub fn condense(
		&mut self,
	) {
		debug_assert!(self.slice.len() > 0);
		if self.slice.len() <= 1 { warn!("attempted to condense slice of length 1"); return; }
		let mut last_rt_idx = 0;
		let mut last_rt_set = false;

		// Find a starting point for condensing to start.
		for (i, rt) in self.slice.iter().enumerate() {
			if rt.is_some() { continue; }
			last_rt_idx = i;
			last_rt_set = true;
			break;
		}

		// No condensing needs to be done. Either all slice cells are
		// full, or only the last one is `None` which is condensed fully.
		if !last_rt_set || (last_rt_idx + 1) == self.slice.len() {
			return;
		}

		// `i` begins at the first `None`. Finds all future `Some`s and brings
		// them back to `last_rt_idx`.
		for i in (last_rt_idx + 1)..self.slice.len() {
			if self.slice[i].as_ref().is_none() { continue; }

			self.slice[last_rt_idx] = self.slice[i];
			self.slice[i] = None;
			last_rt_idx += 1;
		}
	}

	pub fn contains_some(
		&self,
	) -> bool {
		for rt in self.slice.iter() {
			if rt.is_some() {
				return true;
			}
		}
		false
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
	pub order: RTSlice<RTOrder, 8>,
}

impl WorldTile {
	// pub fn new(
	// 	tile_subsurface: TileSubsurface,
	// 	tile_surface: TileSurface,
	// 	tile_topical: TileTopical,
	// ) -> Self {
	// 	let mut world_tile = Self {
	// 		terrain: RTSlice::from_combination(
	// 			tile_subsurface.terrain,
	// 			tile_surface.terrain,
	// 		),
	// 		item: tile_topical.item,
	// 		building: tile_topical.building,
	// 		structure: tile_topical.structure,
	// 		roof: tile_topical.roof,
	// 		cover: tile_topical.cover,
	// 	};
	// 	world_tile.terrain.condense();
	// 	world_tile.building.condense();
	// 	world_tile.cover.condense();
	// 	world_tile
	// }
}