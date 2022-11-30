use std::f32::consts::PI;

use copper_engine::{
    blank_on_awake, blank_tick_update, engine,
    input::InputManager,
    object::{model_loader::get_obj, GameObject, Material},
    structs::Scene,
    Camera,
};

fn main() {
    let input_manager = InputManager::new();
    let main_camera = Camera::new(
        0.1,
        1024.0,
        PI / 3.0,
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        camera_tick_update,
        blank_on_awake,
    );

    let pink_ball = GameObject::new(
        "Pink Ball",
        get_obj("models/ico.obj"),
        [10.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        Material {
            ambient_color: [1.0, 0.2, 0.3],
        },
        blank_tick_update,
        blank_on_awake,
    );

    let blue_ball = GameObject::new(
        "Blue Ball",
        get_obj("models/ico.obj"),
        [0.0, 0.0, -10.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        Material {
            ambient_color: [0.0, 0.2, 0.3],
        },
        blank_tick_update,
        blank_on_awake,
    );

    let green_ball = GameObject::new(
        "Dragon2",
        get_obj("models/ico.obj"),
        [0.0, 0.0, 10.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        Material {
            ambient_color: [0.0, 1.0, 0.0],
        },
        blank_tick_update,
        blank_on_awake,
    );

    let purple_ball = GameObject::new(
        "Ball2",
        get_obj("models/ico.obj"),
        [-10.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        Material {
            ambient_color: [1.0, 0.0, 1.0],
        },
        blank_tick_update,
        blank_on_awake,
    );

    let scene = Scene::new(
        vec![pink_ball, blue_ball, green_ball, purple_ball],
        vec![],
        input_manager,
        main_camera,
    );

    engine(scene);
}

fn camera_tick_update(scene: &mut Scene) {
    let neutral_speed: f32 = 100.0;
    let shift_speed: f32 = 250.0;
    let cam_sens: f32 = 0.0025;

    let mc = scene.main_camera;

    let mut mp = scene.input_manager.mouse_position;
    let cam_rot = mc.rotation;

    mp[0] -= (mc.window_size[0] / 2) as f64;
    mp[1] -= (mc.window_size[1] / 2) as f64;

    //x rotation
    scene.main_camera.rotation[0] = (mp[0] as f32 * cam_sens).cos();
    scene.main_camera.rotation[2] = (mp[0] as f32 * cam_sens).sin();

    // //y rotation
    // scene.main_camera.rotation[0] = (mp[1] as f32 * cam_sens).cos();
    // scene.main_camera.rotation[1] = (mp[1] as f32 * cam_sens).sin();
}
