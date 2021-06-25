use std::f64::consts::PI;
use std::rc::Rc;

use truster::camera::{Camera, Config};
use truster::color::Color;
use truster::light::PointLight;
use truster::material::Material;
use truster::matrix::Matrix;
use truster::sphere::Sphere;
use truster::tuple::Tuple;
use truster::world::World;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut world = World::new();

	let mut floor = Sphere::new();
	floor.set_transform(Matrix::scaling(10.0, 0.01, 10.0));
	floor.set_material(Material {
		color: Color::new(1.0, 0.9, 0.9),
		specular: 0.0,
		..Material::default()
	});
	world.add_shape(Rc::new(floor));

	let mut left_wall = Sphere::new();
	left_wall.set_transform(
		Matrix::translation(0.0, 0.0, 5.0)
			* &Matrix::rotation_y(-PI / 4.0)
			* &Matrix::rotation_x(PI / 2.0)
			* &Matrix::scaling(10.0, 0.01, 10.0),
	);
	left_wall.set_material(Material {
		color: Color::new(1.0, 0.9, 0.9),
		specular: 0.0,
		..Material::default()
	});
	world.add_shape(Rc::new(left_wall));

	let mut right_wall = Sphere::new();
	right_wall.set_transform(
		Matrix::translation(0.0, 0.0, 5.0)
			* &Matrix::rotation_y(PI / 4.0)
			* &Matrix::rotation_x(PI / 2.0)
			* &Matrix::scaling(10.0, 0.01, 10.0),
	);
	right_wall.set_material(Material {
		color: Color::new(1.0, 0.9, 0.9),
		specular: 0.0,
		..Material::default()
	});
	world.add_shape(Rc::new(right_wall));

	let mut middle = Sphere::new();
	middle.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
	middle.set_material(Material {
		color: Color::new(0.1, 1.0, 0.5),
		diffuse: 0.7,
		specular: 0.3,
		..Material::default()
	});
	world.add_shape(Rc::new(middle));

	let mut right = Sphere::new();
	right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5));
	right.set_material(Material {
		color: Color::new(0.5, 1.0, 0.1),
		diffuse: 0.7,
		specular: 0.3,
		..Material::default()
	});
	world.add_shape(Rc::new(right));

	let mut left = Sphere::new();
	left.set_transform(Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33));
	left.set_material(Material {
		color: Color::new(1.0, 0.7, 0.1),
		diffuse: 0.7,
		specular: 0.3,
		..Material::default()
	});
	world.add_shape(Rc::new(left));

	let light1 = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(0.5, 0.5, 0.5));
	world.add_light(Rc::new(light1));

	let light2 = PointLight::new(Tuple::point(10.0, 10.0, -10.0), Color::new(0.5, 0.5, 0.5));
	world.add_light(Rc::new(light2));

	let camera = Camera::new(Config {
		hsize: 1000,
		vsize: 500,
		from: Tuple::point(0.0, 1.5, -5.0),
		at: Tuple::point(0.0, 1.0, 0.0),
		..Config::default()
	});

	let canvas = camera.render(&world);
	canvas.to_ppm(&mut std::io::stdout())?;

	Ok(())
}
