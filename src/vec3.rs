use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Point3D = Vec3;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        let dz = self.z - other.z();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((-*uv).dot(n), 1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

#[test]
fn test_new() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
}

#[test]
fn test_random() {
    let v = Vec3::random(0.0, 1.0);
    assert!(v.x() >= 0.0 && v.x() < 1.0);
    assert!(v.y() >= 0.0 && v.y() < 1.0);
    assert!(v.z() >= 0.0 && v.z() < 1.0);
}

#[test]
fn test_length_squared() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.length_squared(), 14.0);
}

#[test]
fn test_length() {
    let v = Vec3::new(1.0, 2.0, 2.0);
    assert_eq!(v.length(), 3.0);
}

#[test]
fn test_unit_vector() {
    let v = Vec3::new(1.0, 2.0, 2.0);
    let unit_v = v.unit_vector();
    assert!((unit_v.length() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, -5.0, 6.0);
    assert_eq!(v1.dot(&v2), 12.0);
}

#[test]
fn test_cross() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let cross_v = v1.cross(&v2);
    assert_eq!(cross_v, Vec3::new(-3.0, 6.0, -3.0));
}

#[test]
fn test_add() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_sub() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
}

#[test]
fn test_mul() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
}

#[test]
fn test_div() {
    let v = Vec3::new(2.0, 4.0, 6.0);
    assert_eq!(v / 2.0, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_reflect() {
    let v = Vec3::new(1.0, -1.0, 0.0);
    let n = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(Vec3::reflect(&v, &n), Vec3::new(1.0, 1.0, 0.0));
}

#[test]
fn test_refract() {
    let uv = Vec3::new(1.0, -1.0, 0.0).unit_vector();
    let n = Vec3::new(0.0, 1.0, 0.0);
    let etai_over_etat = 0.5;
    let refracted = Vec3::refract(&uv, &n, etai_over_etat);
    assert!((refracted.length() - 1.0).abs() < f64::EPSILON);
}
