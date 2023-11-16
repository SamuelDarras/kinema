use std::sync::{Arc, Mutex};

use crate::{draw::Draw, linkage::Linkage};

pub type RelationIndex = (usize, usize);

pub struct System {
    map: Vec<RelationIndex>,
    linkages: Vec<Arc<Mutex<Linkage>>>,
}
impl System {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            linkages: Vec::new(),
        }
    }

    pub fn add_linkage(&mut self, linkage: &Arc<Mutex<Linkage>>) {
        let linkage_idx = self.linkages.len();
        for (child_idx, _, _) in linkage.lock().unwrap().childs.iter() {
            self.map.push((linkage_idx, *child_idx));
        }
        self.linkages.push(Arc::clone(linkage));
    }

    pub fn get(&self) -> Vec<f32> {
        let mut vals = Vec::new();
        for (linkage_index, node_index) in self.map.iter() {
            vals.push(
                self.linkages[*linkage_index]
                    .lock()
                    .unwrap()
                    .get_q(*node_index),
            );
        }
        vals
    }

    pub fn update(&mut self, new_values: Vec<f32>) -> Vec<Result<(), String>> {
        assert_eq!(new_values.len(), self.map.len());
        let mut r = Vec::new();
        for (relation_index, val) in self.map.iter().zip(new_values) {
            let (linkage_index, node_index) = relation_index;
            r.push(
                self.linkages[*linkage_index]
                    .lock()
                    .unwrap()
                    .set_q(*node_index, val),
            );
        }
        r
    }
}
impl Draw for System {
    fn draw(&mut self, t: nalgebra::Transform3<f32>) {
        self.linkages[0].lock().unwrap().draw(t);
        self.linkages[0].lock().unwrap().reset_draw();
    }
}
