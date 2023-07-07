use crate::{PT_MOD_SQUARED, WorldTile};

pub struct PhysicalChunk {
	pub tiles: [WorldTile; PT_MOD_SQUARED],
}

impl Default for PhysicalChunk {
	fn default() -> Self {
		Self {
			tiles: [(); PT_MOD_SQUARED].map(|_| WorldTile::default()),
		}
	}
}