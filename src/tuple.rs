#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
	x: f64,
	y: f64,
	z: f64,
	w: f64,
}

impl Tuple {
	fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
		Self { x, y, z, w }
	}

	pub fn point(x: f64, y: f64, z: f64) -> Self {
		Self::new(x, y, z, 1.0)
	}

	pub fn vector(x: f64, y: f64, z: f64) -> Self {
		Self::new(x, y, z, 0.0)
	}

	pub fn x(&self) -> f64 {
		self.x
	}

	pub fn y(&self) -> f64 {
		self.y
	}

	pub fn z(&self) -> f64 {
		self.z
	}

	pub fn is_point(&self) -> bool {
		self.w == 1.0
	}

	pub fn is_vector(&self) -> bool {
		self.w == 0.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn point_vs_vector() {
		let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
		assert_eq!(a.x(), 4.3);
		assert_eq!(a.y(), -4.2);
		assert_eq!(a.z(), 3.1);
		assert!(a.is_point(), "{:?}.is_point() == false, want true", a);
		assert!(!a.is_vector(), "{:?}.is_vector() == true, want false", a);
		assert_eq!(Tuple::point(4.3, -4.2, 3.1), a);

		let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
		assert_eq!(a.x(), 4.3);
		assert_eq!(a.y(), -4.2);
		assert_eq!(a.z(), 3.1);
		assert!(!a.is_point(), "{:?}.is_vector() == true, want false", a);
		assert!(a.is_vector(), "{:?}.is_point() == false, want true", a);
		assert_eq!(Tuple::vector(4.3, -4.2, 3.1), a);
	}
}
