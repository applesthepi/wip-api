use bevy::prelude::Entity;
use crate::TileBuilding;

#[derive(Clone, Copy)]
pub struct RTBuilding {
	pub tile: TileBuilding,
	pub dynamic_object: Option<Entity>,
}

impl RTBuilding {
	pub fn new(
		tile: TileBuilding,
	) -> Self {
		Self {
			tile,
			dynamic_object: None,
		}
	}
}
