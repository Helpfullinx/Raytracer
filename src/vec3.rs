use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, DivAssign, MulAssign};

#[derive(Debug,Default,Copy,Clone,PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x:f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    //Vec3 Utility Functions

    #[inline]
    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
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
    pub fn unit_vector(v: &Vec3) -> Vec3{
        *v / v.length()
    }

    pub fn write_color(pixel_color: Color) {
        println!("{} {} {}",
            (255.999 * pixel_color.x) as i16,
            (255.999 * pixel_color.y) as i16,
            (255.999 * pixel_color.z) as i16
        )
    }
}

impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f32> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
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

impl SubAssign<f32> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
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

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
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

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
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

impl Add<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs:f32) -> Self{
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

impl Sub<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs:f32) -> Self{
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

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs:f32) -> Self{
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

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs:f32) -> Self{
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

