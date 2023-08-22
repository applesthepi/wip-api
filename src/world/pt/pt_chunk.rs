use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range, sync::{Arc, RwLock}};

use crate::{PT_MOD_SQUARED, WorldTile, PT_MOD_WCOUNT};

pub struct PhysicalChunk {
	tiles: Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>>,
	dirty: bool,
}

impl PhysicalChunk {
	pub fn default() -> PhysicalChunk {
		let mut tiles: Vec<[WorldTile; PT_MOD_WCOUNT]> = Vec::with_capacity(PT_MOD_WCOUNT);
		tiles.resize_with(PT_MOD_WCOUNT, || { [(); PT_MOD_WCOUNT].map(|_| WorldTile::default()) });
		Self {
			tiles: Arc::new(RwLock::new(tiles)),
			dirty: true,
		}
	}

	pub fn pull_contents(
		&mut self,
	) -> Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>> {
		self.dirty = false;
		self.tiles.clone()
	}

	pub fn modify(
		&mut self,
	) -> Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>> {
		self.dirty = true;
		self.tiles.clone()
	}

	pub fn no_modify(
		&self,
	) -> Arc<RwLock<Vec<[WorldTile; PT_MOD_WCOUNT]>>> {
		self.tiles.clone()
	}

	pub fn is_dirty(
		&self,
	) -> bool {
		self.dirty
	}
}