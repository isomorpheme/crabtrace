use math::*;
use ray::Ray;

/// Represents an intersection between a `Ray` and another object.
#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub distance: f32,
    pub position: Point,
    pub normal: Vector,
}

impl Intersection {
    pub fn nearest(lhs: Self, rhs: Self) -> Intersection {
        if lhs.distance < rhs.distance {
            lhs
        } else {
            rhs
        }
    }
}

/// Something that can be intersected by a ray.
pub trait Intersect {
    /// Intersect a ray with this object. Returns `None` if there is no intersection.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
