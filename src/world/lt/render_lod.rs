use nalgebra::{Vector3, vector};

pub struct RenderLod {
	pub color: Vector3<f32>,
}

impl Default for RenderLod {
	fn default() -> Self {
		Self {
			color: vector![1.0, 1.0, 1.0],
		}
	}
}

// impl RenderLod {
// 	pub fn get_adverage(
// 		&self,
// 	) -> Vector3<f32> {
// 		let mut collective: Vector3<f32> = Vector3::default();
// 		for color in self.color.iter() {
// 			collective += color;
// 		}
// 		vector![
// 			collective.x / 9.0,
// 			collective.y / 9.0,
// 			collective.z / 9.0
// 		]
// 	}
// }