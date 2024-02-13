use wip_primal::TilePositionRel;

use crate::{Gen, Config, Tile, PhysicalChunk, RawPtr, AtomicGuardMut, EstChunk, ConfigForm};

pub struct ConfigNoise {
	pub est: f32,
	pub rel: TilePositionRel,
}

impl Config for ConfigNoise {}

pub struct GenNoise<T: Tile, CF: ConfigForm> {
	pub forms: Vec<(T, CF, ConfigNoise)>,
}

impl<T: Tile, CF: ConfigForm> GenNoise<T, CF> {
	pub fn new(
	) -> Self {
		Self {
			forms: Vec::with_capacity(128),
		}
	}
}

impl<T: Tile, CF: ConfigForm> Gen<T, CF, ConfigNoise> for GenNoise<T, CF> {
	fn add_form(
		&mut self,
		config_form: CF,
		config: ConfigNoise,
		tile: T,
	) {
		self.forms.push((tile, config_form, config));
	}
}