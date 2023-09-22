use wip_primal::{SECTOR_WIDTH, ChunkPositionRel};

use crate::{IntermediateChunk, RawPtr};

// TODO: referance int chunk from here, dont insert it.
#[derive(Default)]
pub struct IntermediateSector {
	chunks: [[Box<IntermediateChunk>; SECTOR_WIDTH as usize]; SECTOR_WIDTH as usize],
}

impl IntermediateSector {
	pub fn get_chunk<'guard>(
		&'guard mut self,
		chunk_position_rel: &ChunkPositionRel,
	) -> &'guard mut IntermediateChunk {
		debug_assert!(
			chunk_position_rel.x < SECTOR_WIDTH &&
			chunk_position_rel.y < SECTOR_WIDTH
		);
		&mut self.chunks
			[chunk_position_rel.x as usize]
			[chunk_position_rel.y as usize]
	}

	pub fn insert_chunk(
		&mut self,
		chunk_position_rel: &ChunkPositionRel,
		mut intermediate_chunk: RawPtr<IntermediateChunk>,
	) {
		debug_assert!(
			chunk_position_rel.x < SECTOR_WIDTH &&
			chunk_position_rel.y < SECTOR_WIDTH
		);
		self.chunks
			[chunk_position_rel.x as usize]
			[chunk_position_rel.y as usize] = intermediate_chunk.take();
	}
}