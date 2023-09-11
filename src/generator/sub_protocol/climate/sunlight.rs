use crate::prelude::op_03_01;

#[derive(Clone)]
pub struct ClimateSunlight {
	/// [0.0, 1.0] Zero means sunlight effects it none, one means sunlight effects it 100%.
	pub effect: f32,
	/// [0.0, 1.0] How much noise is effected. Zero means immune, one means noise is effected 100%.
	pub noise: f32,
	/// [0.0, inf) Multiplier on all generation noise levels for given `ClimateSunlight::noise`. Zero means the tiles wont exist, one means nothing changes.
	pub noise_op: f32,
	/// [0.0, 1.0] How much dryness is effected. Zero means immune, one means dryness is effected 100%.
	pub dryness: f32,
	/// [0, inf) How quickly dryness is effected. Zero means immune. Every tick of sunlight adds little to dryness multiplied by this along with effected variables.
	pub dryness_op: f32,
}

impl ClimateSunlight {
	pub fn validate(&self) -> bool {
		let v_effect = self.effect >= 0.0 && self.effect <= 1.0;
		let v_noise = self.noise >= 0.0 && self.noise <= 1.0;
		let v_noise_op = self.noise_op >= 0.0;
		let v_dryness = self.dryness >= 0.0 && self.dryness <= 1.0;
		let v_dryness_op = self.dryness_op >= 0.0;
		v_effect && v_noise &&
		v_noise_op && v_dryness &&
		v_dryness_op
	}

	/// [0.0, inf)
	/// 0.0  - impossible
	/// <1.0 - negitive modifier on noise
	/// 1.0  - no effect on noise
	/// >1.0 - positive modifier on noise
	pub fn tile_estimate(
		&self,
		height_sample: f32,
	) -> f32 {
		// EFFECTIVNESS AT HEIGHT
		let height_effectivness = op_03_01(height_sample).powf(3.0);
		let effectivness = self.effect;
		let modifier_height = 1.0 - (height_effectivness * effectivness);

		// NOISE OP
		let noise = 1.0 - self.noise;
		let noise_effect = self.noise * self.noise_op;
		let modifier_op = noise_effect + noise;

		// [0.0, inf)
		modifier_height * modifier_op
	}
}