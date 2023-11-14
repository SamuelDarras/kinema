use nalgebra::Transform3;

pub trait Draw {
    fn draw(&mut self, t: Transform3<f32>);
}
