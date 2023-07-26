use std::sync::Arc;

use crate::{WorldGenerator, ChunkGenerator};

pub struct RegistryBlock {
	pub world_generators: Vec<Arc<dyn WorldGenerator>>,
	pub chunk_generators: Vec<Arc<dyn ChunkGenerator + Send + Sync>>,
}

impl RegistryBlock {
	pub fn new(
	) -> Self {
		Self {
			world_generators: Vec::with_capacity(8),
			chunk_generators: Vec::with_capacity(8),
		}
	}
}