use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, DivAssign, MulAssign};
use crate::math::interval::Interval;
use crate::math::utility::{random_float, random_float_range};

const INTENSITY: Interval = Interval{ min: 0.0, max: 0.999 };

#[derive(Debug,Default,Copy,Clone,PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn new_random() -> Self {
        Self {
            x: random_float(),
            y: random_float(),
            z: random_float()
        }
    }

    pub fn new_random_clamped(min: f64, max: f64) -> Self {
        Self {
            x: random_float_range(min, max),
            y: random_float_range(min, max),
            z: random_float_range(min, max)
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn near_zero(self: &Self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
}

#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: (u.y * v.z) - (u.z * v.y),
        y: (u.z * v.x) - (u.x * v.z),
        z: (u.x * v.y) - (u.y * v.x)
    }
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::new_random_clamped(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

#[inline]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[inline]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * dot(v,n)
}

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { return linear_component.sqrt() }
    0.0
}

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}",
             (255.999 * pixel_color.x) as i16,
             (255.999 * pixel_color.y) as i16,
             (255.999 * pixel_color.z) as i16
    )
}

pub fn convert_color(pixel_color: Color) -> Vec<u8> {
    let mut data = Vec::new();
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    data.push((255.999 * INTENSITY.clamp(r)) as u8);
    data.push((255.999 * INTENSITY.clamp(g)) as u8);
    data.push((255.999 * INTENSITY.clamp(b)) as u8);
    data.push(255);

    data
}

impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs.clone();
        self.z += rhs.clone();
    }
}

impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f64> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs.clone();
        self.z -= rhs.clone();
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs.clone();
        self.z /= rhs.clone();
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs.clone();
        self.z *= rhs.clone();
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs:Self) -> Self{
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs:f64) -> Self{
        Self {
            x: self.x + rhs,
            y: self.y + rhs.clone(),
            z: self.z + rhs.clone()
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs:Self) -> Self{
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs:f64) -> Self{
        Self {
            x: self.x - rhs,
            y: self.y - rhs.clone(),
            z: self.z - rhs.clone()
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs:Self) -> Self{
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs:f64) -> Self{
        Self {
            x: self.x * rhs,
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone()
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs:Self) -> Self{
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs:f64) -> Self{
        Self {
            x: self.x / rhs,
            y: self.y / rhs.clone(),
            z: self.z / rhs.clone()
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

