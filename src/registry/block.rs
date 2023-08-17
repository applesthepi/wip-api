use std::{sync::Arc, str::FromStr};

use crate::{WorldGenerator, ChunkGenerator, Protocol, ProtocolTerrain};

pub struct RegistryBlock {
	
	// MOD OPTIONS

	pub display_name: String,
	pub folder_name: String,

	// GENERATORS

	pub world_generators: Vec<Arc<dyn WorldGenerator>>,
	pub chunk_generators: Vec<Arc<dyn ChunkGenerator + Send + Sync>>,

	// REGISTRY

	pub protocol: Option<Protocol>,
}

impl RegistryBlock {
	pub fn new(
	) -> Self {
		Self {
			display_name: String::from_str("NULL").unwrap(),
			folder_name: String::from_str("NULL").unwrap(),

			world_generators: Vec::with_capacity(8),
			chunk_generators: Vec::with_capacity(8),

			protocol: Some(Protocol::new()),
		}
	}

	pub fn register_terrain(
		&mut self,
		protocol_terrain: ProtocolTerrain,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().terrain.push(
			protocol_terrain,
		);
	}}
}