use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

mod block;
pub use block::*;
mod mod_terrain;
pub use mod_terrain::*;

pub struct Registry {
	pub blocks: HashMap<String, Arc<RwLock<RegistryBlock>>>,
}