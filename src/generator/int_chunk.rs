use std::{sync::Arc, mem::MaybeUninit, array};

use wip_primal::{CHUNK_WIDTH, ChunkPositionAbs, TilePositionAbs};

use crate::{IntSubsurface, PhysicalChunk, WorldTile, TileSubsurface, TileSurface, TileTopical, ChunkSubsurface, ChunkSurface, ChunkTopical, RTTerrain, TileTerrain};

// TODO: FIXME
pub struct IntermediateChunk(pub *mut IntermediateChunkRaw);
unsafe impl Send for IntermediateChunk {}
unsafe impl Sync for IntermediateChunk {}
impl Copy for IntermediateChunk {}
impl Clone for IntermediateChunk {
	fn clone(&self) -> Self {
		*self
	}
}

impl Default for IntermediateChunk {
	fn default() -> Self {
		Self(Box::into_raw(Box::new(IntermediateChunkRaw::default())))
	}
}

// impl Drop for IntermediateChunk {
// 	fn drop(&mut self) { unsafe {
// 		drop(Box::from_raw(self.0));
// 	}}
// }

#[derive(Default)]
pub struct IntermediateChunkRaw {
	pub subsurface: IntSubsurface,
}

impl IntermediateChunkRaw {
	pub fn postgen(
		&self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> Arc<PhysicalChunk> {
		let chunk_tile_position_abs = TilePositionAbs::from_chunk_abs_lossy(&chunk_position_abs);
		let mut chunk_subsurface = Box::new(ChunkSubsurface::default());
		let mut chunk_surface = Box::new(ChunkSurface::default());
		let mut chunk_topical = Box::new(ChunkTopical::default());
		println!("subsurface");
		for subsurface in self.subsurface.instances().iter() {
			let patch_origin = subsurface.patch_gen.patch_gen_data().origin;
			let offset = TilePositionAbs::new(
				patch_origin.x - chunk_tile_position_abs.x,
				patch_origin.y - chunk_tile_position_abs.y,
			);
			if offset.x >= 0 &&
				offset.x < CHUNK_WIDTH as i64 &&
				offset.y >= 0 &&
				offset.y < CHUNK_WIDTH as i64 {
				
				chunk_subsurface.tiles[offset.x as usize][offset.y as usize].terrain.set_rt(RTTerrain::new(TileTerrain {
					texture_idx: subsurface.texture_idx as u32,
					tc_hardness: crate::TCHardness::Soft,
					work: 10,
				}));
			}
		}

		let mut tiles: Box<[[WorldTile; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize]> = Box::new(
			[[WorldTile::default(); CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
		);

		for y in 0..(CHUNK_WIDTH as usize) {
			for x in 0..(CHUNK_WIDTH as usize) {
				tiles[x][y] = WorldTile::new(
					chunk_subsurface.tiles[x][y].clone(),
					chunk_surface.tiles[x][y].clone(),
					chunk_topical.tiles[x][y].clone(),
				);
			}
		}

		Arc::new(PhysicalChunk::new(tiles))
	}
}