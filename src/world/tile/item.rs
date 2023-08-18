#[derive(Clone)]
pub enum ICState {
	Singular,
	Bolder,
	Rock,
	Gravel,
}

#[repr(C)]
#[derive(Clone)]
pub struct TileItem {
	pub texture_idx: u32,
	pub ic_state: ICState,
}

impl Default for TileItem {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			ic_state: ICState::Singular,
		}
	}
}