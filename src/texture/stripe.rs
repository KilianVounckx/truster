//! Holds the [Stripe] struct, which implements the [Texture].

use std::rc::Rc;

use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

use super::{solid_color::SolidColor, Texture};

/// Combines 2 other textures and lies them out in stripes. The stripes are perpendicular to the
/// x axis.
pub struct Stripe {
    texture1: Rc<dyn Texture>,
    texture2: Rc<dyn Texture>,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl Stripe {
    pub fn new(texture1: Rc<dyn Texture>, texture2: Rc<dyn Texture>) -> Self {
        Self {
            texture1,
            texture2,
            transform: Matrix::eye(),
            transform_inverse: Matrix::eye(),
        }
    }

    pub fn colors(color1: Color, color2: Color) -> Self {
        Self {
            texture1: Rc::new(SolidColor::new(color1)),
            texture2: Rc::new(SolidColor::new(color2)),
            transform: Matrix::eye(),
            transform_inverse: Matrix::eye(),
        }
    }
}

impl Texture for Stripe {
    fn color_at(&self, point: Tuple) -> Color {
        if point.x().floor() as i32 % 2 == 0 {
            self.texture1.color_at_texture(point)
        } else {
            self.texture2.color_at_texture(point)
        }
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform_inverse = transform.inverse();
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_at_constant_y() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let texture = Stripe::colors(white, black);

        assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(texture.color_at(Tuple::point(0.0, 0.1, 0.0)), white);
        assert_eq!(texture.color_at(Tuple::point(0.0, 0.2, 0.0)), white);
    }

    #[test]
    fn color_at_constant_z() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let texture = Stripe::colors(white, black);

        assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 0.1)), white);
        assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 1.2)), white);
    }

    #[test]
    fn color_at_alternating_x() {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        let texture = Stripe::colors(white, black);

        assert_eq!(texture.color_at(Tuple::point(0.0, 0.0, 0.0)), white);
        assert_eq!(texture.color_at(Tuple::point(0.9, 0.0, 0.0)), white);
        assert_eq!(texture.color_at(Tuple::point(1.0, 0.0, 0.0)), black);
        assert_eq!(texture.color_at(Tuple::point(-0.1, 0.0, 0.0)), black);
        assert_eq!(texture.color_at(Tuple::point(-1.0, 0.0, 0.0)), black);
        assert_eq!(texture.color_at(Tuple::point(-1.1, 0.0, 0.0)), white);
    }
}
