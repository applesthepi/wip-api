use nalgebra::{vector, Vector3};

#[derive(Clone, Copy)]
pub enum TCHardness {
	Solid,
	Soft,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileTerrain {
	pub color: Vector3<f32>,
	pub tc_hardness: TCHardness,
	pub work: u32,
}

impl Default for TileTerrain {
	fn default() -> Self {
		Self {
			color: vector![1.0, 1.0, 1.0],
			tc_hardness: TCHardness::Solid,
			work: 1,
		}
	}
}