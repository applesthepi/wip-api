

mod terrain;
pub use terrain::*;
mod item;
pub use item::*;
mod building;
pub use building::*;
mod structure;
pub use structure::*;
mod roof;
pub use roof::*;
mod entity;
pub use entity::*;
mod covor;
pub use covor::*;

pub struct Protocol {
	pub terrain: Vec<ProtocolTerrain>,
	pub items: Vec<ProtocolItem>,
	pub buildings: Vec<ProtocolBuilding>,
	pub structure: Vec<ProtocolStructure>,
	pub roofs: Vec<ProtocolRoof>,
	pub entities: Vec<ProtocolEntity>,
	pub covor: Vec<ProtocolCovor>,
}

impl Protocol {
	pub fn new(
	) -> Self {
		Self {
			terrain: Vec::with_capacity(128),
			items: Vec::with_capacity(1024 * 10),
			buildings: Vec::with_capacity(1024 * 10),
			structure: Vec::with_capacity(1024),
			roofs: Vec::with_capacity(32),
			entities: Vec::with_capacity(128),
			covor: Vec::with_capacity(128),
		}
	}
}