
use std::sync::Arc;

pub mod noise;
mod terrain;
pub use terrain::*;
mod climate;
pub use climate::*;

pub struct Protocol {
	pub terrain: Vec<ProtocolTerrain>,
}

impl Protocol {
}