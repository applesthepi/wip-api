use crate::ModItem;

#[repr(C)]
pub struct TileItem {
	pub mod_item: Option<Box<dyn ModItem>>,
}

impl Default for TileItem {
	fn default() -> Self {
		Self {
			mod_item: None,
		}
	}
}