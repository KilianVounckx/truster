//! A 2D transformation matrix for points and vectors. For now, only floating point matrices are
//! supported. Generics may come in the future.
//!
//! # Examples
//!
//! Initialize a new Matrix with new:
//! ```
//! # use truster::matrix::Matrix;
//! let m = Matrix::new(&[
//!      1.0,  2.0,  3.0,  4.0,
//!      5.5,  6.5,  7.5,  8.5,
//!      9.0, 10.0, 11.0, 12.0,
//!     13.5, 14.5, 15.5, 16.5,
//! ]);
//!
//! let output = format!("{}", m);
//! assert_eq!(output,
//! "/1 2 3 4\\
//! |5.5 6.5 7.5 8.5|
//! |9 10 11 12|
//! \\13.5 14.5 15.5 16.5/")
//! ```
//!
//! Individual values can be accessed and mutated by index:
//! ```
//! # use truster::matrix::Matrix;
//! let m = Matrix::new(&[
//!      1.0,  2.0,  3.0,  4.0,
//!      5.5,  6.5,  7.5,  8.5,
//!      9.0, 10.0, 11.0, 12.0,
//!     13.5, 14.5, 15.5, 16.5,
//! ]);
//!
//! assert_eq!(m[[0, 0]], 1.0);
//! assert_eq!(m[[0, 3]], 4.0);
//! assert_eq!(m[[1, 0]], 5.5);
//! assert_eq!(m[[1, 2]], 7.5);
//! assert_eq!(m[[2, 2]], 11.0);
//! assert_eq!(m[[3, 0]], 13.5);
//! assert_eq!(m[[3, 2]], 15.5);
//! ```
//!
//! Matrices can be multiplied together:
//! ```
//! # use truster::matrix::Matrix;
//! let m1 = Matrix::new(&[
//!     1.0, 2.0, 3.0, 4.0,
//!     5.0, 6.0, 7.0, 8.0,
//!     9.0, 8.0, 7.0, 6.0,
//!     5.0, 4.0, 3.0, 2.0,
//! ]);
//!
//! let m2 = Matrix::new(&[
//!     -2.0, 1.0, 2.0,  3.0,
//!      3.0, 2.0, 1.0, -1.0,
//!      4.0, 3.0, 6.0,  5.0,
//!      1.0, 2.0, 7.0,  8.0,
//! ]);
//!
//! assert_eq!(m1 * &m2, Matrix::new(&[
//!     20.0, 22.0,  50.0,  48.0,
//!     44.0, 54.0, 114.0, 108.0,
//!     40.0, 58.0, 110.0, 102.0,
//!     16.0, 26.0,  46.0,  42.0,
//! ]));
//! ```
//!
//! You can also multiply matrices with tuples to get their transform:
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! let m = Matrix::new(&[
//!     1.0, 2.0, 3.0, 4.0,
//!     2.0, 4.0, 4.0, 2.0,
//!     8.0, 6.0, 4.0, 1.0,
//!     0.0, 0.0, 0.0, 1.0,
//! ]);
//!
//! let p = Tuple::point(1.0, 2.0, 3.0);
//! assert_eq!(&m * p, Tuple::point(18.0, 24.0, 33.0));
//! ```
//!
//! Take the transpose of a matrix with [Matrix::transpose]:
//! ```
//! # use truster::matrix::Matrix;
//! let m = Matrix::new(&[
//!      1.0,  2.0,  3.0,  4.0,
//!     12.0, 42.0, 69.0, 37.0,
//!     21.0, 24.0, 96.0, 73.0,
//!     -1.0, -2.0, -3.0, -4.0,
//! ]);
//!
//! assert_eq!(m.transpose(), Matrix::new(&[
//! 	1.0, 12.0, 21.0, -1.0,
//! 	2.0, 42.0, 24.0, -2.0,
//! 	3.0, 69.0, 96.0, -3.0,
//! 	4.0, 37.0, 73.0, -4.0,
//! ]));
//! ```
//!
//! Take the inverse of a matrix with [Matrix::inverse]:
//! ```
//! # use truster::matrix::Matrix;
//! let m1 = Matrix::new(&[
//!      3.0, -9.0,  7.0,  3.0,
//!      3.0, -8.0,  2.0, -9.0,
//!     -4.0,  4.0,  4.0,  1.0,
//!     -6.0,  5.0, -1.0,  1.0,
//! ]);
//!
//! let m2 = Matrix::new(&[
//!     8.0,  2.0, 2.0, 2.0,
//!     3.0, -1.0, 7.0, 0.0,
//!     7.0,  0.0, 5.0, 4.0,
//!     6.0, -2.0, 0.0, 5.0,
//! ]);
//!
//! let m3 = m1.clone() * &m2;
//! // m3 * &m2.inverse() == m1 // approximately (floating points)
//! ```
//!
//! ## Transformations
//!
//! This library is meant for using matrices as transformations, so all common transformations can
//! be used directly.
//!
//! The supported transformations are:
//!
//! - Translation
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! let t = Matrix::translation(5.0, -3.0, 2.0);
//! let p = Tuple::point(-3.0, 4.0, 5.0);
//! assert_eq!(&t * p, Tuple::point(2.0, 1.0, 7.0));
//!
//! // vectors stay unchanged.
//! let t = Matrix::translation(5.0, -3.0, 2.0);
//! let v = Tuple::vector(-3.0, 4.0, 5.0);
//! assert_eq!(&t * v, v);
//! ```
//!
//! - Scaling
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! let t = Matrix::scaling(2.0, 3.0, 4.0);
//! let p = Tuple::point(-4.0, 6.0, 8.0);
//! assert_eq!(&t * p, Tuple::point(-8.0, 18.0, 32.0));
//! ```
//!
//! - Rotation around X axis.
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! use std::f64::consts::PI;
//!
//! let t = Matrix::rotation_x(PI / 2.0);
//! let p = Tuple::point(0.0, 1.0, 0.0);
//! // &t * p == Tuple::point(0.0, 0.0, 1.0)  // approximately
//! ```
//!
//! - Rotation around Y axis.
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! use std::f64::consts::PI;
//!
//! let t = Matrix::rotation_y(PI / 2.0);
//! let p = Tuple::point(0.0, 0.0, 1.0);
//! // &t * p == Tuple::point(1.0, 0.0, 0.0)  // approximately
//! ```
//!
//! - Rotation around Z axis.
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! use std::f64::consts::PI;
//!
//! let t = Matrix::rotation_z(PI / 2.0);
//! let p = Tuple::point(0.0, 1.0, 0.0);
//! // &t * p == Tuple::point(-1.0, 0.0, 0.0)  // approximately
//! ```
//!
//! - Shearing
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! let t = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
//! let p = Tuple::point(2.0, 3.0, 4.0);
//! assert_eq!(&t * p, Tuple::point(6.0, 3.0, 4.0));
//! ```
//!
//! Transformations can be chained together by multiplying:
//! ```
//! # use truster::matrix::Matrix;
//! use truster::tuple::Tuple;
//!
//! use std::f64::consts::PI;
//!
//! let p = Tuple::point(1.0, 0.0, 1.0);
//!
//! let a = Matrix::rotation_x(PI / 2.0);
//! let b = Matrix::scaling(5.0, 5.0, 5.0);
//! let c = Matrix::translation(10.0, 5.0, 7.0);
//!
//! let p2 = &a*p;
//! let p3 = &b*p2;
//! let p4 = &c*p3;
//!
//! let t = c * &b * &a;
//!
//! assert_eq!(&t*p, p4);
//! ```

use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
}

impl Matrix {
    /// Returns a new matrix with the given values. Row major.
    pub fn new(data: &[f64; 16]) -> Self {
        return Self {
            data: data.to_vec(),
        };
    }

    /// Returns the identity matrix.
    pub fn eye() -> Self {
        return Self::new(&[
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);
    }

    /// Returns a translation matrix which translates points, but not vectors.
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::new(&[
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Returns a scaling matrix.
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::new(&[
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Returns a matrix which rotates `theta` radians around the X axis.
    pub fn rotation_x(theta: f64) -> Self {
        Self::new(&[
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            theta.cos(),
            -theta.sin(),
            0.0,
            0.0,
            theta.sin(),
            theta.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    /// Returns a matrix which rotates `theta` radians around the Y axis.
    pub fn rotation_y(theta: f64) -> Self {
        Self::new(&[
            theta.cos(),
            0.0,
            theta.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -theta.sin(),
            0.0,
            theta.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    /// Returns a matrix which rotates `theta` radians around the Z axis.
    pub fn rotation_z(theta: f64) -> Self {
        Self::new(&[
            theta.cos(),
            -theta.sin(),
            0.0,
            0.0,
            theta.sin(),
            theta.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    /// Returns a new shearing matrix. The arguments mean which axis moves in proportion to which
    /// other axis. For example, xy means x moves in proportion to y.
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::new(&[
            1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Returns a new transformation matrix which can be used for camera's. If a camera looks from
    /// `from` at `at, where `up` is approximately pointing up, the resulting matrix will be it's
    /// transformation matrix.
    ///
    /// # Examples
    ///
    /// The transformation matrix for the default orientation.
    /// ```
    /// # use truster::matrix::Matrix;
    /// use truster::tuple::Tuple;
    /// let from = Tuple::point(0.0, 0.0, 0.0);
    /// let at = Tuple::point(0.0, 0.0, -1.0);
    /// let up = Tuple::vector(0.0, 1.0, 0.0);
    /// let t = Matrix::view_transform(from, at, up);
    /// assert_eq!(t, Matrix::eye());
    /// ```
    ///
    /// A view transformation matrix looking in the positive z direction.
    /// ```
    /// # use truster::matrix::Matrix;
    /// use truster::tuple::Tuple;
    /// let from = Tuple::point(0.0, 0.0, 0.0);
    /// let at = Tuple::point(0.0, 0.0, 1.0);
    /// let up = Tuple::vector(0.0, 1.0, 0.0);
    /// let t = Matrix::view_transform(from, at, up);
    /// assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    /// ```
    ///
    /// The view transformation moves the world.
    /// ```
    /// # use truster::matrix::Matrix;
    /// use truster::tuple::Tuple;
    /// let from = Tuple::point(0.0, 0.0, 8.0);
    /// let to = Tuple::point(0.0, 0.0, 0.0);
    /// let up = Tuple::vector(0.0, 1.0, 0.0);
    /// let t = Matrix::view_transform(from, to, up);
    /// assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    /// ```
    pub fn view_transform(from: Tuple, at: Tuple, up: Tuple) -> Self {
        let forward = (at - from).normalized();
        let up = up.normalized();
        let left = forward.cross(up);
        let up = left.cross(forward);
        let orientation = Self::new(&[
            left.x(),
            left.y(),
            left.z(),
            0.0,
            up.x(),
            up.y(),
            up.z(),
            0.0,
            -forward.x(),
            -forward.y(),
            -forward.z(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);
        orientation * &Matrix::translation(-from.x(), -from.y(), -from.z())
    }

    /// Returns the transpose of `self`.
    pub fn transpose(&self) -> Self {
        let mut res = Self::default();
        for row in 0..=3 {
            for col in 0..=3 {
                res[[row, col]] = self[[col, row]];
            }
        }
        res
    }

    /// Returns the multiplicative inverse of `self`.
    pub fn inverse(&self) -> Self {
        let mut result = Self::default();

        result.data[0] = self.data[5] * self.data[10] * self.data[15]
            - self.data[5] * self.data[11] * self.data[14]
            - self.data[9] * self.data[6] * self.data[15]
            + self.data[9] * self.data[7] * self.data[14]
            + self.data[13] * self.data[6] * self.data[11]
            - self.data[13] * self.data[7] * self.data[10];

        result.data[4] = -self.data[4] * self.data[10] * self.data[15]
            + self.data[4] * self.data[11] * self.data[14]
            + self.data[8] * self.data[6] * self.data[15]
            - self.data[8] * self.data[7] * self.data[14]
            - self.data[12] * self.data[6] * self.data[11]
            + self.data[12] * self.data[7] * self.data[10];

        result.data[8] = self.data[4] * self.data[9] * self.data[15]
            - self.data[4] * self.data[11] * self.data[13]
            - self.data[8] * self.data[5] * self.data[15]
            + self.data[8] * self.data[7] * self.data[13]
            + self.data[12] * self.data[5] * self.data[11]
            - self.data[12] * self.data[7] * self.data[9];

        result.data[12] = -self.data[4] * self.data[9] * self.data[14]
            + self.data[4] * self.data[10] * self.data[13]
            + self.data[8] * self.data[5] * self.data[14]
            - self.data[8] * self.data[6] * self.data[13]
            - self.data[12] * self.data[5] * self.data[10]
            + self.data[12] * self.data[6] * self.data[9];

        result.data[1] = -self.data[1] * self.data[10] * self.data[15]
            + self.data[1] * self.data[11] * self.data[14]
            + self.data[9] * self.data[2] * self.data[15]
            - self.data[9] * self.data[3] * self.data[14]
            - self.data[13] * self.data[2] * self.data[11]
            + self.data[13] * self.data[3] * self.data[10];

        result.data[5] = self.data[0] * self.data[10] * self.data[15]
            - self.data[0] * self.data[11] * self.data[14]
            - self.data[8] * self.data[2] * self.data[15]
            + self.data[8] * self.data[3] * self.data[14]
            + self.data[12] * self.data[2] * self.data[11]
            - self.data[12] * self.data[3] * self.data[10];

        result.data[9] = -self.data[0] * self.data[9] * self.data[15]
            + self.data[0] * self.data[11] * self.data[13]
            + self.data[8] * self.data[1] * self.data[15]
            - self.data[8] * self.data[3] * self.data[13]
            - self.data[12] * self.data[1] * self.data[11]
            + self.data[12] * self.data[3] * self.data[9];

        result.data[13] = self.data[0] * self.data[9] * self.data[14]
            - self.data[0] * self.data[10] * self.data[13]
            - self.data[8] * self.data[1] * self.data[14]
            + self.data[8] * self.data[2] * self.data[13]
            + self.data[12] * self.data[1] * self.data[10]
            - self.data[12] * self.data[2] * self.data[9];

        result.data[2] = self.data[1] * self.data[6] * self.data[15]
            - self.data[1] * self.data[7] * self.data[14]
            - self.data[5] * self.data[2] * self.data[15]
            + self.data[5] * self.data[3] * self.data[14]
            + self.data[13] * self.data[2] * self.data[7]
            - self.data[13] * self.data[3] * self.data[6];

        result.data[6] = -self.data[0] * self.data[6] * self.data[15]
            + self.data[0] * self.data[7] * self.data[14]
            + self.data[4] * self.data[2] * self.data[15]
            - self.data[4] * self.data[3] * self.data[14]
            - self.data[12] * self.data[2] * self.data[7]
            + self.data[12] * self.data[3] * self.data[6];

        result.data[10] = self.data[0] * self.data[5] * self.data[15]
            - self.data[0] * self.data[7] * self.data[13]
            - self.data[4] * self.data[1] * self.data[15]
            + self.data[4] * self.data[3] * self.data[13]
            + self.data[12] * self.data[1] * self.data[7]
            - self.data[12] * self.data[3] * self.data[5];

        result.data[14] = -self.data[0] * self.data[5] * self.data[14]
            + self.data[0] * self.data[6] * self.data[13]
            + self.data[4] * self.data[1] * self.data[14]
            - self.data[4] * self.data[2] * self.data[13]
            - self.data[12] * self.data[1] * self.data[6]
            + self.data[12] * self.data[2] * self.data[5];

        result.data[3] = -self.data[1] * self.data[6] * self.data[11]
            + self.data[1] * self.data[7] * self.data[10]
            + self.data[5] * self.data[2] * self.data[11]
            - self.data[5] * self.data[3] * self.data[10]
            - self.data[9] * self.data[2] * self.data[7]
            + self.data[9] * self.data[3] * self.data[6];

        result.data[7] = self.data[0] * self.data[6] * self.data[11]
            - self.data[0] * self.data[7] * self.data[10]
            - self.data[4] * self.data[2] * self.data[11]
            + self.data[4] * self.data[3] * self.data[10]
            + self.data[8] * self.data[2] * self.data[7]
            - self.data[8] * self.data[3] * self.data[6];

        result.data[11] = -self.data[0] * self.data[5] * self.data[11]
            + self.data[0] * self.data[7] * self.data[9]
            + self.data[4] * self.data[1] * self.data[11]
            - self.data[4] * self.data[3] * self.data[9]
            - self.data[8] * self.data[1] * self.data[7]
            + self.data[8] * self.data[3] * self.data[5];

        result.data[15] = self.data[0] * self.data[5] * self.data[10]
            - self.data[0] * self.data[6] * self.data[9]
            - self.data[4] * self.data[1] * self.data[10]
            + self.data[4] * self.data[2] * self.data[9]
            + self.data[8] * self.data[1] * self.data[6]
            - self.data[8] * self.data[2] * self.data[5];

        let det = 1.0
            / (self.data[0] * result.data[0]
                + self.data[1] * result.data[4]
                + self.data[2] * result.data[8]
                + self.data[3] * result.data[12]);

        for i in 0..16 {
            result.data[i] *= det;
        }

        return result;
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::eye()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        unsafe {
            writeln!(
                f,
                "/{} {} {} {}\\",
                self.data.get_unchecked(0),
                self.data.get_unchecked(1),
                self.data.get_unchecked(2),
                self.data.get_unchecked(3),
            )?;

            writeln!(
                f,
                "|{} {} {} {}|",
                self.data.get_unchecked(4),
                self.data.get_unchecked(5),
                self.data.get_unchecked(6),
                self.data.get_unchecked(7),
            )?;

            writeln!(
                f,
                "|{} {} {} {}|",
                self.data.get_unchecked(8),
                self.data.get_unchecked(9),
                self.data.get_unchecked(10),
                self.data.get_unchecked(11),
            )?;

            write!(
                f,
                "\\{} {} {} {}/",
                self.data.get_unchecked(12),
                self.data.get_unchecked(13),
                self.data.get_unchecked(14),
                self.data.get_unchecked(15),
            )?;

            Ok(())
        }
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        if index[0] > 3 {
            panic!("Row index out of bounds for matrix, got {}", index[0]);
        }
        if index[1] > 3 {
            panic!("Column index out of bounds for matrix, got {}", index[1]);
        }

        unsafe { self.data.get_unchecked(index[0] * 4 + index[1]) }
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        if index[0] > 3 {
            panic!("Row index out of bounds for matrix, got {}", index[0]);
        }
        if index[1] > 3 {
            panic!("Column index out of bounds for matrix, got {}", index[1]);
        }

        unsafe { self.data.get_unchecked_mut(index[0] * 4 + index[1]) }
    }
}

impl Mul<&Self> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut res = Matrix::default();
        for row in 0..=3 {
            for col in 0..=3 {
                res[[row, col]] = self[[row, 0]] * rhs[[0, col]]
                    + self[[row, 1]] * rhs[[1, col]]
                    + self[[row, 2]] * rhs[[2, col]]
                    + self[[row, 3]] * rhs[[3, col]];
            }
        }
        res
    }
}

impl Mul<Self> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();
        for row in 0..=3 {
            for col in 0..=3 {
                res[[row, col]] = self[[row, 0]] * rhs[[0, col]]
                    + self[[row, 1]] * rhs[[1, col]]
                    + self[[row, 2]] * rhs[[2, col]]
                    + self[[row, 3]] * rhs[[3, col]];
            }
        }
        res
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let x = self[[0, 0]] * rhs.x()
            + self[[0, 1]] * rhs.y()
            + self[[0, 2]] * rhs.z()
            + self[[0, 3]] * rhs.w();
        let y = self[[1, 0]] * rhs.x()
            + self[[1, 1]] * rhs.y()
            + self[[1, 2]] * rhs.z()
            + self[[1, 3]] * rhs.w();
        let z = self[[2, 0]] * rhs.x()
            + self[[2, 1]] * rhs.y()
            + self[[2, 2]] * rhs.z()
            + self[[2, 3]] * rhs.w();
        let w = self[[3, 0]] * rhs.x()
            + self[[3, 1]] * rhs.y()
            + self[[3, 2]] * rhs.z()
            + self[[3, 3]] * rhs.w();
        Tuple::new(x, y, z, w)
    }
}
