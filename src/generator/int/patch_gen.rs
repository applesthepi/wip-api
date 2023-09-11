use wip_primal::{TilePositionAbs, Bounds};

pub trait PatchGen {
	/// Calculates an estimation of existance given
	/// its tile noise sample.
	fn sample_estimation(
		&self,
	) -> f32;

	/// Generates the tiles for this patch.
	fn generate(
		&self,
	);

	fn patch_gen_data<'data>(
		&'data self,
	) -> &'data PatchGenData;
}

pub struct PatchGenData {
	pub origin: TilePositionAbs,
	pub height_sample: f32,
	pub valid_sample: f32,
	pub applied_bounds: Bounds,
}