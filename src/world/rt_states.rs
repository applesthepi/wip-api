use crate::ItemInfo;

#[derive(Clone)]
pub struct RTItemState {
	pub info: ItemInfo,
	pub quantity: u32,
	pub hp: u32,
}