use std::{collections::HashMap, str::FromStr, sync::Arc};

pub struct ModState {
	name: String,
	textures: Arc<HashMap<String, u32>>,
}

impl ModState {
	pub fn new(
		name: &str,
		textures: Arc<HashMap<String, u32>>,
	) -> Arc<Self> {
		let mut mod_name = String::with_capacity(name.len() + 1);
		mod_name += name;
		mod_name += "_";
		Arc::new(Self {
			name: mod_name,
			textures,
		})
	}

	pub fn texture(
		&self,
		full_name: &str,
	) -> u32 {
		*self.textures.get(full_name).expect(
			&format!("failed to retrive texture (mod_texture) \"{}\" from mod (mod) \"{}\"", full_name, &self.name),
		)
	}
}