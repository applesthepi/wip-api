use crate::ProtocolNoise2d;

#[derive(Clone)]
pub enum ProtocolBuildingForm {
	Vegetation(ProtocolNoise2d),
	None,
}