//! Holds the [Ray] struct.

use crate::tuple::Tuple;

/// A ray which can be used for calculating intersections with shapes to render 3D scenes.
pub struct Ray {
	origin: Tuple,
	direction: Tuple,
}

impl Ray {
	/// Returns a new [Ray] with the given origin and direction.
	pub fn new(origin: Tuple, direction: Tuple) -> Self {
		Self { origin, direction }
	}

	/// Returns `self`'s origin.
	pub fn origin(&self) -> Tuple {
		self.origin
	}

	/// Returns `self`'s direction.
	pub fn direction(&self) -> Tuple {
		self.direction
	}

	/// Returns the point at the given distance t along `self`.
	///
	/// # Examples
	///
	/// ```
	/// # use rtc::ray::Ray;
	/// use rtc::tuple::Tuple;
	///
	/// let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
	/// assert_eq!(ray.at(0.0), Tuple::point(2.0, 3.0, 4.0));
	/// assert_eq!(ray.at(1.0), Tuple::point(3.0, 3.0, 4.0));
	/// assert_eq!(ray.at(-1.0), Tuple::point(1.0, 3.0, 4.0));
	/// assert_eq!(ray.at(2.5), Tuple::point(4.5, 3.0, 4.0));
	/// ```
	pub fn at(&self, t: f64) -> Tuple {
		self.origin + self.direction() * t
	}
}
