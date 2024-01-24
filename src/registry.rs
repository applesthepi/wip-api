use std::{collections::HashMap, sync::Arc, hash::Hash};

mod block;
pub use block::*;

#[derive(Hash, Debug, Default, Clone, PartialEq, Eq)]
pub struct RTIdent {
	pub unlocalized_author: &'static str,
	pub unlocalized_name: &'static str,
	pub unlocalized_protocol: &'static str,
}

#[derive(Hash, Debug, Default, Clone, PartialEq, Eq)]
pub struct ModIdentitifier {
	unlocalized_name: String,
	folder_name: String,
}

impl ModIdentitifier {
	pub fn from_name(
		unlocalized_name: &str,
		folder_name: &str,
	) -> Self {
		Self {
			unlocalized_name: unlocalized_name.to_owned(),
			folder_name: folder_name.to_owned(),
		}
	}

	pub fn name_as_str<'str>(
		&'str self,
	) -> &'str str {
		&self.unlocalized_name
	}

	pub fn folder_as_str<'str>(
		&'str self,
	) -> &'str str {
		&self.folder_name
	}
}

pub struct Registry {
	pub blocks: HashMap<ModIdentitifier, RegistryBlock>,
}

impl Registry {
	pub fn new(
	) -> Self {
		Self {
			blocks: HashMap::with_capacity(32),
		}
	}
}