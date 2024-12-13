use crate::vec3::{Point3D, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3D,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point3D {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.orig + self.dir * t
    }
}

#[test]
fn test_new() {
    let origin = Point3D::new(1.0, 2.0, 3.0);
    let direction = Vec3::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(*ray.origin(), origin);
    assert_eq!(*ray.direction(), direction);
}

#[test]
fn test_direction() {
    let direction = Vec3::new(4.0, 5.0, 6.0);
    let ray = Ray::new(Point3D::new(1.0, 2.0, 3.0), direction);
    assert_eq!(*ray.direction(), direction);
}

#[test]
fn test_at() {
    let origin = Point3D::new(1.0, 2.0, 3.0);
    let direction = Vec3::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    let point = ray.at(2.0);
    assert_eq!(point, Point3D::new(9.0, 12.0, 15.0));
}
