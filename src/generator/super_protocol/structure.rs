use crate::ProtocolNoise2d;

#[derive(Clone)]
pub enum ProtocolStructureForm {
	Noise(ProtocolNoise2d),
	None,
}