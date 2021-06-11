use nalgebra::Vector3;

use crate::{
    core::primitive::{EmissivePrimitive, MaterialedPrimitive, Primitive},
    geometry::bounds::Bounds3,
};

struct Point;
impl Primitive for Point {
    fn world_bounds(&self) -> crate::geometry::bounds::Bounds3 {
        let v1 = Vector3::from([0.0, 0.0, 0.0]);
        let v2 = Vector3::from([1.0, 1.0, 1.0]);
        Bounds3::new(v1, v2)
    }

    fn intersect(&self, ray: crate::geometry::ray::Ray) -> Option<()> {
        Some(())
    }

    fn intersect_p(&self, ray: crate::geometry::ray::Ray) {}
}
impl EmissivePrimitive for Point {
    fn area_light(&self) -> Option<()> {
        println!("special point with light");
        Some(())
    }
}
impl MaterialedPrimitive for Point {
    fn material(&self) -> Option<()> {
        println!("it has material!");
        Some(())
    }
}
#[test]
fn dum() {
    let p = Point;
    // for trait primitive
    p.world_bounds();
    assert_eq!(p.area_light(), Some(()));
    assert_eq!(p.material(), Some(()));
}
