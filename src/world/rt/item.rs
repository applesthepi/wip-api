use crate::TileItem;

#[derive(Clone)]
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