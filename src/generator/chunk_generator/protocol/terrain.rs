use crate::{ProtocolNoise, ClimateSunlight, RTTerrain};

#[derive(Clone)]
pub struct PTFSurface {
	pub sunlight: Option<ClimateSunlight>,
}

#[derive(Clone)]
pub enum ProtocolTerrainForm {
	Surface(PTFSurface)
}

#[derive(Clone)]
pub struct ProtocolTerrain {
	pub name: Option<String>,
	pub rt: Option<RTTerrain>,
	pub frequency: f32,
	/// Form of terrain has additional configuration.
	pub form: ProtocolTerrainForm,
	/// Count spawned on surface. Random range of how many are spawned before noise modifer.
	pub count_range: [u32; 2],
	/// Noise from center to outside. Minimum one, rest can be NONE.
	pub noise_lerp: [Option<ProtocolNoise>; 5],
}

impl ProtocolTerrain {
	pub fn instantiate(
		&self,
	) -> Option<RTTerrain> {
		self.rt.clone()
	}
}