use std::f32::consts::PI;

use copper_engine::{
    blank_on_awake, blank_tick_update, engine,
    input::InputManager,
    math::Vector3,
    object::{model_loader::get_obj, GameObject, Material},
    structs::Scene,
    Camera,
};

fn main() {
    let input_manager = InputManager::new();
    let main_camera = Camera::new(
        0.1,
        100.0,
        PI / 4.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
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
    let movement_speed: f32 = 1000.0;

    // W
    if scene.input_manager.pressed_scancodes.contains(&17) {
        scene.main_camera.position.z += movement_speed * scene.delta_time;
        scene.main_camera.rotation.z += movement_speed * scene.delta_time;
    }
    // S
    if scene.input_manager.pressed_scancodes.contains(&31) {
        scene.main_camera.position.z -= movement_speed * scene.delta_time;
        scene.main_camera.rotation.z -= movement_speed * scene.delta_time;

    }
    // A
    if scene.input_manager.pressed_scancodes.contains(&30) {
        scene.main_camera.position.x += movement_speed * scene.delta_time;
        scene.main_camera.rotation.x += movement_speed * scene.delta_time;
    }
    // D
    if scene.input_manager.pressed_scancodes.contains(&32) {
        scene.main_camera.position.x -= movement_speed * scene.delta_time;
        scene.main_camera.rotation.x -= movement_speed * scene.delta_time;
    }

    // Space
    if scene.input_manager.pressed_scancodes.contains(&57) {
        scene.main_camera.position.y += movement_speed * scene.delta_time;
        scene.main_camera.rotation.y += movement_speed * scene.delta_time;
    }
    // Shift
    if scene.input_manager.pressed_scancodes.contains(&42) {
        scene.main_camera.position.y -= movement_speed * scene.delta_time;
        scene.main_camera.rotation.y -= movement_speed * scene.delta_time;
    }
}

//https://learnopengl.com/Getting-started/Camera
