use std::{cmp::Ordering, ops::{Sub, Add}};

use bevy::{math::I64Vec2, prelude::{UVec2, Vec3, Vec2}, core::{Pod, Zeroable}};

use crate::PT_MOD_WCOUNT;

#[derive(Default, Clone, Copy)]
pub struct TilePositionAbs {
	pub x: i64,
	pub y: i64,
}

impl TilePositionAbs {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	pub fn into_chunk_abs(
		self,
	) -> ChunkPositionAbs {
		ChunkPositionAbs::new(
			self.x as i64 / PT_MOD_WCOUNT as i64,
			self.y as i64 / PT_MOD_WCOUNT as i64,
		)
	}
}

impl PartialEq for TilePositionAbs {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
		self.y == other.y
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

#[derive(Default, Clone, Copy)]
pub struct TilePositionRel {
	pub x: u8,
	pub y: u8,
}

impl TilePositionRel {
	pub fn new(
		x: u8,
		y: u8,
	) -> Self {
		Self { x, y }
	}

	/// Takes a tile relative to a src chunk and converts
	/// 	it to an absolute tile position in the world.
	/// 
	/// # Arguments
	/// 
	/// * `chunk_position_abs` - chunk absolute position
	/// 	that this tile is relative to.
	pub fn into_abs(
		self,
		chunk_position_abs: ChunkPositionAbs,
	) -> TilePositionAbs {
		TilePositionAbs::new(
			chunk_position_abs.x * PT_MOD_WCOUNT as i64 + self.x as i64,
			chunk_position_abs.y * PT_MOD_WCOUNT as i64 + self.y as i64,
		)
	}
}

impl PartialEq for TilePositionRel {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
		self.y == other.y
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

#[derive(Default, Clone, Copy)]
pub struct ChunkPositionAbs {
	pub x: i64,
	pub y: i64,
}

impl ChunkPositionAbs {
	pub const NULL: ChunkPositionAbs = ChunkPositionAbs { x: i64::MAX, y: i64::MAX };

	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}
}

impl PartialEq for ChunkPositionAbs {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
		self.y == other.y
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

impl PartialOrd for ChunkPositionAbs {
	fn ge(&self, other: &Self) -> bool {
		self.x >= other.x &&
		self.y >= other.y
	}

	fn le(&self, other: &Self) -> bool {
		self.x <= other.x &&
		self.y <= other.y
	}

	fn gt(&self, other: &Self) -> bool {
		self.x > other.x &&
		self.y > other.y
	}

	fn lt(&self, other: &Self) -> bool {
		self.x < other.x &&
		self.y < other.y
	}

	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.lt(other) { Some(Ordering::Less) } else
		if self.gt(other) { Some(Ordering::Greater) } else
		{ Some(Ordering::Equal) }
	}
}

impl Sub for ChunkPositionAbs {
	type Output = ChunkPositionRel;
	fn sub(self, rhs: Self) -> Self::Output {
		ChunkPositionRel::new(
			self.x - rhs.x,
			self.y - rhs.y,
		)
	}
}

impl Add for ChunkPositionAbs {
	type Output = ChunkPositionAbs;
	fn add(self, rhs: Self) -> Self::Output {
		ChunkPositionAbs::new(
			self.x + rhs.x,
			self.y + rhs.y,
		)
	}
}

#[derive(Default, Clone, Copy)]
pub struct ChunkPositionRel {
	pub x: i64,
	pub y: i64,
}

impl ChunkPositionRel {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	/// Takes a chunk relative to another chunk and converts
	/// 	it to an absolute chunk position in the world.
	/// 
	/// # Arguments
	/// 
	/// * `chunk_position_abs` - chunk absolute position
	/// 	that this chunk is relative to.
	pub fn into_abs(
		self,
		chunk_position_abs: ChunkPositionAbs,
	) -> ChunkPositionAbs {
		ChunkPositionAbs::new(
			chunk_position_abs.x + self.x,
			chunk_position_abs.y + self.y,
		)
	}
}

impl PartialEq for ChunkPositionRel {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
		self.y == other.y
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}