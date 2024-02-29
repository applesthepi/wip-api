use std::slice::Iter;
use crate::{ICState, ModIdentifier, ProtocolGroup, ProtocolIdentifier, RTItem, TileItem};

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
pub struct InventorySlot {
	pub ic_state: ICState,
	pub idx: u32,
	pub count: i32,
}

#[derive(Clone)]
pub struct Inventory {
	slots: Vec<InventorySlot>,

	dirty: bool,
}

impl Default for Inventory {
	fn default() -> Self { Self {
		slots: Vec::new(),
		dirty: false,
	}}
}

impl Inventory {
	pub fn slots(
		&self,
	) -> Iter<InventorySlot> {
		self.slots.iter()
	}

	pub fn slot(
		&self,
		idx: usize
	) -> &InventorySlot {
		&self.slots[idx]
	}

	pub fn drop_idx_count(
		&mut self,
		idx: usize,
		count: i32,
	) -> Option<InventorySlot> {
		if idx >= self.slots.len() {
			return None;
		}
		let inventory_slot = &mut self.slots[idx];
		let init_count = inventory_slot.count;
		inventory_slot.count -= count;
		if inventory_slot.count <= 0 {
			let valid_drop_count = init_count - inventory_slot.count.max(0);
			let mut inventory_slot = self.slots.remove(idx);
			inventory_slot.count = valid_drop_count;
			return Some(inventory_slot);
		}
		let mut inventory_slot = inventory_slot.clone();
		inventory_slot.count = count;
		return Some(inventory_slot);
	}

	pub fn drop_idx_all(
		&mut self,
		idx: usize,
	) -> Option<InventorySlot> {
		if idx >= self.slots.len() {
			return None;
		}
		return Some(self.slots.swap_remove(idx));
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

	pub fn add_slot_rt(
		&mut self,
		rt_item: &RTItem,
	) {
		let inventory_slot = InventorySlot {
			ic_state: rt_item.tile.ic_state.clone(),
			idx: rt_item.tile.texture_idx,
			count: rt_item.count,
		};
		self.dirty = true;
		for item_set in self.slots.iter_mut() {
			if item_set.idx != inventory_slot.idx { continue; }
			item_set.count += inventory_slot.count;
			return;
		}
		self.slots.push(inventory_slot);
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