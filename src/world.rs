use std::sync::{Arc, atomic::AtomicBool};

use nalgebra::Vector2;

mod tile;
pub use tile::*;
mod pt;
pub use pt::*;
mod lt;
pub use lt::*;
mod lt_mod;
pub use lt_mod::*;
mod rt;
pub use rt::*;
mod infos;
pub use infos::*;
mod rt_states;
pub use rt_states::*;
mod singles;
pub use singles::*;
mod operations;
pub use operations::*;
mod pathing;
pub use pathing::*;

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
			in_chunk_position.x == chunk_position_abs.x as i32 && // TODO: FIX
			in_chunk_position.y == chunk_position_abs.y as i32
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
			in_chunk_position.x == chunk_position_abs.x as i32 && // TODO: FIX
			in_chunk_position.y == chunk_position_abs.y as i32
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
					Vector2::new(chunk_position_abs.x as i32, chunk_position_abs.y as i32),
					chunk,
				));
			},
		}
	}
}