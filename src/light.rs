//! Holds the [PointLight] struct.

use crate::color::Color;
use crate::tuple::Tuple;

/// Light at a point in 3D space with a color.
pub struct PointLight {
	position: Tuple,
	color: Color,
}

impl PointLight {
	pub fn new(position: Tuple, color: Color) -> Self {
		Self { position, color }
	}
}
