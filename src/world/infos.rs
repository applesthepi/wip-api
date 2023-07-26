use nalgebra::Vector3;

#[derive(Clone)]
pub struct ItemInfo {
	pub max_quantity: u32,
	pub max_hp: u32,
	pub color: Vector3<f32>,
}