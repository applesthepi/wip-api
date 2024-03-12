use wip_primal::TilePositionAbs;
use crate::{BuildingType, RTPhysicalOrder, WorldTile};

#[derive(Clone)]
pub enum Action {
	/// User actions that dont affect the entities.
	Virtual(VirtualOrder),
	/// User actions that affect the `PhysicalWorld` by modifying what the entities do.
	Task(Task),
}

#[derive(Clone, Debug)]
/// Tasks are generated and queued for entities depending on
pub enum Task {
	Work(PhysicalOrder),
	Intermediate(IntermediateOrder),
}

impl Task {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			Task::Work(physical_order) => { physical_order.as_str() },
			Task::Intermediate(intermediate_order) => { intermediate_order.as_str() },
		}
	}

	pub fn get_position(
		&self,
	) -> TilePositionAbs {
		match self {
			Task::Work(physical_order) => physical_order.get_position(),
			Task::Intermediate(intermediate_order) => intermediate_order.get_position(),
		}
	}

	pub fn validate_tile(
		&self,
		world_tile: &WorldTile,
	) -> bool {
		match self {
			Task::Work(physical_order) => {
				physical_order.validate_tile(world_tile)
			},
			Task::Intermediate(intermediate_order) => {
				intermediate_order.validate_tile(world_tile)
			},
		}
	}

	/// Only orders that DON'T NEED an action entity CAN be generated here if it passes validation.
	/// Intermediate orders by definition can NOT have an action entity, and thus are only generated
	/// here if it passes validation.
	/// (construction needs action entity because it IS the action entity)
	pub fn validate_all_new(
		tile_position_abs: TilePositionAbs,
		world_tile: &WorldTile,
		rt_physical_orders: &Vec<RTPhysicalOrder>,
	) -> Vec<Task> {
		let mut tasks = Vec::new();
		let mut po_mine = true;
		let mut po_pickup = true;

		for rt_physical_order in rt_physical_orders.iter() { match rt_physical_order.physical_order {
			PhysicalOrder::Mine(_) => { po_mine = false; },
			PhysicalOrder::Pickup(_) => { po_pickup = false; },
			_ => {},
		}}

		// PHYSICAL

		if po_mine {
			let po_mine = PhysicalOrder::Mine(tile_position_abs);
			if po_mine.validate_tile(world_tile) {
				tasks.push(Task::Work(po_mine));
			}
		}

		if po_pickup {
			let po_pickup = PhysicalOrder::Pickup(tile_position_abs);
			if po_pickup.validate_tile(world_tile) {
				tasks.push(Task::Work(po_pickup));
			}
		}

		// INTERMEDIATE

		let io_move = IntermediateOrder::Move(tile_position_abs);
		if io_move.validate_tile(world_tile) {
			tasks.push(Task::Intermediate(io_move));
		}

		let io_man_machine = IntermediateOrder::ManMachine(tile_position_abs);
		if io_man_machine.validate_tile(world_tile) {
			tasks.push(Task::Intermediate(io_man_machine));
		}

		tasks
	}
}

#[derive(Clone, Debug)]
/// Orders that entities do on their own for other orders, or the user can force these.
pub enum IntermediateOrder {
	Move(TilePositionAbs),
	ManMachine(TilePositionAbs),
}

impl IntermediateOrder {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			IntermediateOrder::Move(_) => "move",
			IntermediateOrder::ManMachine(_) => "man machine",
		}
	}

	pub fn get_position(
		&self,
	) -> TilePositionAbs {
		match self {
			IntermediateOrder::Move(tile_position_abs) => *tile_position_abs,
			IntermediateOrder::ManMachine(tile_position_abs) => *tile_position_abs,
		}
	}

	pub fn validate_tile(
		&self,
		world_tile: &WorldTile,
	) -> bool {
		match self {
			IntermediateOrder::Move(_) => {
				!world_tile.structure.contains_some()
			},
			IntermediateOrder::ManMachine(_) => {
				for rt_building in world_tile.building.slice().iter() {
					let Some(rt_building) = rt_building else {
						continue;
					};
					if rt_building.tile.building_type != BuildingType::Arms {
						continue;
					}
					return true;
				}
				false
			},
		}
	}
}

#[derive(Clone)]
/// Orders that dont affect the entities.
pub enum VirtualOrder {
	Cancel,
}

impl VirtualOrder {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			VirtualOrder::Cancel => "cancel",
		}
	}

	pub fn all_accessible(
	) -> &'static[(&'static VirtualOrder, &'static str)] { &[
		(&VirtualOrder::Cancel, "game/order_cancel.png"),
	]}
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// Orders that the entities can be given by the player manually.
pub enum PhysicalOrder {
	Mine(TilePositionAbs),
	Pickup(TilePositionAbs),
	ConstructBuilding(TilePositionAbs, u32, BuildingType, bool),
	ConstructStructure(TilePositionAbs, u32),
}

impl PhysicalOrder {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			PhysicalOrder::Mine(_) => "mine",
			PhysicalOrder::Pickup(_) => "pickup",
			PhysicalOrder::ConstructBuilding(_, _, _, _) => "construct",
			PhysicalOrder::ConstructStructure(_, _) => "structure",
		}
	}

	pub fn all_accessible(
	) -> &'static[(&'static PhysicalOrder, &'static str)] { &[
		(&PhysicalOrder::Mine(TilePositionAbs::ZERO), "game/order_mine.png"),
	]}

	pub fn with_position(
		mut self,
		tile_position_abs: TilePositionAbs,
	) -> Self {
		self.set_position(tile_position_abs);
		self
	}

	pub fn set_position(
		&mut self,
		tile_position_abs: TilePositionAbs,
	) {
		match self {
			PhysicalOrder::Mine(local_tile_position_abs) => { *local_tile_position_abs = tile_position_abs; },
			PhysicalOrder::Pickup(local_tile_position_abs) => { *local_tile_position_abs = tile_position_abs; },
			PhysicalOrder::ConstructBuilding(local_tile_position_abs, _, _, _) => { *local_tile_position_abs = tile_position_abs; },
			PhysicalOrder::ConstructStructure(local_tile_position_abs, _) => { *local_tile_position_abs = tile_position_abs; },
		}
	}

	pub fn get_position(
		&self,
	) -> TilePositionAbs {
		match self {
			PhysicalOrder::Mine(tile_position_abs) => *tile_position_abs,
			PhysicalOrder::Pickup(tile_position_abs) => *tile_position_abs,
			PhysicalOrder::ConstructBuilding(tile_position_abs, _, _, _) => *tile_position_abs,
			PhysicalOrder::ConstructStructure(tile_position_abs, _) => *tile_position_abs,
		}
	}

	/// Determines if this `PhysicalOrder` is allowed to be applied to this world tile.
	pub fn validate_tile(
		&self,
		world_tile: &WorldTile,
	) -> bool {
		match self {
			PhysicalOrder::Mine(_) => {
				world_tile.structure.contains_some()
			},
			PhysicalOrder::Pickup(_) => {
				world_tile.item.contains_some()
			},
			PhysicalOrder::ConstructBuilding(_, _, building_type, force_solo) => {
				for building in world_tile.building.slice() {
					let Some(building) = building else {
						continue;
					};
					if *building_type == building.tile.building_type &&
						(*force_solo || building.tile.force_solo) {
						return false;
					}
				}
				true
			},
			PhysicalOrder::ConstructStructure(_, _) => {
				!world_tile.structure.contains_some()
			},
		}
	}
}
