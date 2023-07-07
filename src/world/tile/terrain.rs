use nalgebra::{Vector3, vector};

use crate::ModTerrain;

pub struct TileTerrain {
	pub color: Vector3<f32>,
	pub mod_terrain: Option<Box<dyn ModTerrain>>,
}

impl Default for TileTerrain {
	fn default() -> Self {
		Self {
			color: vector![1.0, 1.0, 1.0],
			mod_terrain: None,
		}
	}
}