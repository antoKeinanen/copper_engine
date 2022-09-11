use std::f32::consts::PI;

use copper_engine::{
    blank_on_awake, blank_tick_update, engine, model_loading, structs::Scene,
    Camera, InputManager, Object, Material,
};
use model_loading::model_loader::get_obj;

fn main() {
    let input_manager = InputManager::new();
    let main_camera = Camera::new(
        0.1,
        1024.0,
        PI / 3.0,
        [0.0, 0.0, -10.0],
        [0.0, 0.0, 1.0],
        blank_tick_update,
        blank_on_awake,
    );

    let dragon = Object::new(
        "Dragon",
        get_obj("models/stanford_dragon_low.obj"),
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        Material{ ambient_color: [0.0, 0.2, 0.3]},
        dragon_tick_update,
        blank_on_awake,
    );

    let scene: Scene = Scene::new(
        vec![dragon],
        vec![],
        input_manager,
        main_camera,
    );

    engine(scene);
}

fn dragon_tick_update(scene: &mut Scene) {
    scene.game_objects[0].rotation[0] = scene.game_objects[0].rotation[0] + 100.0 * scene.delta_time;
    scene.game_objects[0].rotation[1] = scene.game_objects[0].rotation[1] + 100.0 * scene.delta_time;

    if scene.game_objects[0].rotation[0] > 2.0 * PI {
        scene.game_objects[0].rotation[0] = 0.0;
    }
    if scene.game_objects[0].rotation[1] > 2.0 * PI {
        scene.game_objects[0].rotation[1] = 0.0;
    }
}