use std::str::FromStr;

use libloading::Library;

use crate::{Protocols, ProtocolTerrain, ProtocolItem, ProtocolBuilding, ProtocolStructure, ProtocolRoof, ProtocolEntity, ProtocolCover, prelude::ChunkGeneratorSingle};

// pub struct RegistryBlock(pub *mut RegistryBlockRaw);
// unsafe impl Send for RegistryBlock {}
// unsafe impl Sync for RegistryBlock {}
// impl Eq for RegistryBlock {}
// impl PartialEq for RegistryBlock {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.0 == other.0
// 	}
// 	fn ne(&self, other: &Self) -> bool {
// 		self.0 != other.0
// 	}
// }
//
// impl RegistryBlock {
// 	pub fn get<'get>(
// 		&'get mut self,
// 	) -> &'get mut RegistryBlockRaw { unsafe {
// 		&mut *self.0
// 	}}
// }

pub struct RegistryBlock {
	
	// MOD OPTIONS

	// pub display_name: String,
	// pub folder_name: String,

	// REGISTRY

	pub chunk_generators: Vec<ChunkGeneratorSingle>,
	pub protocol: Option<Protocols>,
	pub library: Library,
}

impl RegistryBlock {
	pub fn new(
		library: Library,
	) -> Self {
		Self {
			// display_name: String::from_str("NULL").unwrap(),
			// folder_name: String::from_str("NULL").unwrap(),

			chunk_generators: Vec::with_capacity(1),
			protocol: Some(Protocols::new(false)),
			library,
		}
	}

	pub fn register_terrain(
		&mut self,
		protocol_terrain: ProtocolTerrain,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().terrain.push(
			Box::new(protocol_terrain),
		);
	}}

	pub fn register_item(
		&mut self,
		protocol_item: ProtocolItem,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().items.push(
			Box::new(protocol_item),
		);
	}}

	pub fn register_building(
		&mut self,
		protocol_building: ProtocolBuilding,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().buildings.push(
			Box::new(protocol_building),
		);
	}}

	pub fn register_structure(
		&mut self,
		protocol_structure: ProtocolStructure,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().structure.push(
			Box::new(protocol_structure),
		);
	}}

	pub fn register_roof(
		&mut self,
		protocol_roof: ProtocolRoof,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().roofs.push(
			Box::new(protocol_roof),
		);
	}}

	pub fn register_entity(
		&mut self,
		protocol_entity: ProtocolEntity,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().entities.push(
			Box::new(protocol_entity),
		);
	}}

	pub fn register_cover(
		&mut self,
		protocol_cover: ProtocolCover,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().cover.push(
			Box::new(protocol_cover),
		);
	}}
}