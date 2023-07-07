use std::sync::Arc;

use nalgebra::Vector2;
use tokio::sync::RwLock;

use crate::PT_STORAGE_COUNT;

pub struct StoredChunks {
	stored_chunks: Arc<RwLock<Vec<Vector2<i32>>>>,
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
	) -> Arc<RwLock<Vec<Vector2<i32>>>> {
		self.stored_chunks.clone()
	}
}