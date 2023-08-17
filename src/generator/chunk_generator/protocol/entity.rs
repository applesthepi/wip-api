use std::str::FromStr;

use crate::noise::ProtocolNoise;

#[derive(Clone)]
pub struct ProtocolEntity {
	pub name: Option<String>,
	// pub tile: TileRoof,
	pub noise: ProtocolNoise,
}

impl ProtocolEntity {
	pub fn new(
		name: &str,
		noise: ProtocolNoise,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			// tile: TileRoof {
			// },
			noise,
		}
	}

	// pub fn instantiate(
	// 	&self,
	// ) -> Option<RTRoof> {
	// 	Some(RTRoof::new(self.tile.clone()))
	// }
}