use std::sync::{Arc, RwLock};

use wip_primal::{ChunkPositionAbs, CHUNK_WIDTH_SQ};

use crate::{PhysicalChunk, AtomicLockPtr};

pub struct CachedChunks {
	pub cached_chunks: Vec<(ChunkPositionAbs, AtomicLockPtr<PhysicalChunk>)>,
}

impl Default for CachedChunks {
	fn default() -> Self {
		Self {
			cached_chunks: Vec::with_capacity(CHUNK_WIDTH_SQ),
		}
	}
}