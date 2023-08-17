

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
}