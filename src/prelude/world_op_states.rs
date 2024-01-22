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

	pub fn rotate_cw(
		self,
	) -> GridDirection {
		match self {
			GridDirection::North => GridDirection::NorthEast,
			GridDirection::NorthEast => GridDirection::East,
			GridDirection::East => GridDirection::SouthEast,
			GridDirection::SouthEast => GridDirection::South,
			GridDirection::South => GridDirection::SouthWest,
			GridDirection::SouthWest => GridDirection::West,
			GridDirection::West => GridDirection::NorthWest,
			GridDirection::NorthWest => GridDirection::North,
		}
	}

	pub fn rotate_ccw(
		self,
	) -> GridDirection {
		match self {
			GridDirection::North => GridDirection::NorthWest,
			GridDirection::NorthEast => GridDirection::North,
			GridDirection::East => GridDirection::NorthEast,
			GridDirection::SouthEast => GridDirection::East,
			GridDirection::South => GridDirection::SouthEast,
			GridDirection::SouthWest => GridDirection::South,
			GridDirection::West => GridDirection::SouthWest,
			GridDirection::NorthWest => GridDirection::West,
		}
	}

	pub fn is_edge(
		self,
	) -> bool {
		match self {
			GridDirection::North => true,
			GridDirection::NorthEast => false,
			GridDirection::East => true,
			GridDirection::SouthEast => false,
			GridDirection::South => true,
			GridDirection::SouthWest => false,
			GridDirection::West => true,
			GridDirection::NorthWest => false,
		}
	}
}