use std::str::FromStr;

use crate::ProtocolEntityForm;

#[derive(Clone)]
pub struct ProtocolEntity {
	pub name: Option<String>,
	pub form: ProtocolEntityForm,
	pub texture_idx: u32,
}

impl ProtocolEntity {
	pub fn new(
		name: &str,
		form: ProtocolEntityForm,
	) -> Self {
		Self {
			name: Some(String::from_str(name).unwrap()),
			form,
			texture_idx: 0,
		}
	}
}