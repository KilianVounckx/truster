//! Holds the [Texture] trait, as well as some common textures which implement it.

use crate::color::Color;
use crate::matrix::Matrix;
use crate::shape::Shape;
use crate::tuple::Tuple;

pub mod solid_color;
pub mod stripe;

/// A basic texture implementation. There is no UV mapping or anything like that. The method
/// [Texture::color_at] should just map a point in 3D space to a color. Textures can be
/// transformed, so color_at_shape should return the color as if the texture were not transformed.
/// [Texture::color_at_shape] will perform the transformation, so it should not be overwritten.
/// [Texture::transform] should return the texture transform matrix,
/// [Texture::transform_inverse] should return it's inverse. [Texture::set_transform] should set
/// the texture transform to be `transform`.
pub trait Texture {
	fn color_at_shape(&self, point: Tuple, shape: &dyn Shape) -> Color {
		let point = shape.transform_inverse() * point;
		let point = self.transform_inverse() * point;
		self.color_at(point)
	}
	fn color_at_texture(&self, point: Tuple) -> Color {
		let point = self.transform_inverse() * point;
		self.color_at(point)
	}
	fn color_at(&self, point: Tuple) -> Color;

	fn transform(&self) -> &Matrix;
	fn transform_inverse(&self) -> &Matrix;
	fn set_transform(&mut self, transform: Matrix);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shape::sphere::Sphere;

	struct MockTexture {
		transform: Matrix,
		transform_inverse: Matrix,
	}

	impl MockTexture {
		fn new() -> Self {
			Self {
				transform: Matrix::eye(),
				transform_inverse: Matrix::eye(),
			}
		}
	}

	impl Texture for MockTexture {
		fn color_at(&self, point: Tuple) -> Color {
			Color::new(point.x(), point.y(), point.z())
		}

		fn transform(&self) -> &Matrix {
			&self.transform
		}

		fn transform_inverse(&self) -> &Matrix {
			&self.transform_inverse
		}

		fn set_transform(&mut self, transform: Matrix) {
			self.transform_inverse = transform.inverse();
			self.transform = transform;
		}
	}

	#[test]
	fn color_at_shape_with_shape_transformation() {
		let mut shape = Sphere::new();
		shape.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
		let texture = MockTexture::new();
		let color = texture.color_at_shape(Tuple::point(2.0, 3.0, 4.0), &shape);
		assert_eq!(color, Color::new(1.0, 1.5, 2.0));
	}

	#[test]
	fn color_at_shape_with_texture_transformation() {
		let shape = Sphere::new();
		let mut texture = MockTexture::new();
		texture.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
		let color = texture.color_at_shape(Tuple::point(2.0, 3.0, 4.0), &shape);
		assert_eq!(color, Color::new(1.0, 1.5, 2.0));
	}

	#[test]
	fn color_at_shape_with_shape_and_texture_transformation() {
		let mut shape = Sphere::new();
		shape.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
		let mut texture = MockTexture::new();
		texture.set_transform(Matrix::translation(0.5, 1.0, 1.5));
		let color = texture.color_at_shape(Tuple::point(2.5, 3.0, 3.5), &shape);
		assert_eq!(color, Color::new(0.75, 0.5, 0.25));
	}
}
