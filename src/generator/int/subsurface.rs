use crate::{ModIdentitifier, PatchGen, PatchGenData, ClimateSunlight};

mod earth;
pub use self::earth::*;

#[derive(Default)]
pub struct IntSubsurface {
	instances: Vec<IntSubsurfaceInstance>,
}

impl IntSubsurface {
	pub fn add_earth_subsurface(
		&mut self,
		mod_identifier: ModIdentitifier,
		patch_gen_data: PatchGenData,
		climate_sunlight: Option<ClimateSunlight>,
	) {
		self.instances.push(IntSubsurfaceInstance::new(
			mod_identifier,
			EarthSubsurface::new(
				patch_gen_data,
				climate_sunlight,
			),
		));
	}
}

pub struct IntSubsurfaceInstance {
	pub mod_identifier: ModIdentitifier,
	pub patch_gen: Box<dyn PatchGen + Send + Sync>,
}

impl IntSubsurfaceInstance {
	pub fn new(
		mod_identifier: ModIdentitifier,
		patch_gen: impl PatchGen + 'static + Send + Sync,
	) -> Self {
		Self {
			mod_identifier,
			patch_gen: Box::new(patch_gen),
		}
	}
}