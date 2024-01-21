use crate::TileBuilding;

#[derive(Clone, Copy)]
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
