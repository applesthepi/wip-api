use crate::ModTerrain;

#[repr(C)]
pub struct TileTerrain {
	pub mod_terrain: Option<Box<dyn ModTerrain>>,
}

impl Default for TileTerrain {
	fn default() -> Self {
		Self {
			mod_terrain: None,
		}
	}
}