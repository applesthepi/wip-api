use crate::{ChunkGenerator, GeneratorInfo, IntermediateChunk, IntermediateWorld, RawPtr};

#[derive(Clone, Copy)]
/// Wrapper for `ChunkGenerator`. Enables the mutable pointer
/// to be shared across threads.
pub struct ChunkGeneratorSingle(*mut dyn ChunkGenerator);
unsafe impl Send for ChunkGeneratorSingle {}
unsafe impl Sync for ChunkGeneratorSingle {}

/// See `ChunkGeneratorSingle` and `ChunkGenerator`.
impl ChunkGeneratorSingle {
	pub fn new(
		chunk_generator: *mut dyn ChunkGenerator,
	) -> Self {
		Self(chunk_generator)
	}

	pub fn info(
		&self,
	) -> GeneratorInfo<'_> { unsafe {
		self.0.as_ref().unwrap_unchecked().info()
	}}

	pub fn chunk(
		&mut self,
		intermediate_world: *mut IntermediateWorld,
		intermediate_chunk: RawPtr<IntermediateChunk>,
	) { unsafe {
		self.0.as_mut().unwrap_unchecked().chunk(
			intermediate_world,
			intermediate_chunk,
		);
	}}
}