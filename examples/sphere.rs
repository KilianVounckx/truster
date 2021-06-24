use rtc::canvas::Canvas;
use rtc::color::Color;
use rtc::intersection::Hit;
use rtc::matrix::Matrix;
use rtc::ray::Ray;
use rtc::sphere::Sphere;
use rtc::tuple::Tuple;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let ray_origin = Tuple::point(0.0, 0.0, -5.0);
	let wall_z = 10.0;
	let wall_size = 7.0;

	let canvas_pixels: usize = 100;
	let pixel_size = wall_size / canvas_pixels as f64;
	let half = wall_size / 2.0;

	let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
	let color = Color::new(1.0, 0.0, 0.0);
	let mut shape = Sphere::new();
	shape.set_transform(
		Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &Matrix::scaling(0.5, 1.0, 1.0),
	);

	for y in (0..canvas_pixels).rev() {
		let world_y = half - pixel_size * y as f64;

		for x in 0..canvas_pixels {
			let world_x = pixel_size * x as f64 - half;

			let position = Tuple::point(world_x, world_y, wall_z);

			let ray = Ray::new(ray_origin, (position - ray_origin).normalized());
			let intersections = shape.intersect(&ray);

			if intersections.hit() != None {
				canvas[[x, y]] = color;
			}
		}
	}

	canvas.to_ppm(&mut std::io::stdout())?;

	Ok(())
}
