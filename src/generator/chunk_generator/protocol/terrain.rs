use std::str::FromStr;

use nalgebra::Vector3;

use crate::{ProtocolNoise, ClimateSunlight, RTTerrain, TileTerrain, TCHardness};

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
	pub frequency: f32,
	/// Form of terrain has additional configuration.
	pub form: ProtocolTerrainForm,
	/// Count spawned on surface. Random range of how many are spawned before noise modifer.
	pub count_range: [u32; 2],
	/// Noise from center to outside. Minimum one, rest can be NONE.
	pub noise_lerp: [Option<ProtocolNoise>; 5],
}

impl ProtocolTerrain {
	pub fn new(
		name: &str,
		tile_color: [f32; 3],
		tile_tc_hardness: TCHardness,
		tile_work: u32,
		frequency: f32,
		count_range: [u32; 2],
		noise_lerp: &[ProtocolNoise],
		form: ProtocolTerrainForm,
	) -> Self {
		assert!(noise_lerp.len() <= 5);
		let mut pt_noise_lerp = Vec::with_capacity(noise_lerp.len());
		for nl in noise_lerp.iter() {
			pt_noise_lerp.push(Some(nl.clone()));
		}
		for _ in pt_noise_lerp.len()..5 {
			pt_noise_lerp.push(None);
		}
		Self {
			name: Some(String::from_str(name).unwrap()),
			tile: TileTerrain {
				texture_idx: 0,
				color: Vector3::from_column_slice(&tile_color),
				tc_hardness: tile_tc_hardness,
				work: tile_work,
			},
			frequency,
			form,
			count_range,
			noise_lerp: pt_noise_lerp.try_into().unwrap(),
		}
	}
	pub fn instantiate(
		&self,
	) -> Option<RTTerrain> {
		Some(RTTerrain::new(self.tile.clone()))
	}
}