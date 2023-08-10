use std::str::FromStr;

use nalgebra::Vector3;

use crate::{ClimateSunlight, RTTerrain, TileTerrain, TCHardness, noise::ProtocolNoise};

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
	pub tile: TileTerrain,
	/// Form of terrain has additional configuration.
	pub form: ProtocolTerrainForm,
	pub noise: ProtocolNoise,
}

impl ProtocolTerrain {
	pub fn new(
		name: &str,
		tile_color: [f32; 3],
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		noise: ProtocolNoise,
		form: ProtocolTerrainForm,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileTerrain {
				texture_idx: 0,
				color: Vector3::from_column_slice(&tile_color),
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