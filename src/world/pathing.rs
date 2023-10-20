use std::sync::atomic::AtomicU8;

use wip_primal::TilePositionAbs;

use crate::prelude::GridDirection;

pub struct PathingResult {
	///  0 - Wait
	///  1 - Early path complete
	///  2 - Full path complete
	pub path_status: AtomicU8,
	/// Valid, complete, but suboptimal.
	pub early_path: Pathing,
	/// Valid, complete, and optimal.
	pub full_path: Pathing,
}

pub struct Pathing {
	pub src: TilePositionAbs,
	pub dst: TilePositionAbs,
	// TODO: relative.
	// pub path: Vec<GridDirection>,
	pub path: Vec<TilePositionAbs>,
}

impl Pathing {
	pub fn new(
		src: TilePositionAbs,
		dst: TilePositionAbs,
	) -> Self {
		Self {
			src,
			dst,
			path: Vec::with_capacity(128),
		}
	}
}