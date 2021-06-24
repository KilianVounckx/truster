//! Holds the [Material] struct.

use crate::color::Color;

/// Material with lighting properties. Give it to a shape to change its appearance.
pub struct Material {
	pub color: Color,
	pub ambient: f64,
	pub diffuse: f64,
	pub specular: f64,
	pub shininess: f64,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			color: Color::new(1.0, 1.0, 1.0),
			ambient: 0.1,
			diffuse: 0.9,
			specular: 0.9,
			shininess: 200.0,
		}
	}
}
