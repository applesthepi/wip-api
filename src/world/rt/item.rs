use crate::TileItem;

#[derive(Clone, Copy)]
pub struct RTItem {
	pub tile: TileItem,
	pub selected: bool,
}

impl RTItem {
	pub fn new(
		tile: TileItem,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}