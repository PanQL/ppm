use std::ops::*;

extern crate rand;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other : Vector3) -> Vector3{
        Vector3 { x : self.x + other.x, y : self.y + other.y, z : self.z + other.z,}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other : Vector3) -> Vector3{
        Vector3 { x : self.x - other.x, y : self.y - other.y, z : self.z - other.z,}
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other : Vector3) -> Vector3 {
        Vector3 { x : self.x * other.x, y : self.y * other.y, z : self.z * other.z }
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, other : Vector3) -> Vector3 {
        Vector3 { x : self.x / other.x, y : self.y / other.y, z : self.z / other.z }
    }
}

impl Vector3 {
    pub fn random() -> Self {
        Vector3 { 
            x : rand::thread_rng().gen_range(0.0, 2.0) as f64,
            y : rand::thread_rng().gen_range(0.0, 2.0) as f64,
            z : rand::thread_rng().gen_range(0.0, 2.0) as f64,
        }.normalize()
    }

    pub fn new(x : f64, y : f64, z : f64) -> Self {
        Vector3{ x, y, z,}
    }

    pub fn dot(&self, b : &Vector3 ) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn mult(&self, b : f64) -> Vector3 {
        Vector3 { x : self.x * b, y : self.y * b, z : self.z * b }
    }

    pub fn normalize(&self) -> Vector3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len == 0.0 {
            println!("null !!! {:?}", self);
        }
        Vector3 { x : self.x / len , y : self.y / len, z : self.z / len}
    }

    pub fn is_zero_vec(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn negate(&self) -> Vector3 {
        Vector3 { x : -self.x, y : -self.y, z : -self.z, }
    }

    pub fn to_Int(&self) -> (u8, u8, u8) {
        if !self.is_zero_vec() {
            let temp = self.normalize();
            return ( ( temp.x * 255.0 ) as u8,
            ( temp.y * 255.0 ) as u8,
            ( temp.z * 255.0 ) as u8 );
        }
        (0u8, 0u8, 0u8)
    }
}

#[derive(Debug)]
pub struct Ray {
    pub o : Vector3,
    pub d : Vector3,
}

impl Ray {
    pub fn new(_o : Vector3, _d : Vector3) -> Self {
        Ray { o : _o, d : _d }
    }
}

#[derive(Debug)]
pub enum Refl_t {
    DIFF,
    SPEC,
    REPR,
}

pub fn clamp(x : f64) -> f64 {
    return if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    };
}

