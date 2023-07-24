use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range};

use crate::{PT_MOD_SQUARED, WorldTile, PT_MOD_WCOUNT};

pub struct PhysicalChunk {
	tiles: RefCell<Vec<[WorldTile; PT_MOD_WCOUNT]>>,
}

impl PhysicalChunk {
	pub fn default() -> PhysicalChunk {
		let mut tiles: Vec<[WorldTile; PT_MOD_WCOUNT]> = Vec::with_capacity(PT_MOD_WCOUNT);
		tiles.resize_with(PT_MOD_WCOUNT, || { [(); PT_MOD_WCOUNT].map(|_| WorldTile::default()) });
		Self {
			tiles: RefCell::new(tiles),
		}
	}

	pub fn tiles_ref(
		&self,
	) -> Ref<Vec<[WorldTile; PT_MOD_WCOUNT]>> {
		self.tiles.borrow()
	}

	pub fn tiles_mut(
		&mut self,
	) -> RefMut<Vec<[WorldTile; PT_MOD_WCOUNT]>> {
		self.tiles.borrow_mut()
	}
}