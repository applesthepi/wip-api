use std::sync::{Arc, atomic::AtomicBool};

use wip_primal::{TilePositionAbs, ChunkPositionAbs};

use crate::{PhysicalChunk, RTItemState, PathingResult, prelude::{PawnId, ConstructionProgress}, AtomicLockPtr, RawPtr, Pathing};

//
// REQUESTS
//

#[derive(Clone, PartialEq)]
pub enum GenericRequest {
	PathingReq(PathingRequest),
	GenerationReq(GenerationRequest),
}

#[derive(Clone)]
pub enum PathingRequest {
	Path(TilePositionAbs, TilePositionAbs),
}

impl PartialEq for PathingRequest {
	fn eq(&self, other: &Self) -> bool {
		match self {
			PathingRequest::Path(a, b) => {
				match other {
					PathingRequest::Path(
						o_a,
						o_b,
					) => {
						a == o_a && b == o_b
					},
					_ => { false },
				}
			},
		}
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

#[derive(Clone)]
pub enum GenerationRequest {
	Chunk(ChunkPositionAbs, Option<Arc<AtomicBool>>),
}

impl GenerationRequest {
	pub fn take_flagback(
		&mut self,
	) -> Arc<AtomicBool> {
		let flagback_operation = |
			flagback: &mut Option<Arc<AtomicBool>>
		| {
			flagback.take().unwrap()
		};
		match self {
			Self::Chunk(_, flagback) => flagback_operation(flagback),
		}
	}

	pub fn clone_flagback(
		&self,
	) -> Arc<AtomicBool> {
		let flagback_operation = |
			flagback: &Option<Arc<AtomicBool>>
		| {
			flagback.as_ref().unwrap().clone()
		};
		match self {
			Self::Chunk(_, flagback) => flagback_operation(flagback),
		}
	}
}

impl PartialEq for GenerationRequest {
	fn eq(&self, other: &Self) -> bool {
		match self {
			GenerationRequest::Chunk(pos, _) => {
				match other {
					GenerationRequest::Chunk(o_pos, _) => {
						pos == o_pos
					},
					_ => { false },
				}
			},
		}
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

//
// OPERATIONS
//

// TODO: Organize all operations and requests by WR executor.
#[derive(Clone)]
pub enum GenericOperation {
	WorldOp(WorldOperation),
	SavedOp(SavedOperation),
	PawnOp(PawnOperation),
}

#[derive(Clone)]
pub enum WorldOperation {
	SpawnedChunk(ChunkPositionAbs, AtomicLockPtr<PhysicalChunk>, Option<Arc<AtomicBool>>),
}

#[derive(Clone)]
pub enum SavedOperation {
	ConstructingBuilding(TilePositionAbs, ConstructionProgress),
}

#[derive(Clone)]
pub enum PawnOperation {
	LoadInventoryItem(PawnId, RTItemState),
	Pathing(RawPtr<Pathing>),
}