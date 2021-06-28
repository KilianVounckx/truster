//! Holds the [Plane] struct;

use std::rc::Rc;

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

use super::Shape;

/// A 3D plane.
#[derive(Default, Clone)]
pub struct Plane {
    transform: Matrix,
    transform_inverse: Matrix,
    material: Material,
}

impl Plane {
    /// Returns a new plane perpendicular to the Y axis (parallel to X and Z), through the origin.
    /// Use [Plane::set_transform] to change it's orientation.
    pub fn new() -> Self {
        Self::default()
    }
}

const EPS: f64 = 0.000_001;

impl Shape for Plane {
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

    fn material(&self) -> &Material {
        &self.material
    }
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction().y().abs() < EPS {
            return Vec::new();
        }

        let t = -ray.origin().y() / ray.direction().y();

        vec![Intersection::new(t, Rc::new(self.clone()))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_normal_at() {
        let plane = Plane::new();
        let normal = Tuple::vector(0.0, 1.0, 0.0);
        let n1 = plane.local_normal_at(Tuple::point(0.0, 0.0, 0.0));
        let n2 = plane.local_normal_at(Tuple::point(10.0, 0.0, -10.0));
        let n3 = plane.local_normal_at(Tuple::point(-5.0, 0.0, 150.0));
        assert_eq!(n1, normal);
        assert_eq!(n2, normal);
        assert_eq!(n3, normal);
    }

    #[test]
    fn intersect_parallel() {
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = plane.local_intersect(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn intersect_coplanar() {
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = plane.local_intersect(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn intersect_above() {
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let intersections = plane.local_intersect(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t(), 1.0);
    }

    #[test]
    fn intersect_below() {
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let intersections = plane.local_intersect(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t(), 1.0);
    }
}
