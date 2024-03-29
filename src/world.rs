use std::sync::{Arc, atomic::AtomicBool};

// mod lt;
// pub use lt::*;
// mod lt_mod;
// pub use lt_mod::*;
mod tile;
mod pt;
mod rt;
mod infos;
mod rt_states;
mod singles;
mod operations;
mod pathing;
mod commands;
mod task;

use wip_primal::ChunkPositionAbs;

use crate::AtomicLockPtr;

pub use tile::*;
pub use pt::*;
pub use rt::*;
pub use infos::*;
pub use rt_states::*;
pub use singles::*;
pub use operations::*;
pub use pathing::*;
pub use commands::*;
pub use task::*;

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
	) -> Option<AtomicLockPtr<PhysicalChunk>> {
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
	) -> Result<AtomicLockPtr<PhysicalChunk>, GenerationRequest> {
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