use nalgebra::Vector3;
pub struct Bounds3 {
    pub(crate) p_min: Vector3<f32>,
    pub(crate) p_max: Vector3<f32>,
}

impl Bounds3 {
    pub fn new(p1: Vector3<f32>, p2: Vector3<f32>) -> Self {
        let p_min_iter = p1.iter().zip(p2.iter()).map(|(p1, p2)| p1.min(*p2));
        let p_max_iter = p1.iter().zip(p2.iter()).map(|(p1, p2)| p1.max(*p2));
        let p_min = Vector3::from_iterator(p_min_iter);
        let p_max = Vector3::from_iterator(p_max_iter);
        let ret = Bounds3 { p_min, p_max };
        debug_assert!(ret.assert_correct());
        ret
    }
    /// think of a corner of a cube if the cube is facing you
    ///```graph
    ///         6----7 (max)
    ///         /    /|
    ///        2----3 |
    ///        |    | 5
    ///        |    |/
    ///  (min) 0----1
    ///```
    pub fn corner(&self, corner: u8) -> Vector3<f32> {
        debug_assert!(corner < 8_u8);
        let x = if corner & 1 == 0 {
            self.p_min.x
        } else {
            self.p_max.x
        };
        let y = if corner & 2 == 0 {
            self.p_min.y
        } else {
            self.p_max.y
        };
        let z = if corner & 4 == 0 {
            self.p_min.z
        } else {
            self.p_max.z
        };
        Vector3::from([x, y, z])
    }
    /// simple vec minus
    pub fn diagonal(&self) -> Vector3<f32> {
        self.p_max - self.p_min
    }

    pub fn max_extent(&self) -> u8 {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            0_u8
        } else if d.y > d.z {
            1_u8
        } else {
            2_u8
        }
    }

    pub(crate) fn assert_correct(&self) -> bool {
        self.p_max.x >= self.p_min.x
            && self.p_max.y >= self.p_min.y
            && self.p_max.z >= self.p_min.z
            && self.p_max != self.p_min
    }

    /// TODO: rename it to something like transform location?
    pub fn inner_offset(&self, point: &Vector3<f32>) -> Vector3<f32> {
        debug_assert!(self.assert_correct());

        let mut o = point - self.p_min;
        o.x /= self.p_max.x - self.p_min.x;
        o.y /= self.p_max.y - self.p_min.y;
        o.z /= self.p_max.z - self.p_min.z;
        o
    }
}
