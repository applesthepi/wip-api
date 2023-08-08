use crate::TileTerrain;

#[derive(Clone)]
pub struct RTTerrain {
	pub tile: TileTerrain,
	pub selected: bool,
}

impl RTTerrain {
	pub fn new(
		tile: TileTerrain,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}