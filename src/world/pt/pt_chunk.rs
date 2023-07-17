use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range};

use crate::{PT_MOD_SQUARED, WorldTile};

pub struct PhysicalChunk {
	tiles: RefCell<Vec<WorldTile>>,
}

impl PhysicalChunk {
	pub fn default() -> PhysicalChunk {
		let mut tiles: Vec<WorldTile> = Vec::with_capacity(PT_MOD_SQUARED);
		tiles.resize_with(PT_MOD_SQUARED, || { WorldTile::default() });
		Self {
			tiles: RefCell::new(tiles),
		}
	}

	pub fn tiles_ref(
		&self,
	) -> Ref<Vec<WorldTile>> {
		self.tiles.borrow()
	}

	pub fn tiles_mut(
		&mut self,
	) -> RefMut<Vec<WorldTile>> {
		self.tiles.borrow_mut()
	}
}