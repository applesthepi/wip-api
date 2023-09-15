use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range, sync::{Arc, RwLock}};

use wip_primal::CHUNK_WIDTH;
use crate::WorldTile;

pub struct PhysicalChunk {
	pub tiles: Box<[[WorldTile; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize]>,
	// dirty: bool,
}

impl PhysicalChunk {
	pub fn new(
		tiles: Box<[[WorldTile; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize]>,
	) -> Self {
		Self {
			tiles,
		}
	}

	// pub fn pull_contents(
	// 	&mut self,
	// ) -> Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>> {
	// 	self.dirty = false;
	// 	self.tiles.clone()
	// }

	// pub fn modify(
	// 	&mut self,
	// ) -> Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>> {
	// 	self.dirty = true;
	// 	self.tiles.clone()
	// }

	// pub fn no_modify(
	// 	&self,
	// ) -> Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>> {
	// 	self.tiles.clone()
	// }

	// pub fn is_dirty(
	// 	&self,
	// ) -> bool {
	// 	self.dirty
	// }
}