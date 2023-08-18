use crate::TileStructure;

#[derive(Clone)]
pub struct RTStructure {
	pub tile: TileStructure,
	pub selected: bool,
}

impl RTStructure {
	pub fn new(
		tile: TileStructure,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}