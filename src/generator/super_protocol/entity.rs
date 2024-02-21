use std::slice::Iter;
use crate::{ModIdentifier, ProtocolGroup, ProtocolIdentifier, RTItem, TileItem};

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

#[derive(Clone, Default)]
pub struct RTHumanBuild {
	pub human_build: HumanBuild,
	pub protocol_identifier: ProtocolIdentifier,
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
	pub build: RTHumanBuild,
	pub attire: RTHumanAttire,
}

impl RTEntityHuman {
	pub fn get_textures(
		&self,
	) -> RTEntityTextures {
		let mut extras = Vec::new();
		if let Some(protocol_identifier) = &self.attire.shoes {
			extras.push(protocol_identifier.clone());
		}
		(self.build.protocol_identifier.clone(), extras)
	}
}

pub type RTEntityTextures = (ProtocolIdentifier, Vec<ProtocolIdentifier>);

#[derive(Clone)]
pub enum RTEntityType {
	Human(RTEntityHuman),
}

impl RTEntityType {
	pub fn get_textures(
		&self,
	) -> RTEntityTextures {
		match self {
			RTEntityType::Human(rt_entity_human) => rt_entity_human.get_textures(),
		}
	}
}

#[derive(Clone)]
pub struct Inventory {
	items: Vec<(ProtocolIdentifier, i32)>,
	dirty: bool,
}

impl Default for Inventory {
	fn default() -> Self { Self {
		items: vec![
			(ProtocolIdentifier::new(ModIdentifier::new("", ""), ProtocolGroup::Item, "test"), 1),
		],
		dirty: false,
	}}
}

impl Inventory {
	pub fn items(
		&self,
	) -> Iter<(ProtocolIdentifier, i32)> {
		self.items.iter()
	}

	pub fn dirty(
		&self,
	) -> bool {
		self.dirty
	}

	pub fn clean(
		&mut self,
	) {
		self.dirty = false;
	}

	pub fn add_item(
		&mut self,
		protocol_identifier: ProtocolIdentifier,
		n: i32,
	) {
		self.dirty = true;
		for item_set in self.items.iter_mut() {
			if item_set.0 != protocol_identifier { continue; }
			item_set.1 += n;
			return;
		}
		self.items.push((protocol_identifier, n));
	}
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