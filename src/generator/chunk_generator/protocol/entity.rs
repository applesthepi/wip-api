use std::str::FromStr;

use crate::noise::ProtocolNoise2d;

#[derive(Clone)]
pub struct ProtocolEntity {
	pub name: Option<String>,
	// pub tile: TileE,
	pub noise: ProtocolNoise2d,
}

impl ProtocolEntity {
	pub fn new(
		name: &str,
		noise: ProtocolNoise2d,
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