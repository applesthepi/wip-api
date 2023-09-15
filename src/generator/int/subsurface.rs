use crate::{ModIdentitifier, PatchGen, PatchGenData, ClimateSunlight, TileSubsurface};

mod earth;
pub use self::earth::*;

#[derive(Default)]
pub struct IntSubsurface {
	instances: Vec<IntSubsurfaceInstance>,
}

impl IntSubsurface {
	pub fn add_earth_subsurface(
		&mut self,
		// mod_identifier: ModIdentitifier,
		texture_idx: usize,
		patch_gen_data: PatchGenData,
		climate_sunlight: Option<ClimateSunlight>,
	) {
		self.instances.push(IntSubsurfaceInstance::new(
			// mod_identifier,
			texture_idx,
			EarthSubsurface::new(
				patch_gen_data,
				climate_sunlight,
			),
		));
	}

	pub fn instances<'get>(
		&'get self,
	) -> &'get Vec<IntSubsurfaceInstance> {
		&self.instances
	}
}

pub struct IntSubsurfaceInstance {
	// pub mod_identifier: ModIdentitifier,
	pub texture_idx: usize,
	pub patch_gen: Box<dyn PatchGen + Send + Sync>,
}

impl IntSubsurfaceInstance {
	pub fn new(
		// mod_identifier: ModIdentitifier,
		texture_idx: usize,
		patch_gen: impl PatchGen + 'static + Send + Sync,
	) -> Self {
		Self {
			// mod_identifier,
			texture_idx,
			patch_gen: Box::new(patch_gen),
		}
	}
}