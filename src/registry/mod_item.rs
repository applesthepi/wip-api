use nalgebra::Vector3;

pub struct ModItemReg {
	pub color: Vector3<f32>,
	/// If the item is terrain bound, then it can
	/// not exist with a structure or building.
	pub terrain_bound: bool,
}

pub trait ModItem {
	fn reg(
		&mut self,
	) -> ModItemReg;
}