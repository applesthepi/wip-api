use std::sync::{Arc, RwLock};

use nalgebra::Vector2;

use crate::{PhysicalChunk, PT_CACHE_COUNT};

pub struct CachedChunks {
	pub cached_chunks: Vec<(Vector2<i32>, Arc<PhysicalChunk>)>,
}

impl Default for CachedChunks {
	fn default() -> Self {
		Self {
			cached_chunks: Vec::with_capacity(PT_CACHE_COUNT),
		}
	}
}