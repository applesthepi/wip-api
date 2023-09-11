use crate::ClimateSunlight;

#[derive(Clone)]
pub struct PTFSurface {
	pub sunlight: Option<ClimateSunlight>,
}

#[derive(Clone)]
pub enum ProtocolTerrainForm {
	Surface(PTFSurface)
}