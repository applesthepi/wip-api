#[repr(C)]
#[derive(Clone)]
pub struct TileFloor {
	pub texture_idx: u32,
	pub work: u32,
}

impl Default for TileFloor {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			work: 1,
		}
	}
}