use crate::{WorldGenerator, ChunkGenerator};

pub struct RegistryBlock {
	pub world_generators: Vec<Box<dyn WorldGenerator>>,
	pub chunk_generators: Vec<Box<dyn ChunkGenerator>>,
}