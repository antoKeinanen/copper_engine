use std::f32::consts::PI;

use copper_engine::{
    blank_on_awake, blank_tick_update, engine,
    input::InputManager,
    math::Vector3,
    object,
    object::{GameObject, Material},
    structs::Scene,
    Camera,
};
use object::model_loader::get_obj;

fn main() {
    let input_manager = InputManager::new();
    let main_camera = Camera::new(
        0.1,
        100.0,
        PI / 4.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 10.0),
        blank_tick_update,
        blank_on_awake,
    );

    let dragon = GameObject::new(
        "Dragon",
        get_obj("models/stanford_dragon_low.obj"),
        [0.0, 0.0, 10.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        Material {
            ambient_color: [0.0, 0.2, 0.3],
        },
        dragon_tick_update,
        blank_on_awake,
    );

    let suzane = GameObject::new(
        "Suzane",
        get_obj("models/suzane.obj"),
        [0.0, 0.0, 5.0],
        [0.0, PI, PI],
        [1.0, 1.0, 1.0],
        Material {
            ambient_color: [0.3, 0.2, 0.0],
        },
        suzane_tick_update,
        blank_on_awake,
    );

    let scene: Scene = Scene::new(vec![dragon, suzane], vec![], input_manager, main_camera);

    engine(scene);
}

fn dragon_tick_update(scene: &mut Scene) {
    scene.game_objects[0].rotation[0] =
        scene.game_objects[0].rotation[0] + 100.0 * scene.delta_time;
    scene.game_objects[0].rotation[1] =
        scene.game_objects[0].rotation[1] + 100.0 * scene.delta_time;

    if scene.game_objects[0].rotation[0] > 2.0 * PI {
        scene.game_objects[0].rotation[0] = 0.0;
    }
    if scene.game_objects[0].rotation[1] > 2.0 * PI {
        scene.game_objects[0].rotation[1] = 0.0;
    }
}

fn suzane_tick_update(scene: &mut Scene) {
    // scene.game_objects[1].translation[2] = - (scene.time_since_start.sin() + 10.0);
}
