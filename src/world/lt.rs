use std::sync::Arc;

use crate::{ActiveWorld, PhysicalTree, PT_LOD_DEPTH, PT_MOD_WCOUNT};

use super::lt_mod::LodTreeMod;

mod render_lod;
use nalgebra::{Vector2, vector};
pub use render_lod::*;

pub struct LodTree {
	pub cached_trees: Vec<(Vector2<i32>, LodTreeMod)>,
	pub stored_trees: Vec<Vector2<i32>>,
}

impl LodTree {
	pub fn new(
		// active_world: &Arc<ActiveWorld>,
	) -> Self {
		let mut lod_tree = Self {
			cached_trees: Vec::with_capacity(128),
			stored_trees: Vec::with_capacity(128),
		};
		lod_tree
	}

	pub fn get_lod_chunk_colors(
		&mut self,
		physical_tree: &PhysicalTree,
		lod_chunk_position: Vector2<i32>,
		lod: u32,
	) {
		// println!("======= {:.1?}", lod_chunk_position);
		let mut lod_idx: [usize; PT_LOD_DEPTH as usize] = [0; PT_LOD_DEPTH as usize];
		let mut lod_supers: [Vector2<i32>; PT_LOD_DEPTH as usize] = [(); PT_LOD_DEPTH as usize].map(|_| Vector2::default());
		for i_lod in (lod..=PT_LOD_DEPTH).rev() {
			let ws_count = (PT_MOD_WCOUNT * 3 * i_lod as usize).max(PT_MOD_WCOUNT) as i32;
			let super_position = vector![
				lod_chunk_position.x / 3i32.pow(i_lod),
				lod_chunk_position.y / 3i32.pow(i_lod)
			];
			if i_lod == PT_LOD_DEPTH {
				match self.cached_trees.iter().enumerate().find(
					|(i, (position, render_lod))|
					*position == super_position
				) {
					Some((i, (position, render_lod))) => {
						lod_idx[i_lod as usize - 1] = i;
					},
					None => {
						self.cached_trees.push((
							super_position,
							LodTreeMod::new_chained(PT_LOD_DEPTH as usize),
						));
						lod_idx[i_lod as usize - 1] = self.cached_trees.len() - 1;
					},
				};
				lod_supers[i_lod as usize - 1] = super_position;
			} else {
				let mut super_offset: Vector2<i32> = Vector2::default();
				for i_lod_super in (i_lod..PT_LOD_DEPTH).rev() {
					let i_lod_super_position = lod_supers[i_lod_super as usize];
					super_offset += i_lod_super_position;
				}
				let super_offset = lod_supers[i_lod as usize];
				let relative_super = vector![
					super_position.x - super_offset.x,
					super_position.y - super_offset.y
				];
				// println!("{:.1?}", relative_super);
				// lod_idx[i_lod as usize - 1] = ;
			}
		}
	}
}