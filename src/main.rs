use copper_engine::{
    blank_on_awake, blank_tick_update, engine, model_loading, Camera, InputManager, Object, Scene,
};
use model_loading::model_loader::get_obj;

fn main() {
    let input_manager = InputManager::new();
    let main_camera = Camera::new(
        0.1,
        1024.0,
        3.14159 / 3.0,
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
        [0.0, 0.0, 0.0],
        blank_tick_update,
        blank_on_awake,
    );
    let scene: Scene = Scene::new(vec![dragon], input_manager, main_camera);

    engine(scene);
}
