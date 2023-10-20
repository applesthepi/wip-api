use num::Float;
use num::NumCast;

mod ids;
mod world_op_states;
mod rendering;
mod types;

pub use ids::*;
pub use world_op_states::*;
pub use rendering::*;
pub use types::*;

/// Converts from [0.0, 1.0] to [-1.0, 1.0]
/// # Safty
/// Safe because the unsafe is for hard coded vales. The
/// compiler forces T to allow these operations.
pub fn op_01_11<T: Float>(t: T) -> T { unsafe {
	let offset = NumCast::from(0.5).unwrap_unchecked();
	let scale = NumCast::from(2.0).unwrap_unchecked();
	(t - offset) * scale
}}

/// Converts from [-1.0, 1.0] to [0.0, 1.0]
/// # Safty
/// Safe because the unsafe is for hard coded vales. The
/// compiler forces T to allow these operations.
pub fn op_11_01<T: Float>(t: T) -> T { unsafe {
	let offset = NumCast::from(0.5).unwrap_unchecked();
	let scale = NumCast::from(2.0).unwrap_unchecked();
	t / scale + offset
}}

/// Converts from [-1.0, 1.0] to [0.0, 3.0]
/// # Safty
/// Safe because the unsafe is for hard coded vales. The
/// compiler forces T to allow these operations.
pub fn op_11_03<T: Float>(t: T) -> T { unsafe {
	let offset = NumCast::from(0.5).unwrap_unchecked();
	let scale = NumCast::from(2.0).unwrap_unchecked();
	let scale_03 = NumCast::from(3.0).unwrap_unchecked();
	(t / scale + offset) * scale_03
}}

/// Converts from [0.0, 3.0] to [0.0, 1.0]
/// # Safty
/// Safe because the unsafe is for hard coded vales. The
/// compiler forces T to allow these operations.
pub fn op_03_01<T: Float>(t: T) -> T { unsafe {
	let scale = NumCast::from(3.0).unwrap_unchecked();
	t / scale
}}