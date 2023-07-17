mod tile;
use std::sync::Arc;

pub use tile::*;
mod pt;
pub use pt::*;
mod lt;
pub use lt::*;
mod lt_mod;
pub use lt_mod::*;

use crate::ChunkGenerator;

pub struct PhysicalWorld {
	pub physical_tree: PhysicalTree,
	pub lod_tree: LodTree,
}

impl PhysicalWorld {	
}

pub struct ActiveWorld {
	pub physical_world: PhysicalWorld,
	pub chunk_generator: Arc<dyn ChunkGenerator>,
}