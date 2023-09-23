use crate::{ClimateSunlight, ProtocolNoise2d, ProtocolNoise3d};

#[derive(Clone)]
pub struct PTFSurface {
	pub sunlight: Option<ClimateSunlight>,
}

#[derive(Clone)]
pub enum ProtocolTerrainForm {
	Flat(Vec<(u8, f32)>),
	Noise(ProtocolNoise3d),
	Surface(PTFSurface),
}