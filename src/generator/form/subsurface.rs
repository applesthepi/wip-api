use wip_primal::CHUNK_WIDTH;

use crate::{TileTerrain, GenFlat, PhysicalChunk, EstChunk, RawPtr, AtomicGuard, Gen, Config, ConfigForm, ConfigFlat, RTTerrain};

pub struct SubsurfaceConfig {
	pub height: u8,
}

impl ConfigForm for SubsurfaceConfig {}

pub struct Subsurface {
	pub flat: GenFlat<TileTerrain, SubsurfaceConfig>,
}

impl Subsurface {
	pub fn generate(
		&mut self,
		physical_chunk: &mut AtomicGuard<PhysicalChunk>,
		est_chunk: &mut EstChunk,
	) {
		for (
			tile,
			config_form,
			config,
		) in self.flat.forms.iter() {
			flat(
				physical_chunk,
				est_chunk,
				tile,
				config_form,
				config,
			);
		}
	}
}

impl Default for Subsurface {
	fn default() -> Self {
		Self {
			flat: GenFlat::new(),
		}
	}
}

fn flat(
	physical_chunk: &mut AtomicGuard<PhysicalChunk>,
	est_chunk: &mut EstChunk,
	tile: &TileTerrain,
	config_form: &SubsurfaceConfig,
	config: &ConfigFlat,
) {
	for y in 0..(CHUNK_WIDTH as usize) {
		for x in 0..(CHUNK_WIDTH as usize) {
			let est = &mut est_chunk.subsurface[config_form.height as usize][x][y];
			if *est >= config.est { continue; }
			*est = config.est;
			let world_tile = &mut physical_chunk.get().tiles[x][y];
			world_tile.terrain.set_rt(
				config_form.height,
				RTTerrain::new(*tile),
			);
		}
	}
}