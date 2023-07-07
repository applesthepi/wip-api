mod pt_chunk;
use std::cell::{RefCell, Ref};

pub use pt_chunk::*;
mod cached_chunks;
pub use cached_chunks::*;
mod stored_chunks;
pub use stored_chunks::*;

pub struct PhysicalTree {
	pub cached_chunks: RefCell<CachedChunks>,
	pub stored_chunks: RefCell<StoredChunks>,
}

impl PhysicalTree {
	pub fn new(
	) -> Self {
		Self {
			cached_chunks: RefCell::new(CachedChunks::default()),
			stored_chunks: RefCell::new(StoredChunks::default()),
		}
	}

	pub fn cached(
		&self,
	) -> Ref<CachedChunks> {
		self.cached_chunks.borrow()
	}

	pub fn stored<'a>(
		&self,
	) -> Ref<StoredChunks> {
		self.stored_chunks.borrow()
	}

	// pub fn new_load(
	// 	name: &str,
	// ) -> Self {
	// 	Self {
	// 		cached_chunks: Vec::with_capacity(PT_CACHE_COUNT),
	// 		stored_chunks: Vec::with_capacity(PT_STORAGE_COUNT),
	// 	}
	// }
}