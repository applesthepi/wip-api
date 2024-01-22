use bevy::prelude::info;
use wip_primal::CHUNK_WIDTH;

use crate::{TileTerrain, GenFlat, PhysicalChunk, EstChunk, RawPtr, AtomicGuard, Gen, Config, ConfigForm, ConfigFlat, RTTerrain, GenPatch, GenNoise, ConfigNoise, TileBuilding, RTBuilding, TileStructure, RTStructure};

pub struct MountainConfig {
}

impl ConfigForm for MountainConfig {}

pub struct Mountain {
	pub flat: GenFlat<TileStructure, MountainConfig>,
	pub patch: GenPatch<TileStructure, MountainConfig>,
	pub noise: GenNoise<TileStructure, MountainConfig>,
}

impl Mountain {
	pub fn generate(
		&mut self,
		physical_chunk: &mut AtomicGuard<PhysicalChunk>,
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

impl Default for Mountain {
	fn default() -> Self {
		Self {
			flat: GenFlat::new(),
			patch: GenPatch::new(),
			noise: GenNoise::new(),
		}
	}
}

fn flat(
	physical_chunk: &mut AtomicGuard<PhysicalChunk>,
	est_chunk: &mut EstChunk,
	tile: &TileStructure,
	config_form: &MountainConfig,
	config: &ConfigFlat,
) {
	for y in 0..(CHUNK_WIDTH as usize) {
		for x in 0..(CHUNK_WIDTH as usize) {
			let est = &mut est_chunk.mountain[x][y];
			if *est >= config.est { continue; }
			*est = config.est;
			let world_tile = &mut physical_chunk.get().tiles[x][y];
			world_tile.structure.set_rt_height(
				0,
				RTStructure::new(*tile),
			);
		}
	}
}

fn noise(
	physical_chunk: &mut AtomicGuard<PhysicalChunk>,
	est_chunk: &mut EstChunk,
	tile: &TileStructure,
	config_form: &MountainConfig,
	config: &ConfigNoise,
) {
	let x = config.rel.x;
	let y = config.rel.y;
	let est = &mut est_chunk.mountain[x as usize][y as usize];
	if *est >= config.est { return; }
	*est = config.est;
	let world_tile = &mut physical_chunk.get().tiles[x as usize][y as usize];
	world_tile.structure.set_rt_height(
		0,
		RTStructure::new(*tile),
	);
}