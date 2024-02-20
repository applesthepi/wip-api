use crate::TileStructure;

#[derive(Default, Clone)]
pub struct RTStructure {
	pub tile: TileStructure,
	pub damage: u32,
}

impl RTStructure {
	pub fn new(
		tile: TileStructure,
	) -> Self {
		Self {
			tile,
			..Default::default()
		}
	}
}