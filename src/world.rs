use std::sync::Arc;

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

#[derive(Default)]
pub struct PhysicalWorld {
	pub cached_chunks: CachedChunks,
	// pub physical_tree: PhysicalTree,
	// pub lod_tree: LodTree,
}

impl PhysicalWorld {
	pub fn get_chunk(
		&mut self,
		chunk_position: ChunkPosition,
	) -> Result<Arc<PhysicalChunk>, GenerationRequest> {
		match self.cached_chunks.cached_chunks.iter().find(
			|(in_chunk_position, _)|
			*in_chunk_position == chunk_position.0
		) {
			Some((_, chunk)) => {
				Ok(chunk.clone())
			},
			None => {
				Err(GenerationRequest::Chunk(chunk_position))
			}
		}
	}
}