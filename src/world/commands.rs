use bevy::prelude::Event;
use wip_primal::{EntityPositionAbs, TilePositionAbs};

use crate::{RTEntity, RTIdent};

/// Proxy for submitting world commands cross system/api.
pub struct WorldCommands<'w> {
	pub spawning_criteria: &'w mut EntitySpawningCriteria,
	pub commands: &'w mut Vec<WorldCommand>,
}

impl<'w> WorldCommands<'w> {
	/// Register entity's existence.
	pub fn entity_register(
		&mut self,
		rt_entity: RTEntity,
		tile_position_abs: TilePositionAbs,
	) -> u32 {
		let id = self.spawning_criteria.unused_ids.pop();
		let id = match id {
			Some(id) => id,
			None => {
				self.spawning_criteria.next_id += 1;
				self.spawning_criteria.next_id
			},
		};
		self.commands.push(
			WorldCommand::WorldEntity(WorldEntityCommand::Register(id, rt_entity, tile_position_abs)),
		);
		id
	}

	/// Destroys entity's existence.
	pub fn entity_destroy(
		&mut self,
		id: u32,
		faction_id: u32,
	) {
		self.commands.push(
			WorldCommand::WorldEntity(WorldEntityCommand::Destroy(id, faction_id)),
		);
	}
}

/// Singular command for the world during runtime.
pub enum WorldCommand {
	WorldEntity(WorldEntityCommand),
}

#[derive(Event)]
pub enum WorldEntityCommand {
	/// Register entity's existence.
	Register(u32, RTEntity, TilePositionAbs),
	/// Destroys entity's existence.
	Destroy(u32, u32),
}

/// Defines how entities are spawned on a low level. Used
/// to proxy information cross system/api.
#[derive(Default)]
pub struct EntitySpawningCriteria {
	pub unused_ids: Vec<u32>,
	pub next_id: u32,
}