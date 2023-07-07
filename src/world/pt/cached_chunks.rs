use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{PhysicalChunk, PT_CACHE_COUNT};

pub struct CachedChunks {
	cached_chunks: Arc<RwLock<Vec<Arc<RwLock<PhysicalChunk>>>>>,
}

impl Default for CachedChunks {
	fn default() -> Self {
		Self {
			cached_chunks: Arc::new(RwLock::new(Vec::with_capacity(PT_CACHE_COUNT))),
		}
	}
}

impl CachedChunks {
	pub fn chunks(
		&self,
	) -> Arc<RwLock<Vec<Arc<RwLock<PhysicalChunk>>>>> {
		self.cached_chunks.clone()
	}
}