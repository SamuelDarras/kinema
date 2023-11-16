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
    pub childs: Vec<(usize, Arc<Mutex<Linkage>>, Arc<Mutex<dyn DrawRelation>>)>,
    drawn: bool,
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
            drawn: false,
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

    pub fn set_q(&mut self, child_id: usize, new_q: f32) -> Result<(), String> {
        let mut res = self
            .childs
            .iter()
            .filter(|(anchor, _, _)| *anchor == child_id)
            .map(|(_, _, relation)| relation.lock().unwrap().set_q(new_q));

        if res.all(|r| r.is_ok()) {
            Ok(())
        } else {
            if let Some(e) = res.nth(0) {
                e
            } else {
                Ok(())
            }
        }
    }

    pub fn get_q(&mut self, child_id: usize) -> f32 {
        self.childs
            .iter()
            .filter(|(anchor, _, _)| *anchor == child_id)
            .map(|(_, _, relation)| relation.lock().unwrap().get_q())
            .nth(0)
            .unwrap()
    }

    pub fn reset_draw(&mut self) {
        if !self.drawn {
            return;
        }
        self.drawn = false;
        for (_, child, _) in &self.childs[..] {
            if let Ok(mut child) = child.try_lock() {
                child.reset_draw();
            }
        }
    }
}

impl Draw for Linkage {
    fn draw(&mut self, t: Transform3<f32>) {
        if self.drawn {
            return;
        }
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
            if let Ok(mut relation) = relation.try_lock() {
                let t = transform
                    * Translation3::from(origin)
                    * Translation3::from(child_pos)
                    * relation.get_transform();
                relation.draw(t);
                if let Ok(mut child) = child.try_lock() {
                    child.draw(t);
                }
            }
        }
        self.drawn = true;
    }
}
