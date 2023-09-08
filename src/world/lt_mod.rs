use std::{rc::Rc, sync::Arc};

use crate::{RenderLod, PT_MOD_SQUARED};

pub struct LTMod {
	pub render_lods: [RenderLod; PT_MOD_SQUARED],
	pub sub_mods: [Box<LodTreeMod>; 9],
	// pub render_lod: RenderLod,
}

pub struct LTModL0 {
	pub render_lods: [RenderLod; PT_MOD_SQUARED],
	// pub render_lod: RenderLod,
}

pub struct LodTreeMod {
	pub lt_mod: Option<LTMod>,
	pub lt_mod_l0: Option<LTModL0>,
}

impl LodTreeMod {
	pub fn new_chained(
		remaining_depth: usize,
	) -> Self {
		if remaining_depth == 0 {
			Self {
				lt_mod_l0: Some(LTModL0 {
					render_lods: [(); PT_MOD_SQUARED].map(|_| RenderLod::default()),
					// render_lod: RenderLod::default(),
				}),
				lt_mod: None,
			}
		} else {
			let sub_mods = [(); 9].map(|_| {
				Box::new(LodTreeMod::new_chained(
					remaining_depth - 1,
				))
			});
			Self {
				lt_mod: Some(LTMod {
					render_lods: [(); PT_MOD_SQUARED].map(|_| RenderLod::default()),
					sub_mods,
				}),
				lt_mod_l0: None,
			}
		}
	}
}