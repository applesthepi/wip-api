use crate::TileCover;

#[derive(Clone, Copy)]
pub struct RTCover {
	pub tile: TileCover,
	pub selected: bool,
}

impl RTCover {
	pub fn new(
		tile: TileCover,
	) -> Self {
		Self {
			tile,
			selected: false,
		}
	}
}