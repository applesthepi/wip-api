use nalgebra::Vector3;

pub trait ModTerrain {
	fn color(
		&mut self,
	) -> Vector3<f32>;
}