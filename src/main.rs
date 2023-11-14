use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use kinema_core::draw::Draw;
use kinema_core::macroquad;
use kinema_core::macroquad::prelude::*;
use kinema_core::nalgebra::{self as na, Rotation3, Transform3, Vector3};
use kinema_core::relation::{Hinge, Slide};
use kinema_core::{
    linkage::Linkage,
    nalgebra::{Point3, Translation3},
};

#[macroquad::main("3D")]
async fn main() {
    let mut l1 = Linkage::new(
        na::convert(Translation3::new(0.0, 0.0, 0.0)),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        0,
    );
    let l1 = Arc::new(Mutex::new(l1));

    let mut l2 = Linkage::new(
        na::convert(Translation3::new(0.0, 0.0, 0.0)),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        0,
    );
    let l2 = Arc::new(Mutex::new(l2));
    l1.lock()
        .unwrap()
        .add_child(&l2, 1, Hinge::new(Vector3::z()));

    let mut l3 = Linkage::new(
        na::convert(Translation3::new(0.0, 0.0, 0.0)),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        0,
    );
    let l3 = Arc::new(Mutex::new(l3));
    l2.lock()
        .unwrap()
        .add_child(&l3, 1, Hinge::new(Vector3::z()));

    l3.lock()
        .unwrap()
        .add_child(&l1, 1, Hinge::new(Vector3::z()));

    // let l2 = Linkage::new(
    //     na::convert(Translation3::new(0.0, 0.0, 0.0)),
    //     vec![(0, 1), (0, 2)],
    //     vec![
    //         Point3::new(0.0, 0.0, 0.0),
    //         Point3::new(0.0, 1.0, 0.0),
    //         Point3::new(0.0, 0.0, 1.0),
    //     ],
    //     0,
    // );
    // let l2 = Arc::new(Mutex::new(l2));
    // l1.lock()
    //     .unwrap()
    //     .add_child(&l2, 1, Hinge::new(Vector3::z()));
    //
    // let l3 = Linkage::new(
    //     na::convert(Translation3::new(0.0, 0.0, 0.0)),
    //     vec![(0, 1)],
    //     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
    //     0,
    // );
    // let l3 = Arc::new(Mutex::new(l3));
    // l2.lock()
    //     .unwrap()
    //     .add_child(&l3, 1, Hinge::new(Vector3::z()));
    //
    // let l4 = Linkage::new(
    //     na::convert(Translation3::new(0.0, 0.0, 0.0)),
    //     vec![(0, 1)],
    //     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)],
    //     0,
    // );
    // let l4 = Arc::new(Mutex::new(l4));
    // l2.lock()
    //     .unwrap()
    //     .add_child(&l4, 2, Slide::new(Vector3::z()));
    //
    // let l5 = Linkage::new(
    //     na::convert(Translation3::new(0.0, 0.0, 0.0)),
    //     vec![(0, 1)],
    //     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
    //     0,
    // );
    // let l5 = Arc::new(Mutex::new(l5));
    // l4.lock()
    //     .unwrap()
    //     .add_child(&l5, 1, Hinge::new(Vector3::z()));

    let mut camera_x;
    let mut camera_z;
    let mut camera_angle: f32 = 1.0;

    let mut t: f32 = 0.0;
    loop {
        t += 0.01;

        clear_background(BLACK);

        // angle += 0.005;

        camera_x = camera_angle.cos();
        camera_z = camera_angle.sin();
        set_camera(&Camera3D {
            position: vec3(camera_x * 5.0, 5., camera_z * 5.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., WHITE, GRAY);

        l1.lock().unwrap().draw(Transform3::default());
        l1.lock().unwrap().reset_draw();

        let mut l3 = l3.lock().unwrap();
        let old_q = l3.get_q(0);
        l3.set_q(0, 0.01 + old_q).unwrap();

        // let mut l2 = l2.lock().unwrap();
        // let old_q = l2.get_q(0);
        // l2.set_q(0, -0.01 + old_q).unwrap();
        // l2.set_q(1, (t.sin() + 1.0) / 2.0).unwrap();
        //
        // let mut l4 = l4.lock().unwrap();
        // let old_q = l4.get_q(0);
        // l4.set_q(0, old_q + 0.01);

        next_frame().await
    }
}
