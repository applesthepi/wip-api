use bevy::prelude::{Entity, warn};
use wip_primal::CHUNK_WIDTH;

use crate::{RTTerrain, RTRoof, RTStructure, RTItem, RTCover, RTBuilding, Task, PhysicalOrder, Registry};

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

/// Slice for a tile. Some tile types can have multiple of itself (4 levels of terrain per tile).
#[derive(Clone)]
pub struct RTSlice<Rt: Clone, const LEN: usize> {
	slice: [Option<Rt>; LEN],
}

impl<Rt: Clone, const LEN: usize> Default for RTSlice<Rt, LEN> {
	fn default() -> Self {
		Self {
			slice: [(); LEN].map(|_| None),
		}
	}
}

impl<Rt: Clone, const LEN: usize> RTSlice<Rt, LEN> {
	/// Retrives the highest `RTTile` in the slice.
	pub fn get_high_rt(
		&self,
	) -> Option<&Rt> {
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

	pub fn slice(
		&self,
	) -> &[Option<Rt>; LEN] {
		&self.slice
	}

	pub fn slice_mut(
		&mut self,
	) -> &mut [Option<Rt>; LEN] {
		&mut self.slice
	}

	pub fn single(
		&self,
	) -> &Option<Rt> {
		let Some(first) = self.slice.first() else {
			panic!("single called on empty slice");
		};
		first
	}

	pub fn single_mut(
		&mut self,
	) -> &mut Option<Rt> {
		let Some(first) = self.slice.first_mut() else {
			panic!("single_mut called on empty slice");
		};
		first
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

	pub fn remove_rt_height(
		&mut self,
		height: u8,
	) {
		if height >= self.slice.len() as u8 {
			warn!("tile RT height out of bounds {} >= {}", height, self.slice.len());
			return;
		}
		self.slice[height as usize] = None;
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

	pub fn set_rt_quick(
		&mut self,
		rt: Rt,
	) {
		for cell in self.slice.iter_mut() {
			if cell.is_some() { continue; }
			*cell = Some(rt);
			return;
		}
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
		let mut swap_rt = self.slice[0].as_ref().unwrap().clone();
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
			let nswap = self.slice[idx].as_ref().unwrap().clone();
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

			self.slice[last_rt_idx] = self.slice[i].clone();
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

	pub fn clear(
		&mut self,
	) {
		for rt in self.slice.iter_mut() {
			*rt = None;
		}
	}
}

/// Sub surface map of tiles; restricted depth of 3/4 (guarantees surface has atleast one).
#[derive(Default, Clone)]
pub struct TileSubsurface {
	pub terrain: RTSlice<RTTerrain, 3>,
}

/// Map for a whole chunk of `TileSubsurface`. Used when mapping to tiles for the `PhysicalChunk`.
#[derive(Default, Clone)]
pub struct ChunkSubsurface {
	pub tiles: [[TileSubsurface; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
}

/// Flat map of surface tiles that will drop down elevation.
#[derive(Default, Clone)]
pub struct TileSurface {
	pub terrain: RTSlice<RTTerrain, 1>,
}

/// Map for a whole chunk of `TileSurface`. Used when mapping to tiles for the `PhysicalChunk`.
#[derive(Default, Clone)]
pub struct ChunkSurface {
	pub tiles: [[TileSurface; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
}

/// Topical map of features (including items and buildings).
#[derive(Default, Clone)]
pub struct TileTopical {
	pub item: RTSlice<RTItem, 1>,
	pub building: RTSlice<RTBuilding, 16>,
	pub structure: RTSlice<RTStructure, 1>,
	pub roof: RTSlice<RTRoof, 1>,
	pub cover: RTSlice<RTCover, 16>,
}

/// Map for a whole chunk of `TileTopical`. Used when mapping to tiles for the `PhysicalChunk`.
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
pub struct WorldTile {
	pub terrain: RTSlice<RTTerrain, 4>,
	pub item: RTSlice<RTItem, 1>,
	pub building: RTSlice<RTBuilding, 16>,
	pub structure: RTSlice<RTStructure, 1>,
	pub roof: RTSlice<RTRoof, 1>,
	pub cover: RTSlice<RTCover, 16>,
	pub order: RTSlice<RTPhysicalOrder, 8>,
}

#[derive(Copy, Clone)]
pub struct RTPhysicalOrder {
	pub physical_order: PhysicalOrder,
	pub action_entity: Entity,
}

pub enum TileDamageState {
	None,
	DamageIdx(u8),
	Destroyed,
}

impl WorldTile {
	pub fn damage_structure(
		&mut self,
		quantity: u32,
		registry: &Registry,
	) -> TileDamageState {
		let Some(rt_structure) = self.structure.slice_mut().first_mut().unwrap() else {
			return TileDamageState::None;
		};
		rt_structure.damage += quantity;
		if rt_structure.damage >= rt_structure.tile.work {
			if let Some(drop_table) = &rt_structure.tile.drop_table {
				registry.item(drop_table.random(), |protocol| {
					self.item.set_rt_quick(protocol.instantiate());
				});
			}
			self.structure.remove_rt_height(0);
			let mut remove_order: Vec<usize> = Vec::new();
			for (i, order) in self.order.slice().iter().enumerate() {
				let Some(rt_physical_order) = order else { continue; };
				if let PhysicalOrder::Mine(_) = &rt_physical_order.physical_order {
					remove_order.push(i);
				}
			}
			let order_slice = self.order.slice_mut();
			for idx in remove_order {
				order_slice[idx] = None;
			}
			return TileDamageState::Destroyed;
		}
		let damage_percent = rt_structure.damage as f32 / rt_structure.tile.work as f32;
		let damage_idx =  (damage_percent * 2.0 + 1.0).round() as u8; // [1..=3]
		TileDamageState::DamageIdx(damage_idx)
	}
}