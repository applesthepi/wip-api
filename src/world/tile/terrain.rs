use nalgebra::{vector, Vector3};

use crate::ModTerrain;

#[repr(C)]
pub struct TileTerrain {
	// pub mod_terrain: Option<Box<dyn ModTerrain>>,
	pub color: Vector3<f32>,
}

impl Default for TileTerrain {
	fn default() -> Self {
		Self {
			// mod_terrain: None,
			color: vector![1.0, 1.0, 1.0],
		}
	}
}