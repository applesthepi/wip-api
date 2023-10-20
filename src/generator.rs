mod int_world;
mod int_sector;
mod int_chunk;
mod gen;
mod est_chunk;
mod config;

use crate::RawPtr;

pub use int_world::*;
pub use int_sector::*;
pub use int_chunk::*;
pub use gen::*;
pub use est_chunk::*;
pub use config::*;

pub struct GeneratorInfo<'info> {
	pub name: &'info str,
}

mod protocol;
mod sub_protocol;
mod super_protocol;
mod form;

pub use protocol::*;
pub use sub_protocol::*;
pub use super_protocol::*;
pub use form::*;

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