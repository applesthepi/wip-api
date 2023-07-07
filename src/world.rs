mod tile;
pub use tile::*;
mod pt;
pub use pt::*;

pub struct PhysicalWorld {
	pub physical_tree: PhysicalTree,
}