use std::{collections::HashMap, sync::Arc};

mod block;
pub use self::block::*;

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