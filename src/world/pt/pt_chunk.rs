use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range};

use crate::{PT_MOD_SQUARED, WorldTile, PT_MOD_WCOUNT};

pub struct PhysicalChunk {
	pub tiles: Vec<[WorldTile; PT_MOD_WCOUNT]>,
}

impl PhysicalChunk {
	pub fn default() -> PhysicalChunk {
		let mut tiles: Vec<[WorldTile; PT_MOD_WCOUNT]> = Vec::with_capacity(PT_MOD_WCOUNT);
		tiles.resize_with(PT_MOD_WCOUNT, || { [(); PT_MOD_WCOUNT].map(|_| WorldTile::default()) });
		Self {
			tiles,
		}
	}
}