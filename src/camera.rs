//! Holds the [Camera] struct.

use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;

/// Used for initializing a [Camera].
pub struct Config {
    /// The horizontal number of pixels.
    pub hsize: usize,
    /// The vertical number of pixels.
    pub vsize: usize,
    /// The vertical field of view angle in radians.
    pub fov: f64,
    /// The point from which the camera will be looking.
    pub from: Tuple,
    /// The point at which the camera will be looking.
    pub at: Tuple,
    /// The up direction for the camera.
    pub up: Tuple,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hsize: 100,
            vsize: 100,
            fov: PI / 3.0,
            from: Tuple::point(0.0, 0.0, 0.0),
            at: Tuple::point(0.0, 0.0, -1.0),
            up: Tuple::vector(0.0, 1.0, 0.0),
        }
    }
}

/// Represents a camera which can be used together with an instance of [crate::world::World] to render
/// a scene.
pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform_inverse: Matrix,
}

impl Camera {
    /// Returns a new [Camera] corresponding to `cfg`.
    pub fn new(cfg: Config) -> Self {
        let transform = Matrix::view_transform(cfg.from, cfg.at, cfg.up);
        let transform_inverse = transform.inverse();

        let half_view = (cfg.fov / 2.0).tan();
        let aspect = cfg.hsize as f64 / cfg.vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / cfg.hsize as f64;

        Self {
            hsize: cfg.hsize,
            vsize: cfg.vsize,
            half_height,
            half_width,
            pixel_size,
            transform_inverse,
        }
    }

    /// Returns a ray for the pixel at the given coordinates.
    ///
    /// # Examples
    ///
    /// Constructing a ray through the center of the canvas
    /// ```
    /// # use truster::camera::{Camera, Config};
    /// use std::f64::consts::PI;
    /// use truster::ray::Ray;
    /// use truster::tuple::Tuple;
    ///
    /// let camera = Camera::new(Config {
    ///     hsize: 201,
    ///     vsize: 101,
    ///     fov: PI / 2.0,
    ///     ..Config::default()
    /// });
    /// let ray = camera.ray_for_pixel(100, 50);
    /// // ray == Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, -1.0))); // approximately
    /// # assert_eq!(ray, Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.00000000000000011102230246251565, 0.0, -1.0)));
    /// ```
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let offset_x = (x as f64 + 0.5) * self.pixel_size;
        let offset_y = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        let pixel = &self.transform_inverse * Tuple::point(world_x, world_y, -1.0);
        let origin = &self.transform_inverse * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    /// Renders the `world` to a canvas as seen from `self` and returns it.
    pub fn render(&self, world: &World) -> Canvas {
        let mut result = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                result[[x, y]] = color;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::shape::{sphere::Sphere, Shape};
    use crate::texture::solid_color::SolidColor;
    use std::rc::Rc;

    #[test]
    fn pixel_size() {
        let camera = Camera::new(Config {
            hsize: 200,
            vsize: 125,
            fov: PI / 2.0,
            ..Config::default()
        });
        assert_eq!(camera.pixel_size, 0.009999999999999998);

        let camera = Camera::new(Config {
            hsize: 125,
            vsize: 200,
            fov: PI / 2.0,
            ..Config::default()
        });
        assert_eq!(camera.pixel_size, 0.009999999999999998);
    }

    #[test]
    fn ray_for_pixel_corner() {
        let camera = Camera::new(Config {
            hsize: 201,
            vsize: 101,
            fov: PI / 2.0,
            ..Config::default()
        });
        let ray = camera.ray_for_pixel(0, 0);
        assert_eq!(
            ray,
            Ray::new(
                Tuple::point(0.0, 0.0, 0.0),
                Tuple::vector(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
            )
        );
    }

    fn test_world() -> World {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut sphere1 = Sphere::new();
        sphere1.set_material(Material {
            texture: Rc::new(SolidColor::new(Color::new(0.8, 1.0, 0.6))),
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
    fn render() {
        let world = test_world();
        let camera = Camera::new(Config {
            hsize: 11,
            vsize: 11,
            fov: PI / 2.0,
            from: Tuple::point(0.0, 0.0, -5.0),
            at: Tuple::point(0.0, 0.0, 0.0),
            ..Config::default()
        });
        let image = camera.render(&world);
        assert_eq!(
            image[[5, 5]],
            Color::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }
}
