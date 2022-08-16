
use std::collections::HashSet;

use copper_engine::{engine, Scene, InputManager, Camera};
use glium::glutin;

fn camera_tick_update(scene: &mut Scene){

}

fn camera_awake(scene: &mut Scene){

}

fn main() {
    let mut input_manager = InputManager {
        pressed_scancodes: HashSet::new(),
        modifiers: glutin::event::ModifiersState::default(),
    };

    let mut main_camera = Camera{
        z_near: 0.1,
        z_far: 1024.0,
        fov: 3.141592 / 3.0,
        position: [0.0, 0.0, -10.0],
        rotation: [0.0, 0.0, 1.0],
        up_vector: [0.0, 1.0, 0.0],
        tick_update_func: camera_tick_update,
        on_awake: camera_awake
    };

    let mut scene = Scene{
        game_objects: vec![],
        input_manager: input_manager,
        main_camera: main_camera,
        delta_time: 0.0,
        time_since_start: 0.0,
    };

    engine(scene);
}