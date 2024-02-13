use std::str::FromStr;

use crate::ProtocolEntityForm;

#[derive(Clone)]
pub struct ProtocolEntity {
	pub un_protocol: String,
	pub form: ProtocolEntityForm,
	pub texture_idx: u32,
}

impl ProtocolEntity {
	pub fn new(
		un_protocol: impl Into<String>,
		form: ProtocolEntityForm,
	) -> Self {
		Self {
			un_protocol: un_protocol.into(),
			form,
			texture_idx: 0,
		}
	}
}