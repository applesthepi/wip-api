use std::sync::{Arc, atomic::AtomicBool};

mod tile;
pub use self::tile::*;
mod pt;
pub use self::pt::*;
// mod lt;
// pub use self::lt::*;
// mod lt_mod;
// pub use self::lt_mod::*;
mod rt;
pub use self::rt::*;
mod infos;
pub use self::infos::*;
mod rt_states;
pub use self::rt_states::*;
mod singles;
pub use self::singles::*;
mod operations;
pub use self::operations::*;
mod pathing;
pub use self::pathing::*;

use crate::prelude::ChunkPositionAbs;

#[derive(Default)]
pub struct PhysicalWorld {
	pub cached_chunks: CachedChunks,
	// pub physical_tree: PhysicalTree,
	// pub lod_tree: LodTree,
}

impl PhysicalWorld {
	// TODO: DOC IMPORTANT
	pub fn get_chunk(
		&mut self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> Option<Arc<PhysicalChunk>> {
		match self.cached_chunks.cached_chunks.iter().find(
			|(in_chunk_position, _)|
			*in_chunk_position == *chunk_position_abs
		) {
			Some((_, chunk)) => {
				Some(chunk.clone())
			},
			None => None
		}
	}

	// TODO: DOC IMPORTANT
	pub fn get_chunk_flagback(
		&mut self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> Result<Arc<PhysicalChunk>, GenerationRequest> {
		match self.cached_chunks.cached_chunks.iter().find(
			|(in_chunk_position, _)|
			*in_chunk_position == *chunk_position_abs
		) {
			Some((_, chunk)) => {
				Ok(chunk.clone())
			},
			None => {
				Err(GenerationRequest::Chunk(*chunk_position_abs, Some(Arc::new(AtomicBool::new(false)))))
			}
		}
	}

	// TODO: DOC IMPORTANT
	pub fn operation(
		&mut self,
		world_operation: WorldOperation,
	) {
		match world_operation {
			WorldOperation::SpawnedChunk(chunk_position_abs, chunk, _) => {
				self.cached_chunks.cached_chunks.push(( // TODO: FIX
					chunk_position_abs,
					chunk,
				));
			},
		}
	}
}