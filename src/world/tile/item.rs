use std::slice::Iter;
use crate::ProtocolIdentifier;

#[derive(Clone, Copy, PartialEq)]
pub enum ICState {
	Normal(NormalICState),
	Equipped(EquippedICState),
}

impl ICState {
	pub fn stack_size(
		&self,
	) -> i32 { match self {
		ICState::Normal(normal_ic_state) => normal_ic_state.stack_size(),
		ICState::Equipped(equipped_ic_state) => equipped_ic_state.stack_size(),
	}}
}

#[derive(Clone, Copy, PartialEq)]
pub enum NormalICState {
	Whole,
	Chunk,
	Normal,
}

impl NormalICState {
	pub fn stack_size(
		&self,
	) -> i32 { match self {
		NormalICState::Whole => 1,
		NormalICState::Chunk => 8,
		NormalICState::Normal => 64,
	}}

	pub fn all(
	) -> &'static [NormalICState] {&[
		NormalICState::Whole,
		NormalICState::Chunk,
		NormalICState::Normal,
	]}
}

#[derive(Clone, Copy, PartialEq)]
pub enum EquippedICState {
	Attire,
}

impl EquippedICState {
	pub fn stack_size(
		&self,
	) -> i32 { match self {
		EquippedICState::Attire => 8,
	}}

	pub fn all(
	) -> &'static [EquippedICState] {&[
		EquippedICState::Attire,
	]}
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TileItem {
	pub texture_idx: u32,
	pub ic_state: ICState,
}

impl TileItem {
	pub fn stack_size(
		&self,
	) -> i32 {
		self.ic_state.stack_size()
	}
}

impl Default for TileItem {
	fn default() -> Self {
		Self {
			texture_idx: 0,
			ic_state: ICState::Normal(NormalICState::Normal),
		}
	}
}