use std::f32::NAN;

use nalgebra::Vector3;

/// Ray data structure
/// r(t) = origin + t * direction
#[derive(Debug, Default)]
pub struct Ray {
    /// origin point
    pub origin: Vector3<f32>,
    /// direction
    pub direction: Vector3<f32>,

    /// max distance of t chould be reach
    /// mutable variable
    pub t_max: f32,
    /// the time for this ray for tracing animated object
    time: f32,
    // TODO: finish medium
    /// What kind of propagation medium does this light travel in?
    medium: (),
}

impl Ray {
    /// new function generate a ray start at origin and with identity direction
    /// with t_max 1_000_000.0
    pub fn new() -> Self {
        let mut s = Self::default();
        s.direction = Vector3::from([1.0, 1.0, 1.0]);
        s.t_max = 1_000_000.0;
        s
    }
    /// for ray r calculate point at t
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.origin + (self.direction * t)
    }
    /// check for NaNs and Infs etc...
    pub fn is_validate(&self) -> bool {
        let validate_function = |o: bool, e: f32| o && (!e.is_nan()) && e.is_finite();
        self.origin.fold(true, validate_function)
            && self.direction.fold(true, validate_function)
            && self.t_max.is_finite()
    }
}
