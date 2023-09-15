#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileRoof {
	pub texture_idx: u32,
	pub work: u32,
}

impl Default for TileRoof {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			work: 1,
		}
	}
}