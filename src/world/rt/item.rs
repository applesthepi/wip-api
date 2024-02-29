use crate::TileItem;

#[derive(Clone, Copy)]
pub struct RTItem {
	pub tile: TileItem,
	pub count: i32,
}

impl RTItem {
	pub fn new(
		tile: TileItem,
	) -> Self {
		Self {
			tile,
			count: 1,
		}
	}
}