use crate::PhysicalWorld;

pub trait WorldGenerator {
	/// Generate a blank world (no tile generation).
	fn generate(
		&mut self,
	) -> PhysicalWorld;
}