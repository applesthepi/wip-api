use wip_primal::{CHUNK_WIDTH, ChunkPositionAbs, TilePositionAbs};

use crate::{Subsurface, AtomicLockPtr, PhysicalChunk, Gen, RawPtr, EstChunk};

#[derive(Default)]
pub struct IntermediateChunk {
	pub subsurface: Subsurface,
}

impl IntermediateChunk {
	pub fn postgen(
		&mut self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> AtomicLockPtr<PhysicalChunk> {
		let chunk_tile_position_abs = TilePositionAbs::from_chunk_abs_lossy(&chunk_position_abs);
		let mut est_chunk = RawPtr::new(EstChunk::default());
		let mut physical_chunk = AtomicLockPtr::new(PhysicalChunk::default());
		let mut chunk_guard = physical_chunk.acquire();

		self.subsurface.generate(&mut chunk_guard, est_chunk.get());





		// for subsurface in self.subsurface.instances().iter() {
		// 	// let patch_origin = subsurface.patch_gen.patch_gen_data().origin;
		// 	// let height = op_11_03(subsurface.patch_gen.patch_gen_data().height_sample) as u8;
			

		// 	let patch_gen_data = subsurface.patch_gen.patch_gen_data();
		// 	let offset = TilePositionAbs::new(
		// 		patch_gen_data.origin.x - chunk_tile_position_abs.x,
		// 		patch_gen_data.origin.y - chunk_tile_position_abs.y,
		// 	);
		// 	if offset.x < 0 ||
		// 		offset.x >= CHUNK_WIDTH as i64 ||
		// 		offset.y < 0 ||
		// 		offset.y >= CHUNK_WIDTH as i64 {
		// 		continue;
		// 	}
		// 	patch_gen_data.
		// 	let sample_estimation = subsurface.patch_gen.sample_estimation();
		// 	let (
		// 		last_estimation,
		// 		last_patch_gen,
		// 	) = &mut generation_maps.get().estimation
		// 		[offset.x as usize]
		// 		[offset.y as usize];
		// 	if *last_estimation >= sample_estimation { continue; }
		// 	*last_estimation = sample_estimation;
		// 	*last_patch_gen = Some(subsurface.patch_gen.as_ref());

		// 	// if height == 3 {
		// 	// 	chunk_surface.tiles[offset.x as usize][offset.y as usize].terrain.set_rt(
		// 	// 		0,
		// 	// 		RTTerrain::new(TileTerrain {
		// 	// 			texture_idx: subsurface.texture_idx as u32,
		// 	// 			tc_hardness: crate::TCHardness::Soft,
		// 	// 			work: 10,
		// 	// 		},
		// 	// 	));
		// 	// 	continue;
		// 	// }
		// 	// chunk_subsurface.tiles[offset.x as usize][offset.y as usize].terrain.set_rt(
		// 	// 	height,
		// 	// 	RTTerrain::new(TileTerrain {
		// 	// 		texture_idx: subsurface.texture_idx as u32,
		// 	// 		tc_hardness: crate::TCHardness::Soft,
		// 	// 		work: 10,
		// 	// 	},
		// 	// ));
		// }

		// let mut tiles: Box<[[WorldTile; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize]> = Box::new(
		// 	[[WorldTile::default(); CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
		// );
		// let mut physical_chunk = PhysicalChunk::new(tiles);

		

		// for y in 0..(CHUNK_WIDTH as usize) {
		// 	for x in 0..(CHUNK_WIDTH as usize) {
		// 		physical_chunk.tiles[x][y].
		// 	}
		// }
		drop(chunk_guard);
		physical_chunk
	}
}