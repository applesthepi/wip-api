mod tile;
pub use tile::*;
mod pt;
pub use pt::*;

pub struct PhysicalWorld {
	pub physical_tree: PhysicalTree,
}

impl PhysicalWorld {
	// pub fn access_tile(
	// 	&self,
	// 	tile_position: &TilePosition,
	// ) -> WorldTile {
	// 	let linear_tile_position = tile_position.get_linear_tile_position();
	// 	let chunk = self.physical_tree.cached().chunks().blocking_read().iter().find(
	// 		|(chunk_coordinate,
	// 			physical_chunk,
	// 		)|
	// 		*chunk_coordinate == tile_position.chunk_coordinate
	// 	);
	// 	if let Some(chunk) = chunk {
	// 		chunk.1.blocking_read().tiles[linear_tile_position]
	// 	} else {
	// 		drop(chunk);
	// 		panic!("not impl or tile {}, {} does not exist for chunk {}, {}",
	// 			tile_position.tile_coordinate.x,
	// 			tile_position.tile_coordinate.y,
	// 			tile_position.chunk_coordinate.x,
	// 			tile_position.chunk_coordinate.y
	// 		);
	// 	}
	// }
}