use std::sync::{Arc, atomic::AtomicBool};

use crate::{PhysicalChunk, RTItemState, PawnId, TilePosition, ConstructionProgress, ChunkPosition, PathingResult};

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
	Path(TilePosition, TilePosition, Arc<PathingResult>),
}

impl PartialEq for PathingRequest {
	fn eq(&self, other: &Self) -> bool {
		match self {
			PathingRequest::Path(a, b, _) => {
				match other {
					PathingRequest::Path(o_a, o_b, _) => {
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
	Chunk(ChunkPosition, Option<Arc<AtomicBool>>),
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

#[derive(Clone)]
pub enum GenericOperation {
	WorldOp(WorldOperation),
	SavedOp(SavedOperation),
	PawnOp(PawnOperation),
}

#[derive(Clone)]
pub enum WorldOperation {
	SpawnedChunk(ChunkPosition, Arc<PhysicalChunk>, Option<Arc<AtomicBool>>),
}

#[derive(Clone)]
pub enum SavedOperation {
	ConstructingBuilding(TilePosition, ConstructionProgress),
}

#[derive(Clone)]
pub enum PawnOperation {
	LoadInventoryItem(PawnId, RTItemState),
}