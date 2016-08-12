use cgmath::prelude::*;

use camera::Camera;
use color::Color;
use math::*;
use ray::Ray;
use scene::Scene;

const MAX_DEPTH: f32 = 4.0;

pub struct Raytracer {
    pub camera: Camera,
    pub scene: Scene,
}

impl Raytracer {
    pub fn trace(&self, ray: Ray, max_depth: u32) -> TraceResult {
        let intersection = self.scene.intersect(ray);
        let distance = intersection.map(|i| i.distance).unwrap_or(MAX_DEPTH);

        TraceResult { color: Vector::from_value(distance / MAX_DEPTH).into() }
    }
}

pub struct TraceResult {
    pub color: Color,
}
