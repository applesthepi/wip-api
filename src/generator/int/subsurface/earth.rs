use crate::{PatchGen, PatchGenData, ClimateSunlight};

pub struct EarthSubsurface {
	patch_gen_data: PatchGenData,
	climate_sunlight: Option<ClimateSunlight>,
}

impl EarthSubsurface {
	pub fn new(
		patch_gen_data: PatchGenData,
		climate_sunlight: Option<ClimateSunlight>,
	) -> Self {
		Self {
			patch_gen_data,
			climate_sunlight,
		}
	}
}

impl PatchGen for EarthSubsurface {
	fn sample_estimation(
		&self,
	) -> f32 {
		// NOISE/ESTIMATION MODIFIERS
		let climate_sunlight_estimate = if let Some(
			climate_sunlight,
		) = &self.climate_sunlight {
			climate_sunlight.tile_estimate(self.patch_gen_data.height_sample)
		} else { 1.0 };

		// COMBINE NOISE/ESTIMATION MODIFIERS
		climate_sunlight_estimate
	}

	fn generate(
		&self,
	) {
		
	}

	fn patch_gen_data<'data>(
		&'data self,
	) -> &'data PatchGenData {
		&self.patch_gen_data
	}
}