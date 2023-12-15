use wip_primal::{EntityPositionAbs, TilePositionAbs};

use crate::RTIdent;

/// Proxy for submitting world commands cross system/api.
pub struct WorldCommands<'w> {
	pub spawning_criteria: &'w mut EntitySpawningCriteria,
	pub commands: &'w mut Option<Vec<WorldCommand>>,
}

impl<'w> WorldCommands<'w> {
	/// Register entity's existance.
	pub fn entity_register(
		&mut self,
		rt_ident: RTIdent,
		tile_position_abs: TilePositionAbs,
		faction_id: u32,
	) -> u32 {
		let id = self.spawning_criteria.unused_ids.pop();
		let id = match id {
			Some(id) => id,
			None => {
				self.spawning_criteria.next_id += 1;
				self.spawning_criteria.next_id
			},
		};
		self.commands.as_mut().unwrap().push(
			WorldCommand::EntityRegister(id, faction_id, rt_ident, tile_position_abs),
		);
		id
	}

	/// Destroys entity's existance.
	pub fn entity_destroy(
		&mut self,
		id: u32,
		faction_id: u32,
	) {
		self.commands.as_mut().unwrap().push(
			WorldCommand::EntityDestroy(id, faction_id),
		);
	}

	/// Sets how a registered entity is simulated
	/// in the world.
	pub fn entity_set_state(
		&mut self,
		id: u32,
		entity_state: EntityState,
	) {
		self.commands.as_mut().unwrap().push(
			WorldCommand::EntitySetState(id, entity_state),
		);
	}
}

/// Singular command for the world during runtime.
pub enum WorldCommand {
	/// Register entity's existance.
	EntityRegister(u32, u32, RTIdent, TilePositionAbs),
	/// Destroys entity's existance.
	EntityDestroy(u32, u32),
	/// Sets how a registered entity is simulated
	/// in the world.
	EntitySetState(u32, EntityState),
}

/// State for how an entity is processed.
pub enum EntityState {
	/// Stationary in the world with very
	/// limited and very low priority processing.
	Existance,
	/// Simulated roughly for world movement
	/// and interactions. Limited and fixed priority.
	Limited,
	/// Fully simulated with high priority, no
	/// rendering capabilities.
	Process,
	/// Full high priority processing abilities
	/// with rendering capabilities.
	Render,
}

/// Defines how entities are spawned on a low level. Used
/// to proxy information cross system/api.
#[derive(Default)]
pub struct EntitySpawningCriteria {
	pub unused_ids: Vec<u32>,
	pub next_id: u32,
}