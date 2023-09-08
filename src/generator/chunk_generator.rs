use std::sync::Arc;

use crate::PhysicalChunk;
use crate::PhysicalWorld;

mod protocol;
pub use self::protocol::*;
mod sub_protocol;
pub use self::sub_protocol::*;
mod super_protocol;
pub use self::super_protocol::*;

pub trait ChunkGenerator {
	/// Generate a populated chunk **FOR** the world. **DONT** write the chunk!
	fn generate(
		&mut self,
		physical_world: &PhysicalWorld,
		protocol: Arc<Protocol>,
		chunk_coordinates: [i32; 2],
	) -> PhysicalChunk;
}