use wip_primal::TilePositionAbs;
use crate::{BuildingType, WorldTile};

#[derive(Clone)]
pub enum Action {
	/// User actions that dont affect the entities.
	Virtual(VirtualOrder),
	/// User actions that affect the `PhysicalWorld` by modifying what the entities do.
	Task(Task),
}

#[derive(Clone)]
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
}

#[derive(Clone)]
/// Orders that entities do on their own for other orders, or the user can force these.
pub enum IntermediateOrder {
	Move(TilePositionAbs),
}

impl IntermediateOrder {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			IntermediateOrder::Move(_) => "move",
		}
	}

	pub fn get_position(
		&self,
	) -> TilePositionAbs {
		match self {
			IntermediateOrder::Move(tile_position_abs) => *tile_position_abs,
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

#[derive(Copy, Clone, PartialEq)]
/// Orders that the entities can be given by the player manually.
pub enum PhysicalOrder {
	Mine(TilePositionAbs),
	Pickup(TilePositionAbs),
	// Construct(u32, BuildingType),
}

impl PhysicalOrder {
	pub fn as_str(
		&self,
	) -> &'static str {
		match self {
			PhysicalOrder::Mine(_) => "mine",
			PhysicalOrder::Pickup(_) => "pickup",
			// PhysicalOrder::Construct(_, _) => "construct",
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
		}
	}

	pub fn get_position(
		&self,
	) -> TilePositionAbs {
		match self {
			PhysicalOrder::Mine(tile_position_abs) => *tile_position_abs,
			PhysicalOrder::Pickup(tile_position_abs) => *tile_position_abs,
		}
	}

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
		}
	}
}
