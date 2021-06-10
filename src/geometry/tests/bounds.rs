use nalgebra::Vector3;

use crate::geometry::bounds::Bounds3;

#[test]
fn test_new() {
    let vec1 = Vector3::from([1.0, 0.0, 1.0]);
    let vec2 = Vector3::from([0.0, 1.0, 0.0]);
    let bounds = Bounds3::new(vec1, vec2);
    assert_eq!(bounds.p_max, Vector3::from([1.0, 1.0, 1.0]));
}
#[test]
#[should_panic]
fn new_should_panic() {
    let vec1 = Vector3::from([0.0, 0.0, 0.0]);
    let vec2 = vec1.clone();
    Bounds3::new(vec1, vec2);
}

#[test]
fn all_corner() {
    // let's draw a cube on origin and ijk
    // 0 will be i
    // 1 will be O
    // 2 will be ij
    // 3 will be j
    // 4 will be ...

    let vec0 = Vector3::zeros();
    let vec1 = Vector3::from([1.0, 1.0, 1.0]);
    let bounds = Bounds3::new(vec0, vec1);
    assert_eq!(bounds.corner(0), Vector3::from([0.0, 0.0, 0.0]));
    assert_eq!(bounds.corner(1), Vector3::from([1.0, 0.0, 0.0]));
    assert_eq!(bounds.corner(2), Vector3::from([0.0, 1.0, 0.0]));
    assert_eq!(bounds.corner(3), Vector3::from([1.0, 1.0, 0.0]));
    assert_eq!(bounds.corner(4), Vector3::from([0.0, 0.0, 1.0]));
    assert_eq!(bounds.corner(5), Vector3::from([1.0, 0.0, 1.0]));
    assert_eq!(bounds.corner(6), Vector3::from([0.0, 1.0, 1.0]));
    assert_eq!(bounds.corner(7), Vector3::from([1.0, 1.0, 1.0]));
}

#[test]
fn inner_offset() {
    // basically we just transform the O to ijk
    let vec0 = Vector3::from([1.0, 1.0, 1.0]);
    let vec1 = Vector3::from([3.0, 3.0, 3.0]);
    let bounds = Bounds3::new(vec0, vec1);
    let inner = bounds.inner_offset(&Vector3::from([2.0, 2.0, 2.0]));
    // right in the middle
    assert_eq!(inner, Vector3::from([0.5, 0.5, 0.5]));
}
