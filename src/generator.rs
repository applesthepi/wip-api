mod int_world;
mod int_sector;
mod int_chunk;

pub use self::int_world::*;
pub use self::int_sector::*;
pub use self::int_chunk::*;

pub struct GeneratorInfo<'info> {
	pub name: &'info str,
}

mod protocol;
mod sub_protocol;
mod super_protocol;
mod int;

pub use self::protocol::*;
pub use self::sub_protocol::*;
pub use self::super_protocol::*;
pub use self::int::*;

pub trait ChunkGenerator {
	fn info(
		&self,
	) -> GeneratorInfo<'_>;

	fn chunk(
		&mut self,
		intermediate_world: *mut IntermediateWorld,
		intermediate_chunk: IntermediateChunk,
		// TODO: registry? protocol?
	);
}