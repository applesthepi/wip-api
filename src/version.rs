#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
}

impl Version {
	pub const fn new(
		major: u8,
		minor: u8,
		patch: u8,
	) -> Self {
		Self {
			major,
			minor,
			patch,
		}
	}
}

#[repr(C)]
pub enum RegisterStatus {
	Success,
	Error,
}

impl Version {
	pub fn from_str(
		text: &str,
	) -> Option<Self> {
		let mut chars = text.chars().into_iter();
		let mut staging = String::with_capacity(3);
		let mut part: usize = 0;
		let mut parts: [u8; 3] = [0; 3];
		loop {
			let c = match chars.next() {
				Some(x) => x,
				None => {
					if part < 2 {
						return None;
					}
					parts[part] = match staging.parse() {
						Ok(x) => x,
						Err(_) => { return None; },
					};	
					return Some(Self {
						major: parts[0],
						minor: parts[1],
						patch: parts[2],
					});
				},
			};
			if c == '.' {
				parts[part] = match staging.parse() {
					Ok(x) => x,
					Err(_) => { return None; },
				};
				part += 1;
				staging.clear();
			} else {
				staging.extend_one(c);
			}
		}
	}
}