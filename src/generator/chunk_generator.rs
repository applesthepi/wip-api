use std::sync::Arc;

use nalgebra::Vector2;

use crate::{PhysicalWorld, PhysicalChunk};

mod protocol;
pub use protocol::*;
mod sub_protocol;
pub use sub_protocol::*;
mod super_protocol;
pub use super_protocol::*;

pub trait ChunkGenerator {
	/// Generate a populated chunk **FOR** the world. **DONT** write the chunk!
	fn generate(
		&mut self,
		physical_world: &PhysicalWorld,
		protocol: Arc<Protocol>,
		chunk_coordinates: Vector2<i32>,
	) -> PhysicalChunk;
}