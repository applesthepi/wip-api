use crate::TileFloor;

#[derive(Clone, Copy)]
pub struct RTFloor {
	pub tile: TileFloor,
	pub selected: bool,
}

impl RTFloor {
	pub fn new(
		tile: TileFloor,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}