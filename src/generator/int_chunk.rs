use wip_primal::CHUNK_WIDTH;

use crate::{Subsurface, AtomicLockPtr, PhysicalChunk, RawPtr, EstChunk, Vegitation};

#[derive(Default)]
pub struct IntermediateChunk {
	pub subsurface: Subsurface,
	pub vegitation: Vegitation,
}

impl IntermediateChunk {
	pub fn postgen(
		&mut self,
	) -> AtomicLockPtr<PhysicalChunk> {
		let mut est_chunk = RawPtr::new(EstChunk::default());
		let mut physical_chunk = AtomicLockPtr::new(PhysicalChunk::default());
		let mut chunk_guard = physical_chunk.acquire();

		self.subsurface.generate(&mut chunk_guard, est_chunk.get());
		self.vegitation.generate(&mut chunk_guard, est_chunk.get());

		for y in 0..(CHUNK_WIDTH as usize) {
			for x in 0..(CHUNK_WIDTH as usize) {
				let tile = &mut chunk_guard.get().tiles[x][y];
				tile.terrain.condense();
			}
		}

		drop(chunk_guard);
		physical_chunk
	}
}