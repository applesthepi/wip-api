use std::{rc::Rc, cell::{RefMut, RefCell, Ref}, borrow::BorrowMut, slice::from_ptr_range, sync::{Arc, RwLock}};

use wip_primal::CHUNK_WIDTH;
use crate::{RTItem, WEIdent, WorldTile};

#[derive(Default)]
pub struct PhysicalChunk {
	pub tiles: [[WorldTile; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
	pub entities: Vec<WEIdent>,
	pub dirty: bool,
}