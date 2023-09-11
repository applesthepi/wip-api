use std::sync::{Arc, RwLock};

use wip_primal::ChunkPositionAbs;

use crate::PT_STORAGE_COUNT;

pub struct StoredChunks {
	stored_chunks: Arc<RwLock<Vec<ChunkPositionAbs>>>,
}

impl Default for StoredChunks {
	fn default() -> Self {
		Self {
			stored_chunks: Arc::new(RwLock::new(Vec::with_capacity(PT_STORAGE_COUNT))),
		}
	}
}

impl StoredChunks {
	pub fn chunks(
		&self,
	) -> Arc<RwLock<Vec<ChunkPositionAbs>>> {
		self.stored_chunks.clone()
	}
}