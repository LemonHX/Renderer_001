use std::f32::NAN;

use nalgebra::Vector3;

fn main() {
    println!("\t-h for help");
    let res = Vector3::from([1.0, 2.0, NAN]).iter().fold(true, |o, e| {
        println!("e: {:?}, r: {:?}", e, o && !e.is_nan());
        o && !e.is_nan()
    });
    let a = 1;
}
