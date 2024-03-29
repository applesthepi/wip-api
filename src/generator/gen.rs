use crate::ConfigForm;
use crate::{RawPtr, PhysicalChunk, AtomicGuardMut, EstChunk, Config, Tile};

mod flat;
mod patch;
mod noise;

pub use flat::*;
pub use patch::*;
pub use noise::*;

pub trait Gen<T: Tile, CF: ConfigForm, C: Config> {
	fn add_form(
		&mut self,
		config_form: CF,
		config: C,
		tile: T,
	);
}