//! Holds the [Sphere] struct;

use std::rc::Rc;

use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

/// A 3D ellipsoid (spheroid).
#[derive(Default, Clone)]
pub struct Sphere {
	transform: Matrix,
	transform_inverse: Matrix,
}

impl Sphere {
	/// Returns a new sphere with radius 1, centered at the origin.
	/// Use [Sphere::set_transform] to transform it.
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
	/// assert_eq!(intersections[0].t(), 4.0);
	/// assert_eq!(intersections[1].t(), 6.0);
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
	/// assert_eq!(intersections[0].t(), 5.0);
	/// assert_eq!(intersections[1].t(), 5.0);
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
	/// assert_eq!(intersections[0].t(), -1.0);
	/// assert_eq!(intersections[1].t(), 1.0);
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
	/// assert_eq!(intersections[0].t(), -6.0);
	/// assert_eq!(intersections[1].t(), -4.0);
	/// ```
	///
	/// Intersecting a scaled sphere with a ray.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	/// use rtc::matrix::Matrix;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let mut sphere = Sphere::new();
	/// sphere.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 2);
	/// assert_eq!(intersections[0].t(), 3.0);
	/// assert_eq!(intersections[1].t(), 7.0);
	/// ```
	///
	/// Intersecting a translated sphere with a ray.
	/// ```
	/// # use rtc::sphere::Sphere;
	/// use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	/// use rtc::matrix::Matrix;
	///
	/// let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
	/// let mut sphere = Sphere::new();
	/// sphere.set_transform(Matrix::translation(5.0, 0.0, 0.0));
	/// let intersections = sphere.intersect(&ray);
	/// assert_eq!(intersections.len(), 0);
	/// ```
	pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
		let ray = ray.transform(&self.transform_inverse);

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

		vec![
			Intersection::new(t1, Rc::new(self.clone())),
			Intersection::new(t2, Rc::new(self.clone())),
		]
	}

	/// Sets `self`'s transform to be `transform`.
	pub fn set_transform(&mut self, transform: Matrix) {
		self.transform_inverse = transform.inverse();
		self.transform = transform;
	}

	/// Returns `self`'s transform.
	pub fn transform(&self) -> &Matrix {
		&self.transform
	}

	/// Returns `self`'s transform's inverse.
	pub fn transform_inverse(&self) -> &Matrix {
		&self.transform_inverse
	}
}
