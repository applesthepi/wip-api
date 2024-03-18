use std::slice::Iter;
use bevy::prelude::{error, Rect, Vec2};
use smallvec::SmallVec;
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};
use crate::{EquippedICState, ICState, ModIdentifier, ProjectileLive, ProtocolGroup, ProtocolIdentifier, RTItem, TileItem};

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

pub enum DamageForm {
	BulletDirect(ProjectileLive),
}

pub enum DamageRemainder {
	/// (remaining perforation),
	Bullet(f32),
}

impl DamageForm {
	pub fn defect(
		&self,
		effective_material: &EffectiveMaterial,
		material_health: f32,
		remaining_perforation: Option<f32>,
	) -> (Option<EntityDefect>, Option<DamageRemainder>) { match self {
		Self::BulletDirect(projectile_live) => {
			let (
				remaining_perforation_distance,
				damage_percent,
			) = projectile_live.ammo_type.simulate_perforation(
				effective_material,
				material_health,
				projectile_live.bullet_speed_factor,
				remaining_perforation,
			);
			let damage_percent = damage_percent.min(1.0);
			let mut idr = projectile_live.ammo_type.impact_damage_ratio(projectile_live.bullet_speed_factor);
			idr *= 0.01;
			let mut damage_remainder = None;
			if remaining_perforation_distance > 0.0 {
				damage_remainder = Some(DamageRemainder::Bullet(remaining_perforation_distance));
			}
			if damage_percent > 0.1 {(
				Some(EntityDefect {
					entity_defect_type: EntityDefectType::Perforation,
					defect_strength: damage_percent,
					direct_damage: EffectVal::PercentagePointChange(
						-damage_percent * idr
					),
				}),
				damage_remainder,
			)} else if damage_percent > 0.03 {(
				Some(EntityDefect {
					entity_defect_type: EntityDefectType::Bruise,
					defect_strength: damage_percent * 5.0,
					direct_damage: EffectVal::PercentageChange(damage_percent * -0.05),
				}),
				damage_remainder,
			)} else {(
				None,
				damage_remainder,
			)}
		},
	}}
}

#[derive(Clone)]
pub struct DefectBodyPart {
	pub defect_body_part_type: DefectBodyPartType,
	pub defects: SmallVec<[EntityDefect; 1]>,
	pub health: f32,
}

impl DefectBodyPart {
	pub fn new(
		defect_body_part_type: DefectBodyPartType,
	) -> Self { Self {
		defect_body_part_type,
		defects: SmallVec::default(),
		health: 1.0,
	}}

	pub fn effects(
		&self,
	) -> Vec<EntityEffect> {
		let mut effects = Vec::new();
		for defect in self.defects.iter() {
			effects.extend(defect.effects().into_iter());
		}
		effects
	}

	pub fn recalculate_health(
		&mut self,
	) {
		let mut health = 1.0;
		for defect in self.defects.iter() {
			let EffectVal::PercentagePointChange(v) = defect.direct_damage else {
				continue;
			};
			health -= v;
		}
		for defect in self.defects.iter() {
			let EffectVal::PercentageChange(v) = defect.direct_damage else {
				continue;
			};
			health *= v + 1.0;
		}
		for defect in self.defects.iter() {
			let EffectVal::MaxPercentage(v) = defect.direct_damage else {
				continue;
			};
			health = health.min(v);
		}
		self.health = health.min(1.0).max(0.0);
	}
}

#[derive(Clone, Copy)]
pub enum DefectOrganType {
	Brain,
	Lungs,
	Heart,
}

#[derive(Clone, Copy)]
pub enum DefectBodyPartType {
	Head,
	Eyes,
	Ears,
	Arms,
	Hands,
	Torso,
	Legs,
	Feet,
}

impl DefectBodyPartType {
	pub fn ordered(
	) -> [Self; 8] {[
		Self::Head,
		Self::Eyes,
		Self::Ears,
		Self::Arms,
		Self::Hands,
		Self::Torso,
		Self::Legs,
		Self::Feet,
	]}

	pub fn index(
		&self,
	) -> usize { match self {
		Self::Head => 0,
		Self::Eyes => 1,
		Self::Ears => 2,
		Self::Arms => 3,
		Self::Hands => 4,
		Self::Torso => 5,
		Self::Legs => 6,
		Self::Feet => 7,
	}}

	pub fn weights(
		damage_form: &DamageForm,
	) -> [u32; 8] { match damage_form {
		DamageForm::BulletDirect(_) => [
			10,
			1,
			2,
			20,
			5,
			150,
			50,
			5,
		],
	}}

	pub fn flesh_depth(
		&self,
	) -> f32 { match self {
		Self::Head => 30.0,
		Self::Eyes => 2.0,
		Self::Ears => 2.0,
		Self::Arms => 10.0,
		Self::Hands => 6.0,
		Self::Torso => 30.0,
		Self::Legs => 15.0,
		Self::Feet => 10.0,
	}}

	pub fn postrequisite(
		&self,
	) -> Option<Self> { match self {
		Self::Eyes | Self::Ears => Some(Self::Head),
		_ => None,
	}}

	pub fn organs(
		&self,
	) -> Vec<DefectOrganType> { match self {
		Self::Head => vec![DefectOrganType::Brain],
		Self::Torso => vec![DefectOrganType::Heart, DefectOrganType::Lungs],
		_ => vec![],
	}}
}

#[derive(Clone)]
pub struct HumanDefects {
	body_parts: [DefectBodyPart; 8],
	dirty: bool,
}

impl Default for HumanDefects {
	fn default() -> Self { Self {
		body_parts: [
			DefectBodyPart::new(DefectBodyPartType::Head),
			DefectBodyPart::new(DefectBodyPartType::Eyes),
			DefectBodyPart::new(DefectBodyPartType::Ears),
			DefectBodyPart::new(DefectBodyPartType::Arms),
			DefectBodyPart::new(DefectBodyPartType::Hands),
			DefectBodyPart::new(DefectBodyPartType::Torso),
			DefectBodyPart::new(DefectBodyPartType::Legs),
			DefectBodyPart::new(DefectBodyPartType::Feet),
		],
		dirty: false,
	}}
}

impl HumanDefects {
	pub fn effects(
		&self,
	) -> Vec<EntityEffect> {
		let mut effects = Vec::new();
		for body_part in self.body_parts.iter() {
			effects.extend(body_part.effects().into_iter());
		}
		effects
	}

	pub fn damage(
		&mut self,
		damage_form: DamageForm,
		attire: &mut RTHumanAttire,
	) -> Option<DamageRemainder> {
		let body_parts = DefectBodyPartType::ordered();
		let body_part_weights = DefectBodyPartType::weights(&damage_form);
		let table = WalkerTableBuilder::new(&body_part_weights);
		let table = table.build();
		let body_part = &mut self.body_parts[table.next()];
		let mut remaining_perforation = None;
		for attire_em in attire.effective_materials(body_part.defect_body_part_type).into_iter() {
			let (
				_entity_defect,
				damage_remainder,
			) = damage_form.defect(
				attire_em,
				1.0, // TODO
				remaining_perforation,
			);
			let Some(damage_remainder) = damage_remainder else {
				return None;
			};
			if let DamageRemainder::Bullet(damage_remainder_remaining_perforation)  = damage_remainder {
				remaining_perforation = Some(damage_remainder_remaining_perforation);
			}
		}
		let flesh_depth = body_part.defect_body_part_type.flesh_depth();
		let (
			entity_defect,
			damage_remainder,
		) = damage_form.defect(
			&EffectiveMaterial::Flesh(flesh_depth),
			body_part.health,
			remaining_perforation,
		);
		if let Some(entity_defect) = entity_defect {
			body_part.defects.push(entity_defect);
			body_part.recalculate_health();
		}
		if damage_remainder.is_none() {
			return None;
		}
		if let Some(DamageRemainder::Bullet(damage_remainder_remaining_perforation)) = damage_remainder {
			remaining_perforation = Some(damage_remainder_remaining_perforation);
		}
		if let Some(postrequisite) = body_part.defect_body_part_type.postrequisite() {
			let postrequisite_body_part = &mut self.body_parts[postrequisite.index()];
			let flesh_depth = postrequisite_body_part.defect_body_part_type.flesh_depth();
			let (
				entity_defect,
				damage_remainder,
			) = damage_form.defect(
				&EffectiveMaterial::Flesh(flesh_depth),
				postrequisite_body_part.health,
				remaining_perforation,
			);
			if let Some(entity_defect) = entity_defect {
				postrequisite_body_part.defects.push(entity_defect);
				postrequisite_body_part.recalculate_health();
			}
			return damage_remainder;
		}
		damage_remainder
	}
}

#[derive(Clone)]
pub struct EntityDefect {
	pub entity_defect_type: EntityDefectType,
	/// [0, 1] - Strength of the specific defect type.
	pub defect_strength: f32,
	/// How much to damage the body part directly.
	pub direct_damage: EffectVal<f32>,
}

impl EntityDefect {
	pub fn effects(
		&self,
	) -> Vec<EntityEffect> {
		self.entity_defect_type.effects(self.defect_strength)
	}
}

#[derive(Clone)]
pub enum EntityDefectType {
	Scar,
	Bruise,
	Perforation,
}

impl EntityDefectType {
	pub fn effects(
		&self,
		defect_strength: f32,
	) -> Vec<EntityEffect> { match self {
		Self::Scar => vec![
			EntityEffect {
				entity_effect_type: EntityEffectType::PainReceptors,
				effect_val: EffectVal::PercentagePointChange(-0.01 * defect_strength),
			},
		],
		Self::Bruise => vec![
			EntityEffect {
				entity_effect_type: EntityEffectType::PainReceptors,
				effect_val: EffectVal::PercentagePointChange(-0.1 * defect_strength),
			},
		],
		Self::Perforation => vec![
			EntityEffect {
				entity_effect_type: EntityEffectType::PainReceptors,
				effect_val: EffectVal::PercentageChange(-0.5 * defect_strength),
			},
		],
	}}
}

#[derive(Clone)]
pub struct EntityEffect {
	pub entity_effect_type: EntityEffectType,
	pub effect_val: EffectVal<f32>,
}

#[derive(Clone)]
pub enum EntityEffectType {
	/// Depends: NONE
	Conciseness,
	/// Depends: NONE
	PainReceptors,
	/// Depends: conciseness and pain receptors
	MovementSpeed,
}

#[derive(Clone)]
pub enum EffectVal<T: Sized> {
	MaxPercentage(T),
	PercentagePointChange(T),
	/// Scaled effect on value by T.
	/// T == 0: nothing happens.
	/// T > 0: positive multiplicative change.
	/// T < 0: negative multiplicative change.
	PercentageChange(T),
}

/// Material used in damage calculations (with depth in cm)
#[derive(Clone, Default)]
pub enum EffectiveMaterial {
	#[default]
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
		material_health: f32,// TODO
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
	shoes: Option<u32>,
	pants: Option<u32>,
	shirt: Option<u32>,
	vest: Option<u32>,

	pub shoes_em: EffectiveMaterial,
	pub pants_em: EffectiveMaterial,
	pub shirt_em: EffectiveMaterial,
	pub vest_em: EffectiveMaterial,
}

impl RTHumanAttire {
	pub fn shoes(
		&self,
	) -> Option<u32> {
		self.shoes.clone()
	}

	pub fn pants(
		&self,
	) -> Option<u32> {
		self.pants.clone()
	}

	pub fn shirt(
		&self,
	) -> Option<u32> {
		self.shirt.clone()
	}

	pub fn vest(
		&self,
	) -> Option<u32> {
		self.vest.clone()
	}

	pub fn set_shoes(
		&mut self,
		idx: u32,
		effective_material: EffectiveMaterial,
	) {
		self.shoes = Some(idx);
		self.shoes_em = effective_material;
	}

	pub fn set_pants(
		&mut self,
		idx: u32,
		effective_material: EffectiveMaterial,
	) {
		self.pants = Some(idx);
		self.pants_em = effective_material;
	}

	pub fn set_shirt(
		&mut self,
		idx: u32,
		effective_material: EffectiveMaterial,
	) {
		self.shirt = Some(idx);
		self.shirt_em = effective_material;
	}

	pub fn set_vest(
		&mut self,
		idx: u32,
		effective_material: EffectiveMaterial,
	) {
		self.vest = Some(idx);
		self.vest_em = effective_material;
	}

	pub fn clear_all(
		&mut self,
	) {
		*self = Self::default();
	}

	pub fn effective_materials(
		&self,
		body_part: DefectBodyPartType,
	) -> Vec<&EffectiveMaterial> { match body_part {
		DefectBodyPartType::Head => vec![],
		DefectBodyPartType::Eyes => vec![],
		DefectBodyPartType::Ears => vec![],
		DefectBodyPartType::Arms => vec![&self.shirt_em],
		DefectBodyPartType::Hands => vec![],
		DefectBodyPartType::Torso => vec![&self.vest_em, &self.shirt_em],
		DefectBodyPartType::Legs => vec![&self.pants_em],
		DefectBodyPartType::Feet => vec![&self.shoes_em],
	}}
}

#[derive(Clone, Default)]
pub struct RTEntityHuman {
	pub build: RTHumanBuild,
	pub attire: RTHumanAttire,
	pub defects: HumanDefects,
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
		self.attire.clear_all();
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