mod generator;
pub use generator::*;
mod world;
pub use world::*;
mod version;
pub use version::*;
mod registry;
pub use registry::*;

/// How many tiles square there are per `PhysicalChunk`.
pub const PT_MOD_WCOUNT: usize = 1024;
/// PT_MOD_WCOUNT squared
pub const PT_MOD_SQUARED: usize = PT_MOD_WCOUNT * PT_MOD_WCOUNT;
/// `PhysicalTree`'s `PhysicalChunk` cache count (no realloc chunks stored in cache)
pub const PT_CACHE_COUNT: usize = 1_000;
/// `PhysicalTree`'s `PhysicalChunk` storage count (no realloc chunks stored on disk)
pub const PT_STORAGE_COUNT: usize = 1_000_000;

pub struct Mod<'a> {
	pub name: &'a str,
	pub mod_version: Version,
	pub game_version: Version,
}