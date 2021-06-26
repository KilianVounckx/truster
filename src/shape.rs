//! Stores the [Shape] trait, as well as modules containing its implementation.

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub mod sphere;

/// Represents a 3D shape with all methods to be able to render it, as well as methods for
/// transforming it, and giving it a material.
///
/// [Shape::transform] should return the shape's transform. [Shape::set_transform] should set it's
/// transform. [Shape::transform_inverse] should return the shape's transform's inverse.
///
/// [Shape::material] should return the shape's material. [Shape::set_material] should set it's
/// material.
///
/// [Shape::local_intersect] should return a list of **all** intersections `ray` makes with the
/// implementation of [Shape]. The list should be sorted according to the distances (`t` value) of
/// the intersections. Intersections behind `ray` should also be in the list, but with a negative
/// distance. This means they are at the front of the list. The intersections should be in local
/// space. This means they should be calculated as if the shape where not transformed. The
/// calculations for the transformation happen in [Shape::intersect], which should not be
/// overwritten.
///
/// [Shape::local_normal_at] should return the surface normal of shape at `point`. The caller is
/// responsible for making sure `point` is on the surface the shape. The resulting vector should
/// be normalized. The normal should be in local space. This means they should be calculated as if
/// the shape where not transformed. The calculations for the transformation happen in
/// [Shape::normal_at], which should not be overwritten.
pub trait Shape {
	fn transform(&self) -> &Matrix;
	fn set_transform(&mut self, transform: Matrix);
	fn transform_inverse(&self) -> &Matrix;

	fn material(&self) -> &Material;
	fn set_material(&mut self, material: Material);

	fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
	fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
		self.local_intersect(&ray.transform(&self.transform_inverse()))
	}

	fn local_normal_at(&self, point: Tuple) -> Tuple;
	fn normal_at(&self, point: Tuple) -> Tuple {
		let point = self.transform_inverse() * point;
		let normal = self.local_normal_at(point);
		let normal = &self.transform_inverse().transpose() * normal;
		let normal = Tuple::vector(normal.x(), normal.y(), normal.z());
		normal.normalized()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::RefCell;

	struct MockShape {
		transform: Matrix,
		transform_inverse: Matrix,
		material: Material,
		saved_ray: RefCell<Ray>,
	}

	impl MockShape {
		fn new() -> Self {
			Self {
				transform: Matrix::default(),
				transform_inverse: Matrix::default(),
				material: Material::default(),
				saved_ray: RefCell::new(Ray::new(
					Tuple::vector(0.0, 0.0, 0.0),
					Tuple::vector(0.0, 0.0, 0.0),
				)),
			}
		}
	}

	impl Shape for MockShape {
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

		fn material(&self) -> &Material {
			&self.material
		}
		fn set_material(&mut self, material: Material) {
			self.material = material
		}

		fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
			*self.saved_ray.borrow_mut() = ray.clone();
			Vec::new()
		}

		fn local_normal_at(&self, point: Tuple) -> Tuple {
			Tuple::vector(point.x(), point.y(), point.z())
		}
	}

	#[test]
	fn intersect_scaled() {
		let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let mut shape = MockShape::new();
		shape.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
		shape.intersect(&ray);
		assert_eq!(
			*shape.saved_ray.borrow(),
			Ray::new(Tuple::point(0.0, 0.0, -2.5), Tuple::vector(0.0, 0.0, 0.5))
		);
	}

	#[test]
	fn intersect_translated() {
		let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let mut shape = MockShape::new();
		shape.set_transform(Matrix::translation(5.0, 0.0, 0.0));
		shape.intersect(&ray);
		assert_eq!(
			*shape.saved_ray.borrow(),
			Ray::new(Tuple::point(-5.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0))
		);
	}

	#[test]
	fn normal_at_translated() {
		let mut shape = MockShape::new();
		shape.set_transform(Matrix::translation(0.0, 1.0, 0.0));
		let normal = shape.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
		assert_eq!(
			normal,
			Tuple::vector(0.0, 0.7071067811865475, -0.7071067811865476)
		);
	}

	#[test]
	fn normal_at_transformed() {
		let mut shape = MockShape::new();
		shape.set_transform(
			Matrix::scaling(1.0, 0.5, 1.0) * &Matrix::rotation_z(std::f64::consts::PI / 5.0),
		);
		let normal = shape.normal_at(Tuple::point(0.0, 0.70711, -0.70711));
		assert_eq!(
			normal,
			Tuple::vector(
				0.00000000000000003808016223885823,
				0.9701425001453319,
				-0.24253562503633297
			)
		);
	}
}
