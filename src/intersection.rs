//! Holds the [Intersection] struct.

use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::sphere::Sphere;

static ID: AtomicUsize = AtomicUsize::new(0);

/// Stores some information about intersections between rays and shapes.
/// Cloning is constant time and memory.
#[derive(Clone)]
pub struct Intersection {
	id: usize,
	t: f64,
	shape: Rc<Sphere>,
}

impl Intersection {
	/// Returns a new [Intersection].
	/// `t` is the distance between the ray origin and the intersection point.
	/// `shape` is the the shape which is intersected with.
	pub fn new(t: f64, shape: Rc<Sphere>) -> Self {
		Self {
			t,
			shape: Rc::clone(&shape),
			id: ID.fetch_add(1, AtomicOrdering::SeqCst),
		}
	}

	/// Returns `self`'s distance.
	pub fn t(&self) -> f64 {
		self.t
	}

	/// Returns `self`'s shape.
	pub fn shape(&self) -> Rc<Sphere> {
		Rc::clone(&self.shape)
	}
}

impl Debug for Intersection {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		writeln!(
			f,
			"{{ id: {}, t: {}, shape: <something> }}",
			self.id, self.t
		)
	}
}

impl PartialEq for Intersection {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.t.is_nan() || other.t.is_nan() {
			return None;
		}

		Some(if self.t < other.t {
			Less
		} else if self.t > other.t {
			Greater
		} else {
			Equal
		})
	}
}

/// Hit holds a hit function which should return an instance of `T` if there is a hit. None
/// otherwise.
/// Known implementations: [Vec<Intersection>].
pub trait Hit<T = Intersection> {
	fn hit(&self) -> Option<&T>;
}

/// Implement [Hit] to get the first intersection which is a hit. The list should be sorted. If
/// there are no hits, `None` is returned.
impl Hit for Vec<Intersection> {
	fn hit(&self) -> Option<&Intersection> {
		for i in self.iter() {
			if i.t > 0.0 {
				return Some(i);
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hit_all_positive() {
		let sphere = Rc::new(Sphere::new());
		let i1 = Intersection::new(1.0, Rc::clone(&sphere));
		let i2 = Intersection::new(2.0, Rc::clone(&sphere));
		let mut is = vec![Intersection::clone(&i1), i2];
		is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
		let i = is.hit().unwrap();
		assert_eq!(i, &i1);
	}

	#[test]
	fn hit_some_negative() {
		let sphere = Rc::new(Sphere::new());
		let i1 = Intersection::new(-1.0, Rc::clone(&sphere));
		let i2 = Intersection::new(1.0, Rc::clone(&sphere));
		let mut is = vec![Intersection::clone(&i2), i1];
		is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
		let i = is.hit().unwrap();
		assert_eq!(i, &i2);
	}

	#[test]
	fn hit_all_negative() {
		let sphere = Rc::new(Sphere::new());
		let i1 = Intersection::new(-2.0, Rc::clone(&sphere));
		let i2 = Intersection::new(-1.0, Rc::clone(&sphere));
		let mut is = vec![i2, i1];
		is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
		let i = is.hit();
		assert_eq!(i, None);
	}

	#[test]
	fn hit_many() {
		let sphere = Rc::new(Sphere::new());
		let i1 = Intersection::new(5.0, Rc::clone(&sphere));
		let i2 = Intersection::new(7.0, Rc::clone(&sphere));
		let i3 = Intersection::new(-3.0, Rc::clone(&sphere));
		let i4 = Intersection::new(2.0, Rc::clone(&sphere));
		let mut is = vec![Intersection::clone(&i4), i1, i2, i3];
		is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
		let i = is.hit().unwrap();
		assert_eq!(i, &i4);
	}
}
