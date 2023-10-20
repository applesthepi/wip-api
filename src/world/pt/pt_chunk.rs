use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range, sync::{Arc, RwLock}};

use wip_primal::CHUNK_WIDTH;
use crate::WorldTile;

#[derive(Default)]
pub struct PhysicalChunk {
	pub tiles: [[WorldTile; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
	pub entities: Vec<u32>,
	// dirty: bool,
}

impl PhysicalChunk {
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