use std::{sync::Arc, str::FromStr};

use crate::{WorldGenerator, ChunkGenerator};

pub struct RegistryBlock {
	pub display_name: String,
	pub folder_name: String,
	pub world_generators: Vec<Arc<dyn WorldGenerator>>,
	pub chunk_generators: Vec<Arc<dyn ChunkGenerator + Send + Sync>>,
}

impl RegistryBlock {
	pub fn new(
	) -> Self {
		Self {
			display_name: String::from_str("NULL").unwrap(),
			folder_name: String::from_str("NULL").unwrap(),
			world_generators: Vec::with_capacity(8),
			chunk_generators: Vec::with_capacity(8),
		}
	}
}