use nalgebra::Transform3;

pub trait Draw {
    fn draw(&self, t: Transform3<f32>);
}
