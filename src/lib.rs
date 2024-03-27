#![feature(get_mut_unchecked)]
#![feature(slice_from_ptr_range)]
#![feature(extend_one)]
#![feature(const_trait_impl)]
#![feature(exclusive_range_pattern)]
#![feature(float_minimum_maximum)]

pub mod prelude;

mod generator;
mod world;
mod version;
mod registry;
mod sync;

pub use generator::*;
pub use world::*;
pub use version::*;
pub use registry::*;
pub use sync::*;

// pub const PT_LOD_DEPTH: u32 = 3;

// /// How many tiles square there are per `PhysicalChunk`.
// pub const PT_MOD_WCOUNT: usize = 3usize.pow(PT_LOD_DEPTH);
// /// PT_MOD_WCOUNT squared
// pub const PT_MOD_SQUARED: usize = PT_MOD_WCOUNT * PT_MOD_WCOUNT;
// /// `PhysicalTree`'s `PhysicalChunk` cache count (no realloc chunks stored in cache)
// pub const PT_CACHE_COUNT: usize = 1_000;
// /// `PhysicalTree`'s `PhysicalChunk` storage count (no realloc chunks stored on disk)
// pub const PT_STORAGE_COUNT: usize = 1_000_000;

#[derive(Debug)]
#[repr(C)]
pub struct Mod {
	pub name: String,
	pub mod_version: Version,
	pub game_version: Version,
}