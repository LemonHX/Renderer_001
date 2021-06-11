///
/// Hopefully Rust willcomplete rfc 1210 for specialization soon...
///
use crate::geometry::*;
pub trait Primitive {
    /// bounds in world space
    fn world_bounds(&self) -> bounds::Bounds3;
    /// returning surface intersection
    fn intersect(&self, ray: ray::Ray) -> Option<()>;
    fn intersect_p(&self, ray: ray::Ray);
}

pub trait EmissivePrimitive: Primitive {
    fn area_light(&self) -> Option<()>;
}

default impl<T: Primitive> EmissivePrimitive for T {
    fn area_light(&self) -> Option<()> {
        None
    }
}

pub trait MaterialedPrimitive: Primitive {
    fn material(&self) -> Option<()>;
}

default impl<T: Primitive> MaterialedPrimitive for T {
    fn material(&self) -> Option<()> {
        None
    }
}
