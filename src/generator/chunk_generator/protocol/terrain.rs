use std::str::FromStr;

use crate::{RTTerrain, TileTerrain, TCHardness, ProtocolTerrainForm, noise::ProtocolNoise3d};

#[derive(Clone)]
pub struct ProtocolTerrain {
	pub name: Option<String>,
	pub tile: TileTerrain,
	/// Form of terrain has additional configuration.
	pub form: ProtocolTerrainForm,
	pub noise: ProtocolNoise3d,
}

impl ProtocolTerrain {
	pub fn new(
		name: &str,
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		noise: ProtocolNoise3d,
		form: ProtocolTerrainForm,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileTerrain {
				texture_idx: 0,
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			form,
			noise,
		}
	}

	pub fn instantiate(
		&self,
	) -> Option<RTTerrain> {
		Some(RTTerrain::new(self.tile.clone()))
	}
}