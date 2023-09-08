use std::sync::{Arc, RwLock};

use crate::{PhysicalChunk, PT_CACHE_COUNT, prelude::ChunkPositionAbs};

pub struct CachedChunks {
	pub cached_chunks: Vec<(ChunkPositionAbs, Arc<PhysicalChunk>)>,
}

impl Default for CachedChunks {
	fn default() -> Self {
		Self {
			cached_chunks: Vec::with_capacity(PT_CACHE_COUNT),
		}
	}
}