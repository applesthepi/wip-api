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
}