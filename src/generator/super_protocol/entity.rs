use crate::{ProtocolIdentifier, RTItem, TileItem};

#[derive(Clone, Default)]
pub enum HumanBuild {
	Small,
	#[default]
	Medium,
	Large,
	Bulk,
}

impl HumanBuild {
	pub fn as_prefix_str(
		&self,
	) -> &'static str {
		match self {
			HumanBuild::Small => "s",
			HumanBuild::Medium => "m",
			HumanBuild::Large => "l",
			HumanBuild::Bulk => "b",
		}
	}
}

#[derive(Clone)]
pub enum HumanAttire {
	Shoes,
	Pants,
	Shirt,
	Vest,
}

impl HumanAttire {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			HumanAttire::Shoes => "shoes",
			HumanAttire::Pants => "pants",
			HumanAttire::Shirt => "shirt",
			HumanAttire::Vest => "vest",
		}
	}
}

#[derive(Clone, Default)]
pub struct RTAttireState {
	pub texture_idx: u32,
	pub durability: f32,
}

#[derive(Clone, Default)]
pub struct RTHumanAttire {
	pub shoes: Option<ProtocolIdentifier>,
}

#[derive(Clone, Default)]
pub struct RTEntityHuman {
	pub build: HumanBuild,
	pub attire: RTHumanAttire,
}

impl RTEntityHuman {
	pub fn get_textures(
		&self,
	) -> (String, Vec<String>) {
		let body = String::from_iter([
			"human_",
			self.build.as_prefix_str(),
		]);
		let mut extras = Vec::new();
		if let Some(rt_ident) = &self.attire.shoes {

		}
		(body, extras)
	}
}

#[derive(Clone)]
pub enum RTEntityType {
	Human(RTEntityHuman),
}

impl RTEntityType {
	pub fn get_textures(
		&self,
	) -> (String, Vec<String>) {
		match self {
			RTEntityType::Human(rt_entity_human) => rt_entity_human.get_textures(),
		}
	}
}

#[derive(Clone, Default)]
pub struct Inventory {
	/// Vec<(item, count)>
	pub items: Vec<(TileItem, u32)>,
}

#[derive(Clone)]
pub struct RTEntity {
	/// Faction this entity belongs to.
	pub faction: u32,
	/// Type of entity (human etc.)
	pub rt_type: RTEntityType,
	pub stats: EntityStats,
	// Texture idx in protocol's atlas.
	// pub texture_idx: u32,
	pub inventory: Inventory,
}

#[derive(Clone)]
pub struct EntityStats {
	/// Movement speed in meters per second.
	pub mps: f32,
}

/// Describes the entity relationship for this entity protocol.
#[derive(Clone)]
pub enum ProtocolEntityForm {
	Human(HumanBuild, EntityStats),
	HumanAttire(HumanBuild, HumanAttire),
}