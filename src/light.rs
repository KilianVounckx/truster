//! Holds the [PointLight] struct.

use crate::color::Color;
use crate::tuple::Tuple;

/// Light at a point in 3D space with a color.
pub struct PointLight {
    position: Tuple,
    color: Color,
}

impl PointLight {
    /// Creates and returns a new point light at the given position with the given color.
    pub fn new(position: Tuple, color: Color) -> Self {
        Self { position, color }
    }

    /// Returns `self`'s position.
    pub fn position(&self) -> Tuple {
        self.position
    }

    /// Returns `self`'s color.
    pub fn color(&self) -> Color {
        self.color
    }
}
