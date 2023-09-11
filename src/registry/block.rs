use std::str::FromStr;

use crate::{Protocol, ProtocolTerrain, ProtocolItem, ProtocolBuilding, ProtocolStructure, ProtocolRoof, ProtocolEntity, ProtocolCover, prelude::ChunkGeneratorSingle};

pub struct RegistryBlock {
	
	// MOD OPTIONS

	pub display_name: String,
	pub folder_name: String,

	// GENERATORS

	// pub world_generators: Vec<*mut ChunkGeneratorSingle>,
	pub chunk_generators: Vec<ChunkGeneratorSingle>,

	// REGISTRY

	pub protocol: Option<Protocol>,
}

impl RegistryBlock {
	pub fn new(
	) -> Self {
		Self {
			display_name: String::from_str("NULL").unwrap(),
			folder_name: String::from_str("NULL").unwrap(),

			// world_generators: Vec::with_capacity(8),
			chunk_generators: Vec::with_capacity(8),

			protocol: Some(Protocol::new()),
		}
	}

	pub fn register_terrain(
		&mut self,
		protocol_terrain: ProtocolTerrain,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().terrain.push(
			protocol_terrain,
		);
	}}

	pub fn register_item(
		&mut self,
		protocol_item: ProtocolItem,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().items.push(
			protocol_item,
		);
	}}

	pub fn register_building(
		&mut self,
		protocol_building: ProtocolBuilding,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().buildings.push(
			protocol_building,
		);
	}}

	pub fn register_structure(
		&mut self,
		protocol_structure: ProtocolStructure,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().structure.push(
			protocol_structure,
		);
	}}

	pub fn register_roof(
		&mut self,
		protocol_roof: ProtocolRoof,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().roofs.push(
			protocol_roof,
		);
	}}

	pub fn register_entity(
		&mut self,
		protocol_entity: ProtocolEntity,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().entities.push(
			protocol_entity,
		);
	}}

	pub fn register_cover(
		&mut self,
		protocol_cover: ProtocolCover,
	) { unsafe {
		self.protocol.as_mut().unwrap_unchecked().cover.push(
			protocol_cover,
		);
	}}
}