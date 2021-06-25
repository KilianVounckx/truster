//! A 2D canvas which represents an image of RGB colors. Pixels can be individually changed
//! individually.
//! Get the canvas in PPM format with [Canvas::to_ppm].
//!
//! # Examples
//!
//! A new (black) canvas can be created with new:
//! ```
//! # use truster::canvas::Canvas;
//! let canvas = Canvas::new(10, 20);
//! assert_eq!(canvas.width(), 10);
//! assert_eq!(canvas.height(), 20);
//! ```
//!
//! Pixels can be accessed and mutated with indexing:
//! ```
//! # use truster::canvas::Canvas;
//! use truster::color::Color;
//!
//! let mut canvas = Canvas::new(10, 20);
//! let red = Color::new(1.0, 0.0, 0.0);
//! canvas[[2, 3]] = red;
//! assert_eq!(canvas[[2, 3]], red);
//! ```
//!
//! Save a canvas in PPM format with [Canvas::to_ppm]:
//! ```
//! # use truster::canvas::Canvas;
//! use truster::color::Color;
//!
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! fn main() -> std::io::Result<()> {
//!     let mut canvas = Canvas::new(5, 3);
//!     canvas[[0, 0]] = Color::new(1.5, 0.0, 0.0);
//!     canvas[[2, 1]] = Color::new(0.0, 0.5, 0.0);
//!     canvas[[4, 2]] = Color::new(-0.5, 0.0, 1.0);
//!
//!     let mut file = File::create("foo.ppm")?;
//!     canvas.to_ppm(&mut file);
//!     let mut file = File::open("foo.ppm")?;
//!     let mut output = String::new();
//! 	file.read_to_string(&mut output);
//!
//!     assert_eq!(output, "P3
//! 5 3
//! 255
//! 255 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 128 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 0
//! 0 0 255
//! ");
//!
//! 	Ok(())
//! }
//! ```

use std::io::{Error, Write};
use std::ops::{Index, IndexMut};

use crate::color::Color;

/// A 2D image. See the module's documentation for more information.
pub struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    /// Creates a new canvas with the given width and height. The new canvas is entirely black.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![Color::default(); width]; height],
        }
    }

    /// Returns `self`'s width, that is the number of columns in the image.
    pub fn width(&self) -> usize {
        if self.pixels.len() == 0 {
            0
        } else {
            self.pixels[0].len()
        }
    }

    /// Returns `self`'s height, that is the number of rows in the image.
    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    /// Writes `self` to `file` in PPM format. See the module's documentation for an example.
    pub fn to_ppm(&self, file: &mut dyn Write) -> Result<(), Error> {
        write!(file, "P3\n{} {}\n255\n", self.width(), self.height())?;
        for row in self.pixels.iter() {
            for color in row {
                writeln!(file, "{}", color)?;
            }
        }
        Ok(())
    }
}

impl Index<[usize; 2]> for Canvas {
    type Output = Color;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.pixels[index[1]][index[0]]
    }
}

impl IndexMut<[usize; 2]> for Canvas {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.pixels[index[1]][index[0]]
    }
}
