use crate::TileCovor;

#[derive(Clone, Copy)]
pub struct RTCovor {
	pub tile: TileCovor,
	pub selected: bool,
}

impl RTCovor {
	pub fn new(
		tile: TileCovor,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}