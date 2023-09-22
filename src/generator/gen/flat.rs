use crate::{Gen, Config, Tile, PhysicalChunk, RawPtr, AtomicGuard, EstChunk, ConfigForm};

pub struct ConfigFlat {
	pub est: f32,
}

impl Config for ConfigFlat {}

pub struct GenFlat<T: Tile, CF: ConfigForm> {
	pub forms: Vec<(T, CF, ConfigFlat)>,
}

impl<T: Tile, CF: ConfigForm> GenFlat<T, CF> {
	pub fn new(
	) -> Self {
		Self {
			forms: Vec::with_capacity(128),
		}
	}
}

impl<T: Tile, CF: ConfigForm> Gen<T, CF, ConfigFlat> for GenFlat<T, CF> {
	fn generate(
		&self,
		physical_chunk: &mut AtomicGuard<PhysicalChunk>,
		est_chunk: &mut EstChunk,
	) {}

	fn add_form(
		&mut self,
		config_form: CF,
		config: ConfigFlat,
		tile: T,
	) {
		self.forms.push((tile, config_form, config));
	}
}