//! Contains the [SolidColor] struct which implements [Texture];

use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

use super::Texture;

/// Represents a solid color. The simplest texture which is just a single color.
#[derive(Default)]
pub struct SolidColor {
	color: Color,
	transform: Matrix,
}

impl SolidColor {
	/// Returns a new texture which always returns `color`.
	pub fn new(color: Color) -> Self {
		Self {
			color,
			transform: Matrix::eye(),
		}
	}
}

impl Texture for SolidColor {
	fn transform(&self) -> &Matrix {
		&self.transform
	}

	fn transform_inverse(&self) -> &Matrix {
		&self.transform
	}

	fn set_transform(&mut self, _: Matrix) {}

	fn color_at(&self, _: Tuple) -> Color {
		self.color
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn color_at() {
		let color = Color::new(0.5, 0.5, 0.5);
		let texture = SolidColor::new(color);
		assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 0.0)), color);
		assert_eq!(texture.color_at(Tuple::point(0.0, 1.0, 0.0)), color);
		assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 1.0)), color);
		assert_eq!(texture.color_at(Tuple::point(0.0, -1.0, 0.0)), color);
		assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, -1.0)), color);
		assert_eq!(texture.color_at(Tuple::point(-2.0, 3.5, 5.2)), color);
	}
}
