extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};
use nalgebra::Quaternion;
use nalgebra::Unit;

struct Subject {
    id: i32,
    color: (f32, f32, f32),
    rotation: Unit<Quaternion<f64>>,
    position: (f32, f32, f32),
    dimensions: (f32, f32, f32),
}

fn main() {
    let mut window = Window::new("Simple Graphics");
    let mut subjects: Vec<Subject> = Vec::new();

    // Build subject data structure
    for i in 0..10 {
        let new_subject: Subject = Subject {
            id: i,
            color: (1.0 / i as f32, 0.0, 1.0 / i as f32),
            rotation: UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.5 / i as f64),
            position: (0.0, 0.0, 0.0),
            dimensions: (1.0, 1.0, 1.0),
        };
        println!("\n\nBuilding subject {}", new_subject.id);
        println!("Color: {:?}", new_subject.color);
        println!("Rotation {}", new_subject.rotation);
        println!("Postion {:?}", new_subject.position);
        subjects.push(new_subject);
    }

    // Add subjects to window
    let mut game_subjects: Vec<SceneNode> = Vec::new();
    for subject in subjects {
        match subject.dimensions {
            (x, y, z) => {
                let mut new_subject = window.add_cube(x, y, z);
                new_subject.set_color(subject.color.0, subject.color.1, subject.color.2);
                game_subjects.push(new_subject);
            }
        }
    }

    window.set_light(Light::StickToCamera);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    while window.render() {
        for mut subject in game_subjects.clone() {
            subject.prepend_to_local_rotation(&rot);
        }
    }
}
