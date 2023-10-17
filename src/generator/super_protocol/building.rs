use crate::ProtocolNoise2d;

#[derive(Clone)]
pub enum ProtocolBuildingForm {
	Vegitation(ProtocolNoise2d),
	Noise(ProtocolNoise2d),
	None,
}