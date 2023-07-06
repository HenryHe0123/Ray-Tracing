use rand::Rng;
use std::f64::consts::PI;

pub fn _degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn _random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max) //[min,max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x > max {
        return max;
    }
    if x < min {
        return min;
    }
    x
}
