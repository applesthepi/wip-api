use nalgebra::{vector, Vector3};

#[derive(Clone, Copy)]
pub enum ICState {
	Singular,
	Bolder,
	Rock,
	Gravel,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileItem {
	pub color: Vector3<f32>,
	pub ic_state: ICState,
}

impl Default for TileItem {
	fn default() -> Self {
		Self {
			color: vector![1.0, 1.0, 1.0],
			ic_state: ICState::Singular,
		}
	}
}