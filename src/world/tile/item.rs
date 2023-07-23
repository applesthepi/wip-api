use nalgebra::{vector, Vector3};

use crate::ModItem;

#[repr(C)]
pub struct TileItem {
	// pub mod_item: Option<Box<dyn ModItem>>,
	pub color: Vector3<f32>,
}

impl Default for TileItem {
	fn default() -> Self {
		Self {
			// mod_item: None,
			color: vector![1.0, 1.0, 1.0],
		}
	}
}