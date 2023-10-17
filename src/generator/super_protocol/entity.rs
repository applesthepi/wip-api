use crate::RTIdent;

#[derive(Clone, Default)]
pub enum HumanBuild {
	#[default]
	Small,
	Medium,
	Large,
	Bulk,
}

#[derive(Clone, Default)]
pub enum HumanAttire {
	#[default]
	Shoes,
	Pants,
	Shirt,
	Vest,
}

#[derive(Clone, Default)]
pub struct RTAttireState {
	pub texture_idx: u32,
	pub durability: f32,
}

#[derive(Clone, Default)]
pub struct RTHumanAttire {
	pub shoes: Option<RTIdent>,
}

#[derive(Clone, Default)]
pub struct RTEntityHuman {
	pub build: HumanBuild,
	pub attire: RTHumanAttire,
}

#[derive(Clone)]
pub enum RTEntityType {
	Human(RTEntityHuman),
}

#[derive(Clone)]
pub struct RTEntity {
	/// Type of entity (human etc.)
	pub rt_type: RTEntityType,
	pub stats: EntityStats,
	/// Texture idx in protocol's atlas.
	pub texture_idx: u32,
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