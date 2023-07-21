use nalgebra::Vector3;

pub struct ModTerrainReg {
	pub color: Vector3<f32>,
}

pub trait ModTerrain {
	fn reg(
		&mut self,
	) -> ModTerrainReg;
}