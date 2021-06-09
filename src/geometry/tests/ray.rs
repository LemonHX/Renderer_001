use std::f32::NAN;

use crate::geometry::ray::Ray;
use nalgebra::Vector3;

#[test]
fn test_point_at() {
    let ray = Ray::new();
    for i in 0..114514 {
        assert_eq!(ray.point_at(i as f32), Vector3::from([i as f32; 3]))
    }
}

#[test]
fn test_is_validate() {
    // pass
    let mut ray = Ray::new();
    assert!(ray.is_validate());
    // nan in origin
    let mut ray = Ray::new();
    ray.origin[0] = NAN;
    assert!(!ray.is_validate());
    // nan in direction
    let mut ray = Ray::new();
    ray.direction[0] = NAN;
    assert!(!ray.is_validate());
    // t max too deep
    let mut ray = Ray::new();
    ray.t_max = f32::INFINITY;
    assert!(!ray.is_validate());
}
