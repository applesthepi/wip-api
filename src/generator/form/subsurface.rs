use bevy::prelude::info;
use wip_primal::CHUNK_WIDTH;

use crate::{TileTerrain, GenFlat, PhysicalChunk, EstChunk, RawPtr, AtomicGuardMut, Gen, Config, ConfigForm, ConfigFlat, RTTerrain, GenPatch, GenNoise, ConfigNoise};

pub struct SubsurfaceConfig {
	pub height: u8,
}

impl ConfigForm for SubsurfaceConfig {}

pub struct Subsurface {
	pub flat: GenFlat<TileTerrain, SubsurfaceConfig>,
	pub patch: GenPatch<TileTerrain, SubsurfaceConfig>,
	pub noise: GenNoise<TileTerrain, SubsurfaceConfig>,
}

impl Subsurface {
	pub fn generate(
		&mut self,
		physical_chunk: &mut AtomicGuardMut<PhysicalChunk>,
		est_chunk: &mut EstChunk,
	) {
		// FLAT

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

		// NOISE

		for (
			tile,
			config_form,
			config,
		) in self.noise.forms.iter() {
			noise(
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
			patch: GenPatch::new(),
			noise: GenNoise::new(),
		}
	}
}

fn flat(
	physical_chunk: &mut AtomicGuardMut<PhysicalChunk>,
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
			let world_tile = &mut physical_chunk.tiles[x][y];
			world_tile.terrain.set_rt_height(
				config_form.height,
				RTTerrain::new(*tile),
			);
		}
	}
}

fn noise(
	physical_chunk: &mut AtomicGuardMut<PhysicalChunk>,
	est_chunk: &mut EstChunk,
	tile: &TileTerrain,
	config_form: &SubsurfaceConfig,
	config: &ConfigNoise,
) {
	let x = config.rel.x;
	let y = config.rel.y;
	let est = &mut est_chunk.subsurface[config_form.height as usize][x as usize][y as usize];
	if *est >= config.est { return; }
	*est = config.est;
	let world_tile = &mut physical_chunk.tiles[x as usize][y as usize];
	world_tile.terrain.set_rt_height(
		config_form.height,
		RTTerrain::new(*tile),
	);
}