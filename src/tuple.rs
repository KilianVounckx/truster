//! A 3D tuple which can represent points and vectors.
//! The coordinates are floating point numbers. Support for generics may be added in the future.
//!
//! # Examples
//!
//! You can create points and vectors with [Tuple::point] and [Tuple::vector] respectively:
//! ```
//! # use rtc::tuple::Tuple;
//! let p = Tuple::point(1.0, 4.2, -3.7);
//! assert!(p.is_point());
//! assert!(!p.is_vector());
//! let v = Tuple::vector(1.0, 4.2, -3.7);
//! assert!(!v.is_point());
//! assert!(v.is_vector());
//! ```
//!
//! Points and vectors are not the same:
//! ```
//! # use rtc::tuple::Tuple;
//! let p = Tuple::point(1.0, 4.2, -3.7);
//! let v = Tuple::vector(1.0, 4.2, -3.7);
//! assert_ne!(p, v);
//! ```
//!
//! Points represent a position in 3D space. Vectors represent a displacement or movement.
//!
//!
//! Individual coordinates can be accessed with their respective methods:
//! ```
//! # use rtc::tuple::Tuple;
//! let p = Tuple::point(1.0, 4.2, -3.7);
//! assert_eq!(p.x(), 1.0);
//! assert_eq!(p.y(), 4.2);
//! assert_eq!(p.z(), -3.7);
//! ```
//!
//! ... or with indexing:
//! ```
//! # use rtc::tuple::Tuple;
//! let p = Tuple::point(1.0, 4.2, -3.7);
//! assert_eq!(p[0], 1.0);
//! assert_eq!(p[1], 4.2);
//! assert_eq!(p[2], -3.7);
//! ```
//!
//! ## Arithmetic
//!
//! Tuples support all common arithmetic operations. However, be careful, as for example points
//! can't be added to points. You have to add vectors to points to get another point. This library
//! won't check this for you, because of simplicity and for performance reasons. You should make
//! sure you handle everything correctly to avoid bugs. All operations which support operator
//! overloading support mutable assignment. The available operations are:
//!
//! - Addition (p+v -> v, v+p -> p, v+v -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let mut p = Tuple::point(3.0, -2.0, 5.0);
//! let v = Tuple::vector(-2.0, 3.0, 1.0);
//! assert_eq!(p + v, Tuple::point(1.0, 1.0, 6.0));
//! p += v;
//! assert_eq!(p, Tuple::point(1.0, 1.0, 6.0));
//! ```
//!
//! - Subtraction (p-p -> v, p-v -> p, v-v -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let p1 = Tuple::point(3.0, 2.0, 1.0);
//! let p2 = Tuple::point(5.0, 6.0, 7.0);
//! assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
//!
//! let v1 = Tuple::vector(5.0, 6.0, 7.0);
//! assert_eq!(p1 - v1, Tuple::point(-2.0, -4.0, -6.0));
//!
//! let v2 = Tuple::vector(3.0, 2.0, 1.0);
//! assert_eq!(v2 - v1, Tuple::vector(-2.0, -4.0, -6.0));
//! ```
//!
//! - Negation (-v -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let v = Tuple::vector(1.0, -2.0, 3.0);
//! assert_eq!(-v, Tuple::vector(-1.0, 2.0, -3.0));
//! ```
//!
//! - Scalar multiplication (v*f -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let v = Tuple::vector(1.0, -2.0, 3.0);
//! assert_eq!(v * 3.5, Tuple::vector(3.5, -7.0, 10.5));
//! assert_eq!(v * 0.5, Tuple::vector(0.5, -1.0, 1.5));
//! ```
//!
//! - Scalar division (v/f -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let v = Tuple::vector(1.0, -2.0, 3.0);
//! assert_eq!(v / 2.0, Tuple::vector(0.5, -1.0, 1.5));
//! ```
//!
//! - Dot product (v⋅v -> f)
//! ```
//! # use rtc::tuple::Tuple;
//! let v1 = Tuple::vector(1.0, 2.0, 3.0);
//! let v2 = Tuple::vector(2.0, 3.0, 4.0);
//! assert_eq!(v1.dot(v2), 20.0);
//! ```
//!
//! - Cross product (v×v -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let v1 = Tuple::vector(1.0, 2.0, 3.0);
//! let v2 = Tuple::vector(2.0, 3.0, 4.0);
//! assert_eq!(v1.cross(v2), Tuple::vector(-1.0, 2.0, -1.0));
//! assert_eq!(v2.cross(v1), Tuple::vector(1.0, -2.0, 1.0));
//! ```
//!
//! - Reflection (v.reflect(v) -> v)
//! ```
//! # use rtc::tuple::Tuple;
//! let v = Tuple::vector(1.0, -1.0, 0.0);
//! let n = Tuple::vector(0.0, 1.0, 0.0);
//! let r = v.reflect(n);
//! assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
//! ```
//!
//! ## Normalization
//!
//! When working with vectors (so not points), you can take the norm of vectors and normalize them.
//! You can also ask the square of the norm. This is faster than the norm itself, because no square
//! root has to be taken.
//!
//! ```
//! # use rtc::tuple::Tuple;
//!
//! let sqrt14 = (14.0 as f64).sqrt();
//! let mut v = Tuple::vector(1.0, 2.0, 3.0);
//! assert_eq!(v.norm_squared(), 14.0);
//! assert_eq!(v.norm(), sqrt14);
//! assert_eq!(v.normalized(), Tuple::vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14));
//!
//! v.normalize();
//! assert_eq!(v, Tuple::vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14));
//! ```

use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// Tuple represents a 3D tuple. See the module's documentation for more information.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    /// Returns a new tuple with the given components. You should use [Tuple::point] and
    /// [Tuple::vector] instead.
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Returns a new point with the given coordinates.
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    /// Returns a new vector with the given coordinates.
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    /// Returns `self`s x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns `self`s y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns `self`s z coordinate.
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Returns `self`s w coordinate.
    pub fn w(&self) -> f64 {
        self.w
    }

    /// Returns true if `self` represents a point, false otherwise.
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Returns true if `self` represents a vector, false otherwise.
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Returns the dot product between `self` and `other`. See the module's documentation for
    /// examples. Only works for vectors, not points.
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Returns the cross product between `self` and `other`. See the module's documentation for
    /// examples. Only works for vectors, not points.
    pub fn cross(self, other: Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns the square of the euclidean norm of `self`. See the module's documentation for
    /// examples. Only works for vectors, not points.
    pub fn norm_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns the norm of `self`. See the module's documentation for examples. Only works for
    /// vectors, not points.
    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns a vector in the same direction as `self`, but with euclidean norm of one. See the
    /// module's documentation for examples. Only works for vectors, not points.
    pub fn normalized(self) -> Self {
        self / self.norm()
    }

    /// Changes `self` to have a euclidean norm of one, while keeping its direction. See the
    /// module's documentation for examples. Only works for vectors, not points.
    pub fn normalize(&mut self) {
        *self /= self.norm();
    }

    /// Reflects `self` along `normal`
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_point() {
            write!(f, "P({}, {}, {})", self.x, self.y, self.z)
        } else if self.is_vector() {
            write!(f, "V[{} {} {}]", self.x, self.y, self.z)
        } else {
            write!(f, "[{} {} {} ({})]", self.x, self.y, self.z, self.w)
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
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

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
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

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
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

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
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

impl Index<usize> for Tuple {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for tuple, got {}", index),
        }
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for tuple, got {}", index),
        }
    }
}
