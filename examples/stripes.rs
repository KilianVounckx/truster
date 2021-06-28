use std::error::Error;
use std::f64::consts::PI;
use std::io;
use std::rc::Rc;

use truster::camera::{Camera, Config};
use truster::color::Color;
use truster::light::PointLight;
use truster::material::Material;
use truster::matrix::Matrix;
use truster::shape::{plane::Plane, sphere::Sphere, Shape};
use truster::texture::{stripe::Stripe, Texture};
use truster::tuple::Tuple;
use truster::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = World::new();

    let mut floor = Plane::new();
    floor.set_material(Material {
        texture: Rc::new(Stripe::colors(
            Color::new(0.1, 0.8, 0.3),
            Color::new(0.1, 0.3, 0.8),
        )),
        ..Material::default()
    });
    world.add_shape(Rc::new(floor));

    let mut wall = Plane::new();
    wall.set_transform(Matrix::rotation_x(PI / 2.0));
    wall.set_material(Material {
        texture: Rc::new(Stripe::colors(
            Color::new(0.1, 0.8, 0.3),
            Color::new(0.1, 0.3, 0.8),
        )),
        ..Material::default()
    });
    world.add_shape(Rc::new(wall));

    let mut ball_text = Stripe::colors(Color::new(0.8, 0.3, 0.1), Color::new(0.7, 0.4, 0.1));
    ball_text.set_transform(Matrix::rotation_y(PI / 4.0) * &Matrix::scaling(0.1, 0.1, 0.1));
    let mut ball = Sphere::new();
    ball.set_transform(Matrix::translation(0.0, 2.0, 2.0) * &Matrix::scaling(0.75, 0.75, 0.75));
    ball.set_material(Material {
        texture: Rc::new(ball_text),
        ..Material::default()
    });
    world.add_shape(Rc::new(ball));

    let light = PointLight::new(Tuple::point(-5.0, 10.0, 5.0), Color::new(1.0, 1.0, 1.0));
    world.add_light(Rc::new(light));

    let camera = Camera::new(Config {
        hsize: 1600,
        vsize: 900,
        from: Tuple::point(0.0, 5.0, 10.0),
        at: Tuple::point(0.0, 2.0, 0.0),
        ..Config::default()
    });

    let image = camera.render(&world);
    image.to_ppm(&mut io::stdout())?;

    Ok(())
}
