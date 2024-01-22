use wip_primal::CHUNK_WIDTH;

pub type EstMap = [[f32; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize];

#[derive(Default)]
pub struct EstChunk {
	pub subsurface: [EstMap; 3],
	pub surface: EstMap,
	pub vegitation: EstMap,
	pub mountain: EstMap,
}