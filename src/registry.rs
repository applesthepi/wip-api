use std::{collections::HashMap, sync::Arc, hash::Hash};
use bevy::prelude::error;

mod block;
pub use block::*;
use crate::{AtomicGuard, AtomicLockPtr, ProtocolGroup, ProtocolItem, ProtocolTerrain};

pub struct StaticProtocolIdentifier {
	pub static_mod_identifier: StaticModIdentifier,
	pub protocol_group: ProtocolGroup,
	pub un_protocol: &'static str,
}

impl StaticProtocolIdentifier {
	pub const fn new(
		static_mod_identifier: StaticModIdentifier,
		protocol_group: ProtocolGroup,
		un_protocol: &'static str,
	) -> Self { Self {
		static_mod_identifier,
		protocol_group,
		un_protocol,
	}}
}

impl Into<ProtocolIdentifier> for StaticProtocolIdentifier {
	fn into(self) -> ProtocolIdentifier {
		ProtocolIdentifier::new(
			self.static_mod_identifier.into(),
			self.protocol_group,
			self.un_protocol,
		)
	}
}

#[derive(Hash, Debug, Default, Clone, PartialEq, Eq)]
pub struct ProtocolIdentifier {
	pub mod_identifier: ModIdentifier,
	pub protocol_group: ProtocolGroup,
	pub un_protocol: String,
}

impl ProtocolIdentifier {
	pub fn new(
		mod_identifier: ModIdentifier,
		protocol_group: ProtocolGroup,
		un_protocol: impl Into<String>,
	) -> Self { Self {
		mod_identifier,
		protocol_group,
		un_protocol: un_protocol.into(),
	}}
}

pub struct StaticModIdentifier {
	pub un_mod: &'static str,
	pub un_author: &'static str,
}

impl StaticModIdentifier {
	pub const fn new(
		un_mod: &'static str,
		un_author: &'static str,
	) -> Self { Self {
		un_mod,
		un_author,
	}}
}

impl Into<ModIdentifier> for StaticModIdentifier {
	fn into(self) -> ModIdentifier {
		ModIdentifier::new(
			self.un_mod,
			self.un_author,
		)
	}
}

#[derive(Hash, Debug, Default, Clone, PartialEq, Eq)]
pub struct ModIdentifier {
	pub un_mod: String,
	pub un_author: String,
}

impl ModIdentifier {
	pub fn new(
		un_mod: impl Into<String>,
		un_author: impl Into<String>,
	) -> Self { Self {
		un_mod: un_mod.into(),
		un_author: un_author.into(),
	}}
}

pub struct Registry {
	// TODO: TEMPORARY! REMOVE atomic lock
	// TODO: PRIVATE ACCESS
	pub blocks: HashMap<ModIdentifier, AtomicLockPtr<RegistryBlock>>,
}

impl Registry {
	pub fn new(
	) -> Self {
		Self {
			blocks: HashMap::with_capacity(32),
		}
	}

	pub fn block(
		&self,
		mod_identifier: &ModIdentifier,
	) -> Option<AtomicGuard<RegistryBlock>> {
		let Some(registry_block) = self.blocks.get(mod_identifier) else {
			return None;
		};
		Some(registry_block.acquire())
	}

	pub fn item<F: FnOnce(&Box<ProtocolItem>)>(
		&self,
		protocol_identifier: ProtocolIdentifier,
		operation: F,
	) {
		let Some(mut block) = self.block(&protocol_identifier.mod_identifier) else {
			error!("failed to locate mod with identifier {:?}", protocol_identifier.mod_identifier);
			return;
		};
		let Some(protocols) = &block.protocol else {
			unreachable!();
		};
		let Some(protocol) = protocols.items.iter().find(|protocol| {
			protocol.un_protocol == protocol_identifier.un_protocol
		}) else {
			error!("failed to locate protocol {}", protocol_identifier.un_protocol);
			return;
		};
		operation(protocol);
	}
}