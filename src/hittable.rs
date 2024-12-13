use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3D, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3D::default(),
            normal: Vec3::default(),
            mat: Material::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    Sphere(Sphere),
}

impl Hittable for Object {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Object::Sphere(sphere) => sphere.hit(r, ray_t, rec),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectList {
    pub objects: Vec<Object>,
}

impl ObjectList {
    pub fn new() -> ObjectList {
        ObjectList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for ObjectList {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
