use std::slice::Iter;
use bevy::prelude::{Rect, Vec2};
use crate::{EquippedICState, ICState, ModIdentifier, ProtocolGroup, ProtocolIdentifier, RTItem, TileItem};

#[derive(Clone, Default, PartialEq)]
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

	pub fn get_hitbox(
		&self,
	) -> Rect { match self {
		_ => Rect::from_corners(
			Vec2::new(0.21875, 0.15625),
			Vec2::new(0.78125, 1.34375),
		)
	}}
}

#[derive(Clone, Default)]
pub struct RTHumanBuild {
	pub human_build: HumanBuild,
	pub protocol_identifier: ProtocolIdentifier,
}

/// Material used in damage calculations (with depth in cm)
#[derive(Clone)]
pub enum EffectiveMaterial {
	None,
	Flesh(f32),
	Wood(f32),
	Composite(f32),
	Concrete(f32),
	Steel(f32),
}

impl EffectiveMaterial {
	/// Remaining bullet penetration into this material.
	pub fn simulate_perforation(
		&self,
		perforation_base: f32,
		material_health: f32,
	) -> (f32, f32) { match self {
		Self::None => (perforation_base, 0.0),
		Self::Flesh(depth) => Self::perforate(perforation_base, material_health, 20.0, *depth),
		Self::Wood(depth) => Self::perforate(perforation_base, material_health, 5.0, *depth),
		Self::Composite(depth) => Self::perforate(perforation_base, material_health, 2.0, *depth),
		Self::Concrete(depth) => Self::perforate(perforation_base, material_health, 1.5, *depth),
		Self::Steel(depth) => Self::perforate(perforation_base, material_health, 1.0, *depth),
	}}

	fn perforate(
		perforation_base: f32,
		material_health: f32,
		material_softness: f32,
		depth: f32,
	) -> (f32, f32) {
		let remaining_perforation = (perforation_base * material_softness) - depth;
		let remaining_perforation_steel = remaining_perforation / material_softness;
		let material_damage = ((remaining_perforation_steel + depth) / depth).min(1.0).powf(4.0);
		(remaining_perforation_steel, material_damage)
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
	pub shoes: Option<u32>,
	pub pants: Option<u32>,
	pub shirt: Option<u32>,
	pub vest: Option<u32>,
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
			extras.push(*protocol_identifier);
		}
		if let Some(protocol_identifier) = &self.attire.pants {
			extras.push(*protocol_identifier);
		}
		if let Some(protocol_identifier) = &self.attire.shirt {
			extras.push(*protocol_identifier);
		}
		if let Some(protocol_identifier) = &self.attire.vest {
			extras.push(*protocol_identifier);
		}
		(self.build.protocol_identifier.clone(), extras)
	}

	pub fn clear_attire(
		&mut self,
	) {
		self.attire = RTHumanAttire::default();
	}
}

pub type RTEntityTextures = (ProtocolIdentifier, Vec<u32>);

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

	pub fn clear_attire(
		&mut self,
	) { match self {
		RTEntityType::Human(rt_entity_human) => rt_entity_human.clear_attire(),
	}}

	pub fn get_hitbox(
		&self,
	) -> Rect { match self {
		Self::Human(rt_entity_human) => rt_entity_human.build.human_build.get_hitbox(),
	}}
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
	dirty_world_entity: bool,
}

impl Default for Inventory {
	fn default() -> Self { Self {
		slots: Vec::new(),
		dirty: false,
		dirty_world_entity: false,
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
	) -> Option<(InventorySlot, bool)> {
		if idx >= self.slots.len() {
			return None;
		}
		self.dirty_world_entity = true;
		let inventory_slot = &mut self.slots[idx];
		let init_count = inventory_slot.count;
		inventory_slot.count -= count;
		if inventory_slot.count <= 0 {
			let valid_drop_count = init_count - inventory_slot.count.max(0);
			let mut inventory_slot = self.slots.remove(idx);
			inventory_slot.count = valid_drop_count;
			return Some((inventory_slot, true));
		}
		let mut inventory_slot = inventory_slot.clone();
		inventory_slot.count = count;
		return Some((inventory_slot, false));
	}

	pub fn drop_idx_all(
		&mut self,
		idx: usize,
	) -> Option<(InventorySlot, bool)> {
		if idx >= self.slots.len() {
			return None;
		}
		self.dirty_world_entity = true;
		return Some((self.slots.swap_remove(idx), true));
	}

	pub fn dirty(
		&self,
	) -> bool {
		self.dirty
	}

	pub fn dirty_world_entity(
		&self,
	) -> bool {
		self.dirty_world_entity
	}

	pub fn clean(
		&mut self,
	) {
		self.dirty = false;
	}

	pub fn clean_world_entity(
		&mut self,
	) {
		self.dirty_world_entity = false;
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
		self.dirty_world_entity = true;
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
	pub faction: u32,
	pub rt_type: RTEntityType,
	pub stats: EntityStats,
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
	HumanAttire(HumanBuild, HumanAttire, EffectiveMaterial),
}