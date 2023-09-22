mod int_world;
mod int_sector;
mod int_chunk;
mod gen;
mod est_chunk;
mod config;

use crate::RawPtr;

pub use self::int_world::*;
pub use self::int_sector::*;
pub use self::int_chunk::*;
pub use self::gen::*;
pub use self::est_chunk::*;
pub use self::config::*;

pub struct GeneratorInfo<'info> {
	pub name: &'info str,
}

mod protocol;
mod sub_protocol;
mod super_protocol;
mod form;

pub use self::protocol::*;
pub use self::sub_protocol::*;
pub use self::super_protocol::*;
pub use self::form::*;

pub trait ChunkGenerator {
	fn info(
		&self,
	) -> GeneratorInfo<'_>;

	fn chunk(
		&mut self,
		intermediate_world: *mut IntermediateWorld,
		intermediate_chunk: RawPtr<IntermediateChunk>,
		// TODO: registry? protocol?
	);
}