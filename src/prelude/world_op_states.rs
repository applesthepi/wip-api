use wip_primal::TilePositionAbs;

pub type ConstructionProgress = f32;

#[derive(Clone, Copy)]
pub enum GridDirection {
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest,
}

impl GridDirection {
	pub fn tile_position_abs(
		self,
	) -> TilePositionAbs {
		match self {
			GridDirection::North => TilePositionAbs::new(0, 1),
			GridDirection::NorthEast => TilePositionAbs::new(1, 1),
			GridDirection::East => TilePositionAbs::new(1, 0),
			GridDirection::SouthEast => TilePositionAbs::new(1, -1),
			GridDirection::South => TilePositionAbs::new(0, -1),
			GridDirection::SouthWest => TilePositionAbs::new(-1, -1),
			GridDirection::West => TilePositionAbs::new(-1, 0),
			GridDirection::NorthWest => TilePositionAbs::new(-1, 1),
		}
	}
}