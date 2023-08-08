
mod noise;
use std::sync::Arc;

pub use noise::*;
mod terrain;
pub use terrain::*;
mod climate;
pub use climate::*;

pub struct Protocol {
	pub terrain: Vec<ProtocolTerrain>,
}

impl Protocol {
	pub fn terrain_by_frequency(
		&self,
	) -> Vec<ProtocolTerrain> { unsafe {
		let mut sorted = Vec::with_capacity(self.terrain.len());
		sorted.extend_from_slice(&self.terrain);
		sorted.sort_by(
			|x, y| x.frequency.partial_cmp(&y.frequency).unwrap_unchecked()
		);
		sorted
	}}
}