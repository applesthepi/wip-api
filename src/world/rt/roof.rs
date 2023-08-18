use crate::TileRoof;

#[derive(Clone)]
pub struct RTRoof {
	pub tile: TileRoof,
	pub selected: bool,
}

impl RTRoof {
	pub fn new(
		tile: TileRoof,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}