mod terrain;
mod item;
mod building;
mod structure;
mod roof;
mod entity;
mod cover;

use wip_primal::ChunkPositionAbs;
use wip_primal::TilePositionAbs;
use wip_primal::TilePositionRel;

use crate::IntermediateChunk;
use crate::RawPtr;

pub use self::terrain::*;
pub use self::item::*;
pub use self::building::*;
pub use self::structure::*;
pub use self::roof::*;
pub use self::entity::*;
pub use self::cover::*;

pub trait Protocol {
	fn pregen_chunk(
		&self,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		chunk_position_abs: &ChunkPositionAbs,
	);

	fn pregen_tile(
		&self,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		chunk_position_abs: &ChunkPositionAbs,
		tile_position_abs: &TilePositionAbs,
		tile_position_rel: &TilePositionRel,
	);
}

pub struct Protocols {
	// TODO: `Vec<RODynSrc<ProtocolTerrain, ProtocolDyn>>,`
	//  and: `protocol.make_ref::<ProtocolDyn>()`
	// pub type ProtocolDyn = Protocol;
	// pub type ProtocolRef = RORef<ProtocolDyn>;
	pub terrain: Vec<Box<ProtocolTerrain>>,
	pub items: Vec<Box<ProtocolItem>>,
	pub buildings: Vec<Box<ProtocolBuilding>>,
	pub structure: Vec<Box<ProtocolStructure>>,
	pub roofs: Vec<Box<ProtocolRoof>>,
	pub entities: Vec<Box<ProtocolEntity>>,
	pub cover: Vec<Box<ProtocolCover>>,

	pub protocol_ptrs: Option<Vec<Box<dyn Protocol>>>,
}

impl Protocols {
	pub fn new(
		global: bool,
	) -> Self {
		Self {
			terrain: Vec::with_capacity(128),
			items: Vec::with_capacity(1024 * 10),
			buildings: Vec::with_capacity(1024 * 10),
			structure: Vec::with_capacity(1024),
			roofs: Vec::with_capacity(32),
			entities: Vec::with_capacity(128),
			cover: Vec::with_capacity(128),

			protocol_ptrs: if global { Some(Vec::with_capacity(1024 * 20)) } else { None },
		}
	}

	pub fn finalize(
		&mut self,
	) {
		for protocol in self.terrain.iter() {
			self.protocol_ptrs.as_mut().unwrap().push(protocol.clone());
		}
	}

	pub fn get_ptrs<'get>(
		&'get self,
	) -> &'get Vec<Box<dyn Protocol>> {
		self.protocol_ptrs.as_ref().unwrap()
	}
}