use crate::{Gen, Config, Tile, PhysicalChunk, RawPtr, AtomicGuard, EstChunk, ConfigForm};

// TODO: https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/

pub struct ConfigPatch {
	pub est: f32,
}

impl Config for ConfigPatch {}

pub struct GenPatch<T: Tile, CF: ConfigForm> {
	pub forms: Vec<(T, CF, ConfigPatch)>,
}

impl<T: Tile, CF: ConfigForm> GenPatch<T, CF> {
	pub fn new(
	) -> Self {
		Self {
			forms: Vec::with_capacity(128),
		}
	}
}

impl<T: Tile, CF: ConfigForm> Gen<T, CF, ConfigPatch> for GenPatch<T, CF> {
	fn add_form(
		&mut self,
		config_form: CF,
		config: ConfigPatch,
		tile: T,
	) {
		self.forms.push((tile, config_form, config));
	}
}