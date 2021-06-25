//! An RGB color representation.
//! The components are floating point numbers. Support for generics may be added in the future.
//! When used, the components should be clamped between 0 and 1.
//!
//! # Examples
//!
//! You can create colors [Color::new]:
//! ```
//! # use truster::color::Color;
//! let c = Color::new(-0.5, 0.4, 1.7);
//! assert_eq!(c.r(), -0.5);
//! assert_eq!(c.g(), 0.4);
//! assert_eq!(c.b(), 1.7);
//! ```
//!
//! Individual components can be accessed with their respective methods, or with indexing:
//!
//! ... or with indexing:
//! ```
//! # use truster::color::Color;
//! let c = Color::new(1.0, 4.2, -3.7);
//! assert_eq!(c[0], 1.0);
//! assert_eq!(c[1], 4.2);
//! assert_eq!(c[2], -3.7);
//! ```
//!
//! ## Arithmetic
//!
//! Colors support all common arithmetic operations. All operations which support operator
//! overloading support mutable assignment. The available operations are:
//!
//! - Addition
//! ```
//! # use truster::color::Color;
//! let mut c1 = Color::new(3.0, -2.0, 5.0);
//! let c2 = Color::new(-2.0, 3.0, 1.0);
//! assert_eq!(c1 + c2, Color::new(1.0, 1.0, 6.0));
//! c1 += c2;
//! assert_eq!(c1, Color::new(1.0, 1.0, 6.0));
//! ```
//!
//! - Subtraction
//! ```
//! # use truster::color::Color;
//! let c1 = Color::new(3.0, 2.0, 1.0);
//! let c2 = Color::new(5.0, 6.0, 7.0);
//! assert_eq!(c1 - c2, Color::new(-2.0, -4.0, -6.0));
//! ```
//!
//! - Scalar multiplication
//! ```
//! # use truster::color::Color;
//! let c = Color::new(1.0, -2.0, 3.0);
//! assert_eq!(c * 3.5, Color::new(3.5, -7.0, 10.5));
//! assert_eq!(c * 0.5, Color::new(0.5, -1.0, 1.5));
//! ```
//!
//! - Hadamard multiplication
//! ```
//! # use truster::color::Color;
//! let c1 = Color::new(1.0, 0.2, 0.4);
//! let c2 = Color::new(0.9, 1.0, 0.1);
//! assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04000000000000001));
//! ```

use std::fmt::Display;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

/// Represents an RGB color. See the module's documentation for more info.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    /// Creates a new color with the given rgb components.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Returns `self`'s red component.
    pub fn r(&self) -> f64 {
        self.r
    }

    /// Returns `self`'s green component.
    pub fn g(&self) -> f64 {
        self.g
    }

    /// Returns `self`'s blue component.
    pub fn b(&self) -> f64 {
        self.b
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let r = ((self.r * 256.0) as i32).clamp(0, 255);
        let g = ((self.g * 256.0) as i32).clamp(0, 255);
        let b = ((self.b * 256.0) as i32).clamp(0, 255);
        write!(f, "{} {} {}", r, g, b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of bounds for color, got {}", index),
        }
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Index out of bounds for color, got {}", index),
        }
    }
}
