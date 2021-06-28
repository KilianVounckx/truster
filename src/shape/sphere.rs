//! Holds the [Sphere] struct;

use std::rc::Rc;

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

use super::Shape;

/// A 3D ellipsoid (spheroid).
#[derive(Default, Clone)]
pub struct Sphere {
    transform: Matrix,
    transform_inverse: Matrix,
    material: Material,
}

impl Sphere {
    /// Returns a new sphere with radius 1, centered at the origin.
    /// Use [Sphere::set_transform] to transform it. Give it a material with [Sphere::set_material].
    pub fn new() -> Self {
        Self::default()
    }
}

impl Shape for Sphere {
    /// Returns a sorted vector of all distances where `ray` intersects `self`.
    ///
    /// # Examples
    ///
    /// A ray intersects a sphere at two points.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::ray::Ray;
    /// use truster::tuple::Tuple;
    ///
    /// let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(&ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), 4.0);
    /// assert_eq!(intersections[1].t(), 6.0);
    /// ```
    ///
    /// A ray intersects a sphere at a tangent.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::ray::Ray;
    /// use truster::tuple::Tuple;
    ///
    /// let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(&ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), 5.0);
    /// assert_eq!(intersections[1].t(), 5.0);
    /// ```
    ///
    /// A ray misses a sphere.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::ray::Ray;
    /// use truster::tuple::Tuple;
    ///
    /// let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(&ray);
    /// assert_eq!(intersections.len(), 0);
    /// ```
    ///
    /// A ray originates inside a sphere.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::ray::Ray;
    /// use truster::tuple::Tuple;
    ///
    /// let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(&ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), -1.0);
    /// assert_eq!(intersections[1].t(), 1.0);
    /// ```
    ///
    /// A ray is behind a sphere.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::ray::Ray;
    /// use truster::tuple::Tuple;
    ///
    /// let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(&ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), -6.0);
    /// assert_eq!(intersections[1].t(), -4.0);
    /// ```
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let oc = ray.origin() - Tuple::point(0.0, 0.0, 0.0);

        let a = ray.direction().norm_squared();
        let b = ray.direction().dot(oc);
        let c = oc.norm_squared() - 1.0;

        let d = b * b - a * c;

        if d < 0.0 {
            return Vec::new();
        }

        let sqrtd = d.sqrt();
        let t1 = (-b - sqrtd) / a;
        let t2 = (-b + sqrtd) / a;

        vec![
            Intersection::new(t1, Rc::new(self.clone())),
            Intersection::new(t2, Rc::new(self.clone())),
        ]
    }

    /// Returns the surface normal of `self` at `point`.
    ///
    /// # Examples
    ///
    /// The normal on a sphere at a point on the X axis.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::tuple::Tuple;
    ///
    /// let sphere = Sphere::new();
    /// let normal = sphere.normal_at(Tuple::point(1.0, 0.0, 0.0));
    /// assert_eq!(normal, Tuple::vector(1.0, 0.0, 0.0));
    /// ```
    ///
    /// The normal on a sphere at a point on the Y axis.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::tuple::Tuple;
    ///
    /// let sphere = Sphere::new();
    /// let normal = sphere.normal_at(Tuple::point(0.0, 1.0, 0.0));
    /// assert_eq!(normal, Tuple::vector(0.0, 1.0, 0.0));
    /// ```
    ///
    /// The normal on a sphere at a point on the Z axis.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::tuple::Tuple;
    ///
    /// let sphere = Sphere::new();
    /// let normal = sphere.normal_at(Tuple::point(0.0, 0.0, 1.0));
    /// assert_eq!(normal, Tuple::vector(0.0, 0.0, 1.0));
    /// ```
    ///
    /// The normal on a sphere at a point on the non-axial point.
    /// ```
    /// # use truster::shape::{Shape, sphere::Sphere};
    /// use truster::tuple::Tuple;
    ///
    /// let sphere = Sphere::new();
    /// let normal = sphere.normal_at(Tuple::point(
    ///     (3.0 as f64).sqrt() / 3.0,
    ///     (3.0 as f64).sqrt() / 3.0,
    ///     (3.0 as f64).sqrt() / 3.0,
    /// ));
    /// assert_eq!(normal, Tuple::vector(
    ///     (3.0 as f64).sqrt() / 3.0,
    ///     (3.0 as f64).sqrt() / 3.0,
    ///     (3.0 as f64).sqrt() / 3.0,
    /// ));
    /// ```
    fn local_normal_at(&self, point: Tuple) -> Tuple {
        (point - Tuple::point(0.0, 0.0, 0.0)).normalized()
    }

    /// Sets `self`'s transform to be `transform`.
    fn set_transform(&mut self, transform: Matrix) {
        self.transform_inverse = transform.inverse();
        self.transform = transform;
    }

    /// Returns `self`'s transform.
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    /// Returns `self`'s transform's inverse.
    fn transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }

    /// Returns `self`'s material.
    fn material(&self) -> &Material {
        &self.material
    }

    /// Sets `self`'s material to be `material`.
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}
