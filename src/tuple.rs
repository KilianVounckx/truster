use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl Add for Tuple {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self::new(
			self.x + rhs.x,
			self.y + rhs.y,
			self.z + rhs.z,
			self.w + rhs.w,
		)
	}
}

impl AddAssign for Tuple {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
		self.w += rhs.w;
	}
}

impl Sub for Tuple {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self::new(
			self.x - rhs.x,
			self.y - rhs.y,
			self.z - rhs.z,
			self.w - rhs.w,
		)
	}
}

impl SubAssign for Tuple {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
		self.w -= rhs.w;
	}
}

impl Mul<f64> for Tuple {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self {
		Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
	}
}

impl MulAssign<f64> for Tuple {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
		self.w *= rhs;
	}
}

impl Div<f64> for Tuple {
	type Output = Self;

	fn div(self, rhs: f64) -> Self {
		Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
	}
}

impl DivAssign<f64> for Tuple {
	fn div_assign(&mut self, rhs: f64) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
		self.w /= rhs;
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
