pub mod ray;
pub mod vec3;

use rand::Rng;

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max) //[min,max)
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, max as f64 + 1.0) as i32 //[min,max]
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}
