use std::{collections::HashMap, sync::Arc};

mod block;
pub use block::*;
mod mod_terrain;
pub use mod_terrain::*;

pub struct Registry {
	pub blocks: HashMap<String, Arc<RegistryBlock>>,
}

impl Registry {
	pub fn new(
	) -> Self {
		Self {
			blocks: HashMap::with_capacity(32),
		}
	}
}