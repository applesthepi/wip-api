use crate::TileTerrain;

#[derive(Clone, Copy)]
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