use std::{cell::{RefCell, Ref}, sync::{Arc, RwLock}};

use nalgebra::Vector2;

use crate::{ChunkGenerator, PhysicalWorld};

mod pt_chunk;
pub use pt_chunk::*;
mod cached_chunks;
pub use cached_chunks::*;
mod stored_chunks;
pub use stored_chunks::*;

pub struct PhysicalTree {
	pub cached_chunks: RefCell<CachedChunks>,
	pub stored_chunks: RefCell<StoredChunks>,
	pub chunk_generator: Arc<dyn ChunkGenerator>,
	pub physical_world: Option<Arc<PhysicalWorld>>,
}

impl PhysicalTree {
	pub fn new(
		chunk_generator: Arc<dyn ChunkGenerator>,
	) -> Self {
		Self {
			cached_chunks: RefCell::new(CachedChunks::default()),
			stored_chunks: RefCell::new(StoredChunks::default()),
			chunk_generator,
			physical_world: None,
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

	pub fn get_chunk(
		&mut self,
		position: Vector2<i32>,
	) -> Arc<RwLock<PhysicalChunk>> {
		let cached = self.cached().chunks();
		let cached_chunks = cached.read().unwrap();
		match cached_chunks.iter().find(
			|(chunk_position, chunk)|
			*chunk_position == position
		) {
			Some((position, chunk)) => {
				chunk.clone()
			},
			None => {
				drop(cached_chunks);
				let mut cached_chunks = cached.write().unwrap();
				let chunk_generator = unsafe { Arc::get_mut_unchecked(
					&mut self.chunk_generator
				)};
				let physical_chunk = chunk_generator.generate(
					self.physical_world.as_ref().expect("no world set").clone(),
					position,
				);
				let chunk = Arc::new(RwLock::new(
					physical_chunk,
				));
				cached_chunks.push((
					position,
					chunk.clone(),
				));
				chunk
			},
		}
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