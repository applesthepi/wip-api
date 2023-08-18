use crate::TileBuilding;

#[derive(Clone)]
pub struct RTBuilding {
	pub tile: TileBuilding,
	pub selected: bool,
}

impl RTBuilding {
	pub fn new(
		tile: TileBuilding,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}