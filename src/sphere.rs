//! Holds the [Sphere] struct;

use crate::ray::Ray;
use crate::tuple::Tuple;

/// A unit sphere centered at the origin.
#[derive(Default)]
pub struct Sphere;

impl Sphere {
	/// Returns a new sphere.
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns a sorted vector of all distances where `ray` intersects `self`.
	///
	/// # Examples
	///
	/// A ray intersects a sphere at two points.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let sphere = Sphere::new();
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 2);
	/// assert_eq!(intersections[0], 4.0);
	/// assert_eq!(intersections[1], 6.0);
	/// ```
	///
	/// A ray intersects a sphere at a tangent.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let sphere = Sphere::new();
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 2);
	/// assert_eq!(intersections[0], 5.0);
	/// assert_eq!(intersections[1], 5.0);
	/// ```
	///
	/// A ray misses a sphere.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let sphere = Sphere::new();
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 0);
	/// ```
	///
	/// A ray originates inside a sphere.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let sphere = Sphere::new();
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 2);
	/// assert_eq!(intersections[0], -1.0);
	/// assert_eq!(intersections[1], 1.0);
	/// ```
	///
	/// A ray is behind a sphere.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let sphere = Sphere::new();
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 2);
	/// assert_eq!(intersections[0], -6.0);
	/// assert_eq!(intersections[1], -4.0);
	/// ```
	pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
		let oc = ray.origin() - Tuple::point(0.0, 0.0, 0.0);

		let a = ray.direction().norm_squared();
		let b = ray.direction().dot(oc);
		let c = oc.norm_squared() - 1.0;

		let d = b * b - a * c;

		if d < 0.0 {
			return Vec::new();
		}

		let sqrtd = d.sqrt();
		let t1 = (-b - sqrtd) / a;
		let t2 = (-b + sqrtd) / a;

		vec![t1, t2]
	}
}
