#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
	pub beta: u8,
}

impl Version {
	pub const fn new(
		major: u8,
		minor: u8,
		beta: u8,
	) -> Self {
		Self {
			major,
			minor,
			beta,
		}
	}
}

pub enum RegisterStatus {
	Success,
	Error,
}