use std::sync::Arc;

use crate::{PhysicalChunk, RTItemState, PawnId, TilePosition, ConstructionProgress, ChunkPosition, PathingResult};

//
// REQUESTS
//

#[derive(Clone)]
pub enum GenericRequest {
	PathingReq(PathingRequest),
	GenerationReq(GenerationRequest),
}

#[derive(Clone)]
pub enum PathingRequest {
	Path(TilePosition, TilePosition, Arc<PathingResult>),
}

#[derive(Clone)]
pub enum GenerationRequest {
	Chunk(ChunkPosition),
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
	SpawnedChunk(ChunkPosition, Arc<PhysicalChunk>),
}

#[derive(Clone)]
pub enum SavedOperation {
	ConstructingBuilding(TilePosition, ConstructionProgress),
}

#[derive(Clone)]
pub enum PawnOperation {
	LoadInventoryItem(PawnId, RTItemState),
}