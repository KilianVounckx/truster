//! Holds the [Material] struct.

use crate::color::Color;
use crate::light::PointLight;
use crate::tuple::Tuple;

/// Material with lighting properties. Give it to a shape to change its appearance.
#[derive(Clone)]
pub struct Material {
	pub color: Color,
	pub ambient: f64,
	pub diffuse: f64,
	pub specular: f64,
	pub shininess: f64,
}

impl Material {
	/// Shades the object. Returns the color they would emit at `position`. `light` is the light
	/// that is lighting the scene. `eye` is the direction of the 'eye' that is looking at the
	/// scene. `normal` is the normal vector of the shape that the material is on at `position`.
	/// `in_shadow` should be true if `position` is in a shadow of `light`.
	pub fn lighting(
		&self,
		light: &PointLight,
		position: Tuple,
		eye: Tuple,
		normal: Tuple,
		in_shadow: bool,
	) -> Color {
		let color = self.color * light.color();
		let lightv = (light.position() - position).normalized();
		let ambient = color * self.ambient;
		let light_dot_normal = lightv.dot(normal);

		if in_shadow || light_dot_normal < 0.0 {
			return ambient;
		}

		let diffuse = color * self.diffuse * light_dot_normal;
		let reflectv = (-lightv).reflect(normal);
		let reflect_dot_eye = reflectv.dot(eye);

		if reflect_dot_eye <= 0.0 {
			return ambient + diffuse;
		}

		let factor = reflect_dot_eye.powf(self.shininess);
		let specular = light.color() * self.specular * factor;

		return ambient + diffuse + specular;
	}
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lighting_eye_between_light_and_surface() {
		let material = Material::default();
		let position = Tuple::point(0.0, 0.0, 0.0);

		let eye = Tuple::vector(0.0, 0.0, -1.0);
		let normal = Tuple::vector(0.0, 0.0, -1.0);
		let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = material.lighting(&light, position, eye, normal, false);
		assert_eq!(result, Color::new(1.9, 1.9, 1.9));
	}

	#[test]
	fn lighting_eye_between_light_and_surface_light_offset_45deg() {
		let material = Material::default();
		let position = Tuple::point(0.0, 0.0, 0.0);

		let eye = Tuple::vector(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
		let normal = Tuple::vector(0.0, 0.0, -1.0);
		let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = material.lighting(&light, position, eye, normal, false);
		assert_eq!(result, Color::new(1.0, 1.0, 1.0));
	}

	#[test]
	fn lighting_eye_opposite_surface_light_offset_45deg() {
		let material = Material::default();
		let position = Tuple::point(0.0, 0.0, 0.0);

		let eye = Tuple::vector(0.0, 0.0, -1.0);
		let normal = Tuple::vector(0.0, 0.0, -1.0);
		let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = material.lighting(&light, position, eye, normal, false);
		assert_eq!(
			result,
			Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
		);
	}

	#[test]
	fn lighting_eye_in_path_reflector() {
		let material = Material::default();
		let position = Tuple::point(0.0, 0.0, 0.0);

		let eye = Tuple::vector(0.0, -(2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
		let normal = Tuple::vector(0.0, 0.0, -1.0);
		let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = material.lighting(&light, position, eye, normal, false);
		assert_eq!(
			result,
			Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
		);
	}

	#[test]
	fn lighting_light_behind_surface() {
		let material = Material::default();
		let position = Tuple::point(0.0, 0.0, 0.0);

		let eye = Tuple::vector(0.0, 0.0, -1.0);
		let normal = Tuple::vector(0.0, 0.0, -1.0);
		let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

		let result = material.lighting(&light, position, eye, normal, false);
		assert_eq!(result, Color::new(0.1, 0.1, 0.1));
	}

	#[test]
	fn lighting_surface_in_shadow() {
		let material = Material::default();
		let position = Tuple::point(0.0, 0.0, 0.0);

		let eye = Tuple::vector(0.0, 0.0, -1.0);
		let normal = Tuple::vector(0.0, 0.0, -1.0);
		let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = material.lighting(&light, position, eye, normal, true);
		assert_eq!(result, Color::new(0.1, 0.1, 0.1));
	}
}
