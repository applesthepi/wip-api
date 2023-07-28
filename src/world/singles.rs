use nalgebra::Vector2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkPosition(pub Vector2<i32>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TilePosition(pub Vector2<i32>, pub Vector2<i32>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PawnId(pub u32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionProgress(pub u32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridDirection(pub Vector2<i8>);

impl TilePosition {
	pub fn chunk(&self) -> Vector2<i32> {
		self.0
	}

	pub fn tile_rel(&self) -> Vector2<i32> {
		self.1
	}

	pub fn tile_abs(&self) -> Vector2<i32> {
		self.0 + self.1
	}
}