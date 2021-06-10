use crate::geometry::*;
pub trait Primitive {
    /// bounds in world space
    fn world_bounds() -> bounds::Bounds3;
    /// returning surface intersection
    fn intersect(&self, ray: ray::Ray) -> Option<()>;
    fn intersect_p(&self, ray: ray::Ray);
}

pub trait EmissivePrimitive{
    fn area_light() -> Option<()>;
}

impl<T> EmissivePrimitive for T
where T: Primitive
{
    fn area_light() -> Option<()> {
        None
    }
}


pub trait MaterialedPrimitive {
    fn material() -> Option<()>;
}

impl<T> MaterialedPrimitive for T
where T: Primitive
{
    fn material() -> Option<()> {
        None
    }
}
