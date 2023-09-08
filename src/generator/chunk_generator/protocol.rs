

mod terrain;
pub use self::terrain::*;
mod item;
pub use self::item::*;
mod building;
pub use self::building::*;
mod structure;
pub use self::structure::*;
mod roof;
pub use self::roof::*;
mod entity;
pub use self::entity::*;
mod cover;
pub use self::cover::*;

pub struct Protocol {
	pub terrain: Vec<ProtocolTerrain>,
	pub items: Vec<ProtocolItem>,
	pub buildings: Vec<ProtocolBuilding>,
	pub structure: Vec<ProtocolStructure>,
	pub roofs: Vec<ProtocolRoof>,
	pub entities: Vec<ProtocolEntity>,
	pub cover: Vec<ProtocolCover>,
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
			cover: Vec::with_capacity(128),
		}
	}
}