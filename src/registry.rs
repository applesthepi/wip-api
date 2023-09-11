use std::{collections::HashMap, sync::Arc};

mod block;
pub use self::block::*;

pub struct ModIdentitifier(String);
impl ModIdentitifier {
	pub fn from_name(
		name: &str,
	) -> Self {
		Self(name.to_string())
	}
}

pub struct Registry {
	pub blocks: HashMap<ModIdentitifier, Arc<RegistryBlock>>,
}

impl Registry {
	pub fn new(
	) -> Self {
		Self {
			blocks: HashMap::with_capacity(32),
		}
	}
}