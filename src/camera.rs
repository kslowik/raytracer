use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable, ObjectList};
use crate::interval::Interval;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::vec3::{Point3D, Vec3};
use chrono::{Local, Timelike};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use std::fs::File;
use std::io;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);

    encoder
        .write_image(
            pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            ExtendedColorType::Rgb8,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "CameraParams")]
pub struct Camera {
    pub height: usize,
    pub width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3D,
    pub lookat: Point3D,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    #[serde(skip_serializing)]
    pub aspect_ratio: f64,
    #[serde(skip_serializing)]
    pixel_samples_scale: f64,
    #[serde(skip_serializing)]
    center: Point3D,
    #[serde(skip_serializing)]
    pixel00_loc: Point3D,
    #[serde(skip_serializing)]
    pixel_delta_u: Vec3,
    #[serde(skip_serializing)]
    pixel_delta_v: Vec3,
    #[serde(skip_serializing)]
    u: Vec3,
    #[serde(skip_serializing)]
    v: Vec3,
    #[serde(skip_serializing)]
    w: Vec3,
    #[serde(skip_serializing)]
    defocus_disk_u: Vec3,
    #[serde(skip_serializing)]
    defocus_disk_v: Vec3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraParams {
    pub height: usize,
    pub width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3D,
    pub lookat: Point3D,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl From<CameraParams> for Camera {
    fn from(p: CameraParams) -> Self {
        Camera::new(
            p.height,
            p.width,
            p.samples_per_pixel,
            p.max_depth,
            p.vfov,
            p.lookfrom,
            p.lookat,
            p.vup,
            p.defocus_angle,
            p.focus_dist,
        )
    }
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    pub fn new(
        height: usize,
        width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
        lookfrom: Point3D,
        lookat: Point3D,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let mut camera = Camera {
            height,
            width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            aspect_ratio: 0.0,
            pixel_samples_scale: 0.0,
            center: Point3D::default(),
            pixel00_loc: Point3D::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        };
        camera.initialize();
        camera
    }

    fn initialize(&mut self) {
        self.aspect_ratio = self.width as f64 / self.height as f64;
        self.height = if self.height < 1 { 1 } else { self.height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.width as f64;
        self.pixel_delta_v = viewport_v / self.height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&self, filename: &str, world: &ObjectList) -> io::Result<()> {
        let mut pixels = vec![Color::default(); self.width * self.height];
        let mut buffer = Vec::with_capacity(self.width * self.height * 3);

        let rows: Vec<(usize, &mut [Color])> = pixels.chunks_mut(self.width).enumerate().collect();

        rows.into_par_iter().for_each(|(j, row)| {
            let second_mod_4 = Local::now().second() % 4;
            let dots = ".".repeat(second_mod_4 as usize % 4);
            eprint!("\rRunning{}", dots);

            for (i, pixel_color) in row.iter_mut().enumerate() {
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    *pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                *pixel_color *= self.pixel_samples_scale;
            }
        });

        for pixel_color in pixels.iter() {
            write_color(&mut buffer, *pixel_color)?;
        }

        write_image(filename, &buffer, (self.width, self.height))?;

        eprintln!("\rDone.                 ");
        Ok(())
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Point3D {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn ray_color(&self, r: &Ray, depth: usize, world: &ObjectList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        if world.hit(r, &Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
