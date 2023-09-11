use std::collections::BTreeMap;

use wip_primal::{SectorPositionAbs, ChunkPositionAbs};

use crate::{IntermediateSector, AtomicLock, AtomicGuard, IntermediateChunk};

/// Wrapper for ease of use due to `AtomicLock`. See
/// `IntermediateWorldRaw`.
pub struct IntermediateWorld(
	AtomicLock<IntermediateWorldRaw>,
);

impl IntermediateWorld {
	pub fn new(
	) -> Self {
		IntermediateWorld(AtomicLock::new(
			IntermediateWorldRaw::default(),
		))
	}

	/// Gets a guarded referance to the intermediate
	/// world object. Make sure to read/write as quick
	/// as possible and drop the guard to allow other
	/// threads to access it.
	pub fn world<'guard>(
		&'guard mut self,
	) -> AtomicGuard<'guard, IntermediateWorldRaw> {
		self.0.acquire()
	}
}

type CachedSectors = BTreeMap<SectorPositionAbs, IntermediateSector>;

#[derive(Default)]
/// Intermediate state before final generation occurs.
pub struct IntermediateWorldRaw {
	/// Sectors cached when either they are generated
	/// or loaded into cache when accessed.
	cached_sectors: CachedSectors,
	// TODO: stored chunks
}

impl IntermediateWorldRaw {
	/// Retrives the chunk using given position. If
	/// no chunk exists, `None` will be returned.
	/// Remember, this is intermediate, failing to retrive
	/// a chunk will **not** generate it.
	/// 
	/// # Thread Saftey
	/// 
	/// Access is protected by `AtomicGuard` retrived
	/// by `IntermediateWorld::world`.
	pub fn get_chunk<'guard>(
		&'guard mut self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> Option<&'guard IntermediateChunk> {
		let sector_position_abs = chunk_position_abs.as_sector();
		let intermediate_sector = match self.cached_sectors.get_mut(
			&sector_position_abs,
		) {
			Some(x) => x,
			None => { return None; },
		};
		let chunk_position_rel = match sector_position_abs.attempt_rel_from_abs(
			chunk_position_abs,
		) {
			Some(x) => x,
			None => { return None; },
		};
		Some(
			intermediate_sector.get_chunk(&chunk_position_rel),
		)
	}
}