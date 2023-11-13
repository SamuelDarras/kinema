use std::sync::{Arc, Mutex};

use macroquad::prelude::{draw_line_3d, draw_sphere, Vec3, LIGHTGRAY, WHITE};
use nalgebra::{Point3, Transform3, Translation3};

use crate::draw::Draw;
use crate::relation::DrawRelation;

pub struct Linkage {
    pub transform: Transform3<f32>,
    geometry: Vec<(usize, usize)>,
    points: Vec<Point3<f32>>,
    origin_index: usize,
    /// (anchor, linkage, relation)
    childs: Vec<(usize, Arc<Mutex<Linkage>>, Arc<Mutex<dyn DrawRelation>>)>,
}

impl Linkage {
    pub fn new(
        transform: Transform3<f32>,
        geometry: Vec<(usize, usize)>,
        points: Vec<Point3<f32>>,
        origin_index: usize,
    ) -> Self {
        Self {
            transform,
            geometry,
            points,
            origin_index,
            childs: Vec::new(),
        }
    }

    pub fn add_child(
        &mut self,
        child: &Arc<Mutex<Linkage>>,
        link_id: usize,
        relation: impl DrawRelation + 'static,
    ) {
        self.childs
            .push((link_id, Arc::clone(child), Arc::new(Mutex::new(relation))));
    }

    pub fn set_q(&mut self, child_id: usize, new_q: f32) -> Result<(), ()> {
        let (_, _, relation) = &self.childs[child_id];
        relation.lock().unwrap().set_q(new_q)
    }

    pub fn get_q(&mut self, child_id: usize) -> f32 {
        let (_, _, relation) = &self.childs[child_id];
        relation.lock().unwrap().get_q()
    }
}

impl Draw for Linkage {
    fn draw(&self, t: Transform3<f32>) {
        let transform = t * self.transform;
        for p in &self.points[..] {
            let p = transform * p;
            draw_sphere(Vec3::new(p.x, p.y, p.z), 0.05, None, LIGHTGRAY);
        }
        for (a, b) in &self.geometry[..] {
            let a = transform * self.points[*a];
            let b = transform * self.points[*b];
            draw_line_3d(Vec3::new(a.x, a.y, a.z), Vec3::new(b.x, b.y, b.z), WHITE);
        }
        let origin = self.points[self.origin_index];
        for (child_idx, child, relation) in &self.childs[..] {
            let child_pos = self.points[*child_idx];
            let relation = relation.lock().unwrap();
            let t = transform
                * Translation3::from(origin)
                * Translation3::from(child_pos)
                * relation.get_transform();
            child.lock().unwrap().draw(t);
            relation.draw(t);
        }
    }
}
