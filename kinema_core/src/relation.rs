use std::f32::consts::PI;

use macroquad::prelude::{draw_line_3d, draw_sphere, Vec3, GREEN, RED};
use nalgebra::{Point3, Rotation3, Transform3, Translation3, Vector3};

use crate::draw::Draw;

pub trait DrawRelation: Relation + Draw {}

pub trait Relation {
    fn get_transform(&self) -> Transform3<f32>;
    fn set_q(&mut self, new_q: f32) -> Result<(), String>;
    fn get_q(&self) -> f32;
}

pub struct Hinge {
    z: Vector3<f32>,
    q: f32,
}
impl Hinge {
    pub fn new(z: Vector3<f32>) -> Self {
        Self { z, q: 0.0 }
    }
}
impl Relation for Hinge {
    fn get_transform(&self) -> Transform3<f32> {
        let v = self.z * self.q;
        nalgebra::convert(Rotation3::new(v))
    }

    fn set_q(&mut self, new_q: f32) -> Result<(), String> {
        self.q = new_q * PI * 2.0;
        Ok(())
    }

    fn get_q(&self) -> f32 {
        self.q
    }
}
impl Draw for Hinge {
    fn draw(&mut self, t: Transform3<f32>) {
        let center = t * Point3::new(0.0, 0.0, 0.0);
        draw_sphere(Vec3::new(center.x, center.y, center.z), 0.06, None, RED);
        let z = self.z.normalize() * 0.2;
        let p = Point3::new(z.x, z.y, z.z);
        let line_start = t * p;
        let line_end = t * -p;
        draw_line_3d(
            Vec3::new(line_start.x, line_start.y, line_start.z),
            Vec3::new(line_end.x, line_end.y, line_end.z),
            RED,
        )
    }
}
impl DrawRelation for Hinge {}

pub struct Slide {
    z: Vector3<f32>,
    q: f32,
}
impl Slide {
    pub fn new(z: Vector3<f32>) -> Self {
        Self { z, q: 0.0 }
    }
}
impl Relation for Slide {
    fn get_transform(&self) -> Transform3<f32> {
        let v = self.z * self.q;
        nalgebra::convert(Translation3::from(v))
    }

    fn set_q(&mut self, new_q: f32) -> Result<(), String> {
        if !(0.0..=1.0).contains(&new_q) {
            return Err(format!("Can't set this Slide q to `{new_q}`"));
        }
        self.q = new_q;
        Ok(())
    }

    fn get_q(&self) -> f32 {
        self.q
    }
}
impl Draw for Slide {
    fn draw(&mut self, t: Transform3<f32>) {
        let center = t * Point3::new(0.0, 0.0, 0.0);
        draw_sphere(Vec3::new(center.x, center.y, center.z), 0.06, None, GREEN);
        let original_length = self.z.norm();
        let z = self.z.normalize();
        let p1 = Point3::new(z.x, z.y, z.z * self.q);
        let p2 = Point3::new(z.x, z.y, 1.0 - z.z * self.q);
        let line_start = t * -p1 * original_length;
        let line_end = t * p2 * original_length;
        draw_line_3d(
            Vec3::new(line_start.x, line_start.y, line_start.z),
            Vec3::new(line_end.x, line_end.y, line_end.z),
            GREEN,
        )
    }
}
impl DrawRelation for Slide {}
