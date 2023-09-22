use crate::ConfigForm;
use crate::{RawPtr, PhysicalChunk, AtomicGuard, EstChunk, Config, Tile};

mod flat;
mod patch;
mod noise;

pub use self::flat::*;
pub use self::patch::*;
pub use self::noise::*;

pub trait Gen<T: Tile, CF: ConfigForm, C: Config> {
	fn generate(
		&self,
		physical_chunk: &mut AtomicGuard<PhysicalChunk>,
		est_chunk: &mut EstChunk,
	);

	fn add_form(
		&mut self,
		config_form: CF,
		config: C,
		tile: T,
	);
}