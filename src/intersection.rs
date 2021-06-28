//! Holds the [Intersection] struct, and the [HitRecord] struct, as well as some helpful trait
//! implementations.

use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;

static ID: AtomicUsize = AtomicUsize::new(0);

/// Stores some information about intersections between rays and shapes.
/// Cloning is constant time and memory.
#[derive(Clone)]
pub struct Intersection {
    id: usize,
    t: f64,
    shape: Rc<dyn Shape>,
}

impl Intersection {
    /// Returns a new [Intersection].
    /// `t` is the distance between the ray origin and the intersection point.
    /// `shape` is the the shape which is intersected with.
    pub fn new(t: f64, shape: Rc<dyn Shape>) -> Self {
        Self {
            t,
            shape,
            id: ID.fetch_add(1, AtomicOrdering::SeqCst),
        }
    }

    /// Returns `self`'s distance.
    pub fn t(&self) -> f64 {
        self.t
    }

    /// Returns `self`'s shape.
    pub fn shape(&self) -> Rc<dyn Shape> {
        Rc::clone(&self.shape)
    }
}

impl Debug for Intersection {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(
            f,
            "{{ id: {}, t: {}, shape: <something> }}",
            self.id, self.t
        )
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.t.is_nan() || other.t.is_nan() {
            return None;
        }

        Some(if self.t < other.t {
            Less
        } else if self.t > other.t {
            Greater
        } else {
            Equal
        })
    }
}

/// Hit holds a hit function which should return an instance of `T` if there is a hit. None
/// otherwise.
/// Known implementations: [Vec<Intersection>].
pub trait Hit<T = Intersection> {
    fn hit(&self) -> Option<&T>;
}

/// Implement [Hit] to get the first intersection which is a hit. The list should be sorted. If
/// there are no hits, `None` is returned.
impl Hit for Vec<Intersection> {
    fn hit(&self) -> Option<&Intersection> {
        for i in self.iter() {
            if i.t > 0.0 {
                return Some(i);
            }
        }
        None
    }
}

const EPS: f64 = 0.000_001;

/// HitRecord stores some information relating to ray-shape intersections. Cloning is near constant
/// time and memory.
#[derive(Clone)]
pub struct HitRecord {
    t: f64,
    shape: Rc<dyn Shape>,
    point: Tuple,
    over_point: Tuple,
    under_point: Tuple,
    eye: Tuple,
    normal: Tuple,
    inside: bool,
}

impl HitRecord {
    /// Returns a new [HitRecord] corresponding to the given intersection and ray.
    pub fn new(intersection: &Intersection, ray: &Ray) -> Self {
        let t = intersection.t;
        let shape = Rc::clone(&intersection.shape);
        let point = ray.at(t);
        let eye = -ray.direction();

        let mut normal = shape.normal_at(point);
        let inside = if normal.dot(eye) < 0.0 {
            normal = -normal;
            true
        } else {
            false
        };

        let over_point = point + normal * EPS;
        let under_point = point - normal * EPS;

        Self {
            t,
            shape,
            point,
            over_point,
            under_point,
            eye,
            normal,
            inside,
        }
    }

    /// Returns the shape `self` is holding.
    pub fn shape(&self) -> Rc<dyn Shape> {
        Rc::clone(&self.shape)
    }

    /// Returns the point `self` is holding.
    pub fn point(&self) -> Tuple {
        self.point
    }

    /// Returns the normal `self` is holding.
    pub fn normal(&self) -> Tuple {
        self.normal
    }

    /// Returns the eye `self` is holding.
    pub fn eye(&self) -> Tuple {
        self.eye
    }

    /// Returns `self`'s point slightly outwards from from the shape.
    pub fn over_point(&self) -> Tuple {
        self.over_point
    }

    /// Returns `self`'s point slightly inwards from from the shape.
    pub fn under_point(&self) -> Tuple {
        self.under_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::shape::sphere::Sphere;

    #[test]
    fn hit_all_positive() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new());
        let i1 = Intersection::new(1.0, Rc::clone(&sphere));
        let i2 = Intersection::new(2.0, Rc::clone(&sphere));
        let mut is = vec![Intersection::clone(&i1), i2];
        is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let i = is.hit().unwrap();
        assert_eq!(i, &i1);
    }

    #[test]
    fn hit_some_negative() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new());
        let i1 = Intersection::new(-1.0, Rc::clone(&sphere));
        let i2 = Intersection::new(1.0, Rc::clone(&sphere));
        let mut is = vec![Intersection::clone(&i2), i1];
        is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let i = is.hit().unwrap();
        assert_eq!(i, &i2);
    }

    #[test]
    fn hit_all_negative() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new());
        let i1 = Intersection::new(-2.0, Rc::clone(&sphere));
        let i2 = Intersection::new(-1.0, Rc::clone(&sphere));
        let mut is = vec![i2, i1];
        is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let i = is.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_many() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new());
        let i1 = Intersection::new(5.0, Rc::clone(&sphere));
        let i2 = Intersection::new(7.0, Rc::clone(&sphere));
        let i3 = Intersection::new(-3.0, Rc::clone(&sphere));
        let i4 = Intersection::new(2.0, Rc::clone(&sphere));
        let mut is = vec![Intersection::clone(&i4), i1, i2, i3];
        is.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let i = is.hit().unwrap();
        assert_eq!(i, &i4);
    }

    #[test]
    fn hit_record_outside() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let intersection = Intersection::new(4.0, Rc::new(shape));
        let rec = HitRecord::new(&intersection, &ray);

        assert_eq!(rec.t, intersection.t);
        assert_eq!(rec.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(rec.eye, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(rec.normal, Tuple::vector(0.0, 0.0, -1.0));
        assert!(!rec.inside);
    }

    #[test]
    fn hit_record_inside() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let intersection = Intersection::new(1.0, Rc::new(shape));
        let rec = HitRecord::new(&intersection, &ray);

        assert_eq!(rec.t, intersection.t);
        assert_eq!(rec.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(rec.eye, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(rec.normal, Tuple::vector(0.0, 0.0, -1.0));
        assert!(rec.inside);
    }

    #[test]
    fn hit_record_over_under_point() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::new();
        shape.set_transform(Matrix::translation(0.0, 0.0, 1.0));
        let intersection = Intersection::new(5.0, Rc::new(shape));
        let rec = HitRecord::new(&intersection, &ray);
        assert!(rec.over_point.z() < -EPS / 2.0);
        assert!(rec.point.z() > rec.over_point.z());
        assert!(rec.under_point.z() > -EPS / 2.0);
        assert!(rec.point.z() < rec.under_point.z());
    }
}
