use nalgebra::Vector2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkPosition(pub [i32; 2]);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TilePosition(pub [i32; 2], pub [i32; 2]);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PawnId(pub u32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionProgress(pub u32);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridDirection(pub Vector2<i8>);

impl TilePosition {
	pub fn chunk(&self) -> [i32; 2] {
		self.0
	}

	pub fn tile_rel(&self) -> [i32; 2] {
		self.1
	}

	pub fn tile_abs(&self) -> [i64; 2] {
		[
			self.0[0] as i64 + self.1[0] as i64,
			self.0[1] as i64 + self.1[1] as i64,
		]
	}
}