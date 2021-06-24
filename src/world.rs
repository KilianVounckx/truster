//! Holds the [World] struct.

use std::rc::Rc;

use crate::color::Color;
use crate::intersection::{Hit, HitRecord, Intersection};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;

/// A 3D world which has shapes and lights.
#[derive(Default)]
pub struct World {
	shapes: Vec<Rc<Sphere>>,
	lights: Vec<Rc<PointLight>>,
}

impl World {
	/// Returns a new empty [World].
	pub fn new() -> Self {
		Self::default()
	}

	/// Adds `shape` to `self`.
	pub fn add_shape(&mut self, shape: Rc<Sphere>) {
		self.shapes.push(shape);
	}

	/// Adds `light` to `self`.
	pub fn add_light(&mut self, light: Rc<PointLight>) {
		self.lights.push(light)
	}

	/// Returns a list of all intersections the ray makes with any shape in the world.
	/// The list is sorted by distance.
	pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
		let mut result = Vec::new();
		for shape in self.shapes.iter() {
			let mut intersections = shape.intersect(ray);
			result.append(&mut intersections);
		}
		result.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
		result
	}

	/// Returns the color at the intersection encapsulated by `rec` in `self`.
	pub fn shade_hit(&self, light_index: usize, rec: HitRecord) -> Color {
		rec.shape().material().lighting(
			&self.lights[light_index],
			rec.point(),
			rec.eye(),
			rec.normal(),
		)
	}

	/// Returns the color the `self` shows at the intersection point with `ray`.
	pub fn color_at(&self, ray: &Ray) -> Color {
		let intersections = self.intersect(ray);
		let hit = if let Some(hit) = intersections.hit() {
			hit
		} else {
			return Color::new(0.0, 0.0, 0.0);
		};

		let rec = HitRecord::new(&hit, ray);
		let mut result = Color::new(0.0, 0.0, 0.0);
		for (i, _) in self.lights.iter().enumerate() {
			let color = self.shade_hit(i, HitRecord::clone(&rec));
			result += color;
		}
		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::color::Color;
	use crate::material::Material;
	use crate::matrix::Matrix;
	use crate::tuple::Tuple;

	fn test_world() -> World {
		let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let mut sphere1 = Sphere::new();
		sphere1.set_material(Material {
			color: Color::new(0.8, 1.0, 0.6),
			diffuse: 0.7,
			specular: 0.2,
			..Material::default()
		});

		let mut sphere2 = Sphere::new();
		sphere2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

		let mut world = World::new();
		world.add_light(Rc::new(light));
		world.add_shape(Rc::new(sphere1));
		world.add_shape(Rc::new(sphere2));

		world
	}

	#[test]
	fn intersect() {
		let world = test_world();
		let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let intersections = world.intersect(&ray);

		assert_eq!(intersections.len(), 4);
		assert_eq!(intersections[0].t(), 4.0);
		assert_eq!(intersections[1].t(), 4.5);
		assert_eq!(intersections[2].t(), 5.5);
		assert_eq!(intersections[3].t(), 6.0);
	}

	#[test]
	fn shade_hit() {
		let world = test_world();
		let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let shape = Rc::clone(&world.shapes[0]);
		let intersection = Intersection::new(4.0, shape);
		let rec = HitRecord::new(&intersection, &ray);
		let color = world.shade_hit(0, rec);
		assert_eq!(
			color,
			Color::new(
				0.38066119308103435,
				0.47582649135129296,
				0.28549589481077575,
			),
		);
	}

	#[test]
	fn shade_hit_inside() {
		let mut world = test_world();
		world.lights[0] = Rc::new(PointLight::new(
			Tuple::point(0.0, 0.25, 0.0),
			Color::new(1.0, 1.0, 1.0),
		));
		let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
		let shape = Rc::clone(&world.shapes[1]);
		let intersection = Intersection::new(0.5, shape);
		let rec = HitRecord::new(&intersection, &ray);
		let color = world.shade_hit(0, rec);
		assert_eq!(
			color,
			Color::new(0.9049844720832575, 0.9049844720832575, 0.9049844720832575),
		);
	}

	#[test]
	fn color_at_miss() {
		let world = test_world();
		let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
		let color = world.color_at(&ray);
		assert_eq!(color, Color::new(0.0, 0.0, 0.0));
	}

	#[test]
	fn color_at_hit() {
		let world = test_world();
		let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let color = world.color_at(&ray);
		assert_eq!(
			color,
			Color::new(
				0.38066119308103435,
				0.47582649135129296,
				0.28549589481077575,
			),
		);
	}
}
