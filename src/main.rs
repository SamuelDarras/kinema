use std::sync::{Arc, Mutex};

use kinema_core::draw::Draw;
use kinema_core::macroquad::prelude::*;
use kinema_core::nalgebra::{self as na, Transform3, Vector3};
use kinema_core::relation::{Hinge, Slide};
use kinema_core::system::System;
use kinema_core::{
    linkage::Linkage,
    nalgebra::{Point3, Translation3},
};
use kinema_core::{macroquad, make_linkage};

#[macroquad::main("3D")]
async fn main() {
    let l0 = make_linkage!(
        Translation3::new(0.0, 0.0, 0.0),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)]
    );

    let l1 = make_linkage!(
        Translation3::new(0.0, 0.0, 0.0),
        vec![(0, 1), (0, 2)],
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0)
        ],
        vec![(&l0, 1, Hinge::new(Vector3::z()))]
    );

    let l2 = make_linkage!(
        Translation3::new(0.0, 0.0, 0.0),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        vec![(&l1, 1, Hinge::new(Vector3::z()))]
    );

    let l3 = make_linkage!(
        Translation3::new(0.0, 0.0, 0.0),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        vec![(&l2, 1, Hinge::new(Vector3::z()))]
    );

    let l4 = make_linkage!(
        Translation3::new(0.0, 0.0, 0.0),
        vec![(0, 1)],
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        vec![(&l1, 2, Slide::new(Vector3::z()))]
    );

    let mut s = System::new();
    s.add_linkage(&l0);
    s.add_linkage(&l1);
    s.add_linkage(&l2);
    s.add_linkage(&l3);
    s.add_linkage(&l4);

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

        // {
        //     let mut l1 = l1.lock().unwrap();
        //     let old_q = l1.get_q(0);
        //     l1.set_q(0, -0.01 + old_q).unwrap();
        // }
        // {
        //     let mut l1 = l1.lock().unwrap();
        //     l1.set_q(1, (t.sin() + 1.0) / 2.0).unwrap();
        // }
        // {
        //     let mut l2 = l2.lock().unwrap();
        //     let old_q = l2.get_q(0);
        //     l2.set_q(0, -0.01 + old_q).unwrap();
        // }

        draw_grid(20, 1., WHITE, GRAY);

        let v = (t.sin() + 1.0) / 2.0;
        let mut vals = s.get();
        vals[0] = v;
        vals[1] = v;
        vals[2] = v;
        vals[3] = v;
        s.update(vals).iter().for_each(|r| match r {
            Ok(_) => {}
            Err(msg) => panic!("{msg}"),
        });
        s.draw(Transform3::default());

        // l0.lock().unwrap().draw(Transform3::default());
        // l0.lock().unwrap().reset_draw();

        next_frame().await
    }
}
