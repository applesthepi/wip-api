use bevy::prelude::{Commands, Entity, Event};
use wip_primal::{EntityPositionAbs, TilePositionAbs};

use crate::{RTEntity, RTIdent};

/// Proxy for submitting world commands cross system/api.
pub struct WorldCommands<'w> {
	// pub spawning_criteria: &'w mut EntitySpawningCriteria,
	pub commands: &'w mut Vec<WorldCommand>,
}

impl<'w> WorldCommands<'w> {
	/// Register entity's existence.
	pub fn entity_register(
		&mut self,
		commands: &mut Commands,
		rt_entity: RTEntity,
		tile_position_abs: TilePositionAbs,
	) -> WEIdent {
		let we_ident = WEIdent {
			world: commands.spawn_empty().id(),
			ui: commands.spawn_empty().id(),
		};
		self.commands.push(
			WorldCommand::WorldEntity(WorldEntityCommand::Register(we_ident, rt_entity, tile_position_abs)),
		);
		we_ident
	}

	/// Destroys entity's existence.
	pub fn entity_destroy(
		&mut self,
		we_ident: WEIdent,
	) {
		self.commands.push(
			WorldCommand::WorldEntity(WorldEntityCommand::Destroy(we_ident)),
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
	Register(WEIdent, RTEntity, TilePositionAbs),
	/// Destroys entity's existence.
	Destroy(WEIdent),
}

#[derive(Copy, Clone, PartialEq)]
pub struct WEIdent {
	pub world: Entity,
	pub ui: Entity,
}

// Defines how entities are spawned on a low level. Used
// to proxy information cross system/api.

// #[derive(Default)]
// pub struct EntitySpawningCriteria {
// 	pub unused_ids: Vec<u32>,
// 	pub next_id: u32,
// }